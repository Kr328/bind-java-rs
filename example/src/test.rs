use crate::vm::with_java_vm;
use bind_java::{bind_java, call, Class, ClassLoader, Context, DefaultClassLoader, FromJava, IntoJava, Object};
use jni_sys::{jobject, jstring};
use proc_macro2::TokenStream;
use quote::quote;
use std::process::Stdio;

#[test]
pub fn test_create_vm() {
    with_java_vm(|_| {
        println!("CRATED");
    })
}

#[test]
pub fn test_convert_string() {
    with_java_vm(|ctx| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                const TEST_CONTENT_URL: &str = "https://www.cogsci.ed.ac.uk/~richard/unicode-sample-3-2.html";

                let response = reqwest::get(TEST_CONTENT_URL).await.unwrap();
                let content = response.text().await.unwrap();

                let o_string: jstring = content.clone().into_java(ctx).unwrap();
                let r_content: String = unsafe { String::from_java(o_string, ctx).unwrap() };

                assert_eq!(content, r_content);
            })
    })
}

#[test]
pub fn test_convert_string_array() {
    with_java_vm(|env| {
        let length = rand::random::<usize>() % 128;
        let array = (0..length)
            .map(|_| {
                let length = rand::random::<usize>() % 128;
                (0..length).map(|_| rand::random::<char>()).collect::<String>()
            })
            .collect::<Vec<_>>();

        let o_array = array.clone().into_java(env).unwrap();
        let r_array = unsafe { Vec::<String>::from_java(o_array, env).unwrap() };

        assert_eq!(array, r_array);
    })
}

#[test]
pub fn test_convert_bool_array() {
    with_java_vm(|env| {
        let length: usize = rand::random::<usize>() % 128;
        let array: Vec<bool> = (0..length).map(|_| rand::random::<bool>()).collect();

        let o_array = array.clone().into_java(env).unwrap();
        let r_array = unsafe { Vec::<bool>::from_java(o_array, env).unwrap() };

        assert_eq!(array, r_array);
    })
}

bind_java! {
    @ClassName("java.io.File")
    class JavaFile {
        JavaFile(java.lang.String path);

        java.net.URI toURI();
    }

    @ClassName("java.net.URI")
    class JavaURI {
        java.net.URL toURL();
    }

    @ClassName("java.net.URLClassLoader")
    class JavaUrlClassLoader {
        static java.net.URLClassLoader newInstance(java.net.URL[] urls);

        java.lang.Class loadClass(java.lang.String name);
    }
}

struct UrlClassLoader {
    _class_path: tempdir::TempDir,
    binding: JavaUrlClassLoader,
    object: Object,
}

impl ClassLoader for UrlClassLoader {
    fn load_class(&self, ctx: Context, name: &str) -> bind_java::Result<Class> {
        let name = name.replace("/", ".");

        unsafe { self.binding.load_class(ctx, self.object, name) }
    }
}

fn compile_file_and_load_classes(ctx: Context, public_class_name: &str, content: TokenStream) -> UrlClassLoader {
    let temp = tempdir::TempDir::new("classes").unwrap();
    let file_content = content.to_string().replace(" . ", ".").replace(" $ ", "$");
    let file = temp.path().join(public_class_name).with_extension("java");

    std::fs::write(&file, file_content).unwrap();

    let javac_ret = std::process::Command::new("javac")
        .arg("-J-Duser.language=en")
        .arg(file.file_name().unwrap().to_str().unwrap())
        .current_dir(temp.path())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if !javac_ret.success() {
        panic!("compile java failed");
    }

    unsafe {
        let c_file = JavaFile::find_class(ctx, &DefaultClassLoader::default()).unwrap();
        let b_file = JavaFile::bind(ctx, c_file).unwrap();
        let o_file = b_file.new(ctx, c_file, temp.path().to_str().unwrap()).unwrap();

        let c_uri = JavaURI::find_class(ctx, &DefaultClassLoader::default()).unwrap();
        let b_uri = JavaURI::bind(ctx, c_uri).unwrap();
        let o_uri: jobject = b_file.to_uri(ctx, o_file).unwrap();

        let c_url = call!(ctx, FindClass, "java/net/URL\0".as_ptr().cast());
        let o_url_array = call!(ctx, NewObjectArray, 1, c_url, b_uri.to_url(ctx, o_uri).unwrap());
        let c_url_class_loader = JavaUrlClassLoader::find_class(ctx, &DefaultClassLoader::default()).unwrap();
        let b_url_class_loader = JavaUrlClassLoader::bind(ctx, c_url_class_loader).unwrap();
        let o_url_class_loader: jobject = b_url_class_loader.new_instance(ctx, c_url_class_loader, o_url_array).unwrap();

        UrlClassLoader {
            _class_path: temp,
            binding: b_url_class_loader,
            object: o_url_class_loader,
        }
    }
}

#[test]
pub fn test_inner_class() {
    with_java_vm(|env| {
        let loader = compile_file_and_load_classes(
            env,
            "RustTest",
            quote! {
                public class RustTest {
                    static final InnerClass INNER = new InnerClass();

                    public static class InnerClass {
                        public final String VALUE = "STRING FROM INNER CLASS";
                    }
                }
            },
        );

        bind_java! {
            @ClassName("RustTest")
            class JavaRustTest {
                static final RustTest$InnerClass INNER;
            }

            @ClassName("RustTest$InnerClass")
            class JavaRustTestInnerClass {
                final java.lang.String VALUE;
            }
        }

        unsafe {
            let c_test = JavaRustTest::find_class(env, &loader).unwrap();
            let b_test = JavaRustTest::bind(env, c_test).unwrap();
            let o_inner = b_test.get_inner(env, c_test).unwrap();

            let c_inner = JavaRustTestInnerClass::find_class(env, &loader).unwrap();
            let b_inner = JavaRustTestInnerClass::bind(env, c_inner).unwrap();
            let value: String = b_inner.get_value(env, o_inner).unwrap();

            assert_eq!("STRING FROM INNER CLASS", value);
        }
    });
}

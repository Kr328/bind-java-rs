use crate::vm::with_java_vm;
use bind_java::{FromJava, IntoJava};
use jni_sys::jstring;

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

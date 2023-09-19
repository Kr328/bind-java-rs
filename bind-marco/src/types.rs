use std::fmt::{Display, Formatter};

use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::format_ident;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
    token::Bracket,
};

pub struct ClassName(pub Punctuated<Ident, Token![.]>);

impl Parse for ClassName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ClassName(Punctuated::parse_separated_nonempty(input)?))
    }
}

impl ClassName {
    pub fn to_class_name(&self) -> String {
        self.0.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(".")
    }
}

pub struct TypeName {
    class_name: ClassName,
    array_marks: Vec<Bracket>,
}

impl Parse for TypeName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TypeName {
            class_name: input.parse()?,
            array_marks: {
                let mut marks = Vec::<Bracket>::new();

                while let Some(_) = input.cursor().group(Delimiter::Bracket) {
                    let _content;
                    marks.push(bracketed!(_content in input));
                }

                marks
            },
        })
    }
}

impl TypeName {
    pub fn to_type(&self) -> Type {
        let class_name = self.class_name.0.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(".");
        let base_type = match class_name.as_str() {
            "void" => Type::Void,
            "boolean" => Type::Boolean,
            "byte" => Type::Byte,
            "char" => Type::Char,
            "short" => Type::Short,
            "int" => Type::Int,
            "long" => Type::Long,
            "float" => Type::Float,
            "double" => Type::Double,
            "java.lang.String" => Type::String,
            "java.lang.Class" => Type::Class,
            _ => Type::Object(class_name),
        };

        self.array_marks.iter().fold(base_type, |a, _| Type::Array(Box::new(a)))
    }
}

pub enum Type {
    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Class,
    Object(String),
    Array(Box<Type>),
}

impl Type {
    pub fn to_signature(&self) -> String {
        match self {
            Type::Void => "V".to_owned(),
            Type::Boolean => "Z".to_owned(),
            Type::Byte => "B".to_owned(),
            Type::Char => "C".to_owned(),
            Type::Short => "S".to_owned(),
            Type::Int => "I".to_owned(),
            Type::Long => "J".to_owned(),
            Type::Float => "F".to_owned(),
            Type::Double => "D".to_owned(),
            Type::String => "Ljava/lang/String;".to_owned(),
            Type::Class => "Ljava/lang/Class;".to_owned(),
            Type::Object(name) => {
                format!("L{};", name.replace(".", "/"))
            }
            Type::Array(inner) => {
                format!("[{}", inner.to_signature())
            }
        }
    }

    pub fn to_jni_type(&self) -> &'static str {
        match self {
            Type::Void => "()",
            Type::Boolean => "::jni_sys::jboolean",
            Type::Byte => "::jni_sys::jbyte",
            Type::Char => "::jni_sys::jchar",
            Type::Short => "::jni_sys::jshort",
            Type::Int => "::jni_sys::jint",
            Type::Long => "::jni_sys::jlong",
            Type::Float => "::jni_sys::jfloat",
            Type::Double => "::jni_sys::jdouble",
            Type::String => "::jni_sys::jstring",
            Type::Class => "::jni_sys::jclass",
            Type::Object(_) => "::jni_sys::jobject",
            Type::Array(inner) => match inner.as_ref() {
                Type::Boolean => "::jni_sys::jbooleanArray",
                Type::Byte => "::jni_sys::jbyteArray",
                Type::Char => "::jni_sys::jcharArray",
                Type::Short => "::jni_sys::jshortArray",
                Type::Int => "::jni_sys::jintArray",
                Type::Long => "::jni_sys::jlongArray",
                Type::Float => "::jni_sys::jfloatArray",
                Type::Double => "::jni_sys::jdoubleArray",
                _ => "::jni_sys::jobjectArray",
            },
        }
    }

    pub fn render_jni_type(&self) -> TokenStream {
        self.to_jni_type().parse().unwrap()
    }

    pub fn to_method_key(&self) -> &'static str {
        match self {
            Type::Void => "Void",
            Type::Boolean => "Boolean",
            Type::Byte => "Byte",
            Type::Char => "Char",
            Type::Short => "Short",
            Type::Int => "Int",
            Type::Long => "Long",
            Type::Float => "Float",
            Type::Double => "Double",
            _ => "Object",
        }
    }

    pub fn to_call_method_name(&self) -> Ident {
        format_ident!("Call{}Method", self.to_method_key())
    }

    pub fn to_call_static_method_name(&self) -> Ident {
        format_ident!("CallStatic{}Method", self.to_method_key())
    }

    pub fn to_get_field_method_name(&self) -> Ident {
        format_ident!("Get{}Field", self.to_method_key())
    }

    pub fn to_get_static_field_method_name(&self) -> Ident {
        format_ident!("GetStatic{}Field", self.to_method_key())
    }

    pub fn to_set_field_method_name(&self) -> Ident {
        format_ident!("Set{}Field", self.to_method_key())
    }

    pub fn to_set_static_field_method_name(&self) -> Ident {
        format_ident!("SetStatic{}Field", self.to_method_key())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => f.write_str("void"),
            Type::Boolean => f.write_str("boolean"),
            Type::Byte => f.write_str("byte"),
            Type::Char => f.write_str("char"),
            Type::Short => f.write_str("short"),
            Type::Int => f.write_str("int"),
            Type::Long => f.write_str("long"),
            Type::Float => f.write_str("float"),
            Type::Double => f.write_str("double"),
            Type::String => f.write_str("java.lang.String"),
            Type::Class => f.write_str("java.lang.Class"),
            Type::Object(name) => f.write_str(name),
            Type::Array(inner) => {
                Display::fmt(inner, f)?;

                f.write_str("[]")
            }
        }
    }
}

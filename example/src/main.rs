use jni_sys::{jlong, jobject, jstring};

use bind_java::{bind_java, call};

use crate::vm::with_java_vm;

mod vm;

#[cfg(test)]
mod test;

bind_java! {
    @ClassName("java.io.PrintStream")
    class JavaPrintStream {
        void println(java.lang.String value);
    }

    @ClassName("java.lang.System")
    class JavaSystem {
        static final java.io.PrintStream out;
    }

    @ClassName("java.nio.CharBuffer")
    class JavaCharBuffer {
        java.lang.String toString();
    }

    @ClassName("java.nio.charset.Charset")
    class JavaCharset {
        java.nio.CharBuffer decode(java.nio.ByteBuffer byteBuffer);
    }

    @ClassName("java.nio.charset.StandardCharsets")
    class JavaStandardCharsets {
        static final java.nio.charset.Charset UTF_8;
    }
}

fn main() {
    with_java_vm(|ctx| unsafe {
        let hello = "hello";
        let hello_bytes = hello.as_bytes();
        let hello_buffer = call!(
            v1_4,
            ctx,
            NewDirectByteBuffer,
            hello_bytes.as_ptr().cast_mut().cast(),
            hello_bytes.len() as jlong
        );

        let c_standard_charsets = JavaStandardCharsets::find_class(ctx, None).unwrap();
        let b_standard_charsets = JavaStandardCharsets::bind(ctx, c_standard_charsets).unwrap();
        let o_utf8: jobject = b_standard_charsets.get_utf_8(ctx, c_standard_charsets).unwrap();

        let c_charset = JavaCharset::find_class(ctx, None).unwrap();
        let b_charset = JavaCharset::bind(ctx, c_charset).unwrap();
        let o_char_buffer: jobject = b_charset.decode(ctx, o_utf8, hello_buffer).unwrap();

        let c_char_buffer = JavaCharBuffer::find_class(ctx, None).unwrap();
        let b_char_buffer = JavaCharBuffer::bind(ctx, c_char_buffer).unwrap();
        let o_hello: jstring = b_char_buffer.to_string(ctx, o_char_buffer).unwrap();

        let c_system = JavaSystem::find_class(ctx, None).unwrap();
        let b_system = JavaSystem::bind(ctx, c_system).unwrap();
        let o_out: jobject = b_system.get_out(ctx, c_system).unwrap();

        let c_print_stream = JavaPrintStream::find_class(ctx, None).unwrap();
        let b_print_stream = JavaPrintStream::bind(ctx, c_print_stream).unwrap();
        b_print_stream.println(ctx, o_out, o_hello).unwrap();
        b_print_stream.println(ctx, o_out, "world").unwrap();
        b_print_stream.println(ctx, o_out, "!").unwrap();
    });
}

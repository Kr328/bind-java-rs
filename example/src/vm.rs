use std::sync::OnceLock;

use jni::JavaVM;

use bind_java::Context;

pub fn with_java_vm<R, F: FnOnce(Context) -> R>(f: F) -> R {
    static VM: OnceLock<JavaVM> = OnceLock::new();
    let vm = VM.get_or_init(|| JavaVM::new(jni::InitArgsBuilder::new().build().unwrap()).unwrap());
    let env = vm.attach_current_thread().unwrap();

    f(env.get_raw())
}

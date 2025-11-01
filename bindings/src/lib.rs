use celestia_core::{send_command, Command};

type JNIEnv = *mut core::ffi::c_void;
type JClass = *mut core::ffi::c_void;
type JString = *mut core::ffi::c_void;
type jboolean = u8;

#[no_mangle]
pub extern "system" fn Java_dev_celestia_Core_ping(_env: JNIEnv, _class: JClass) -> jboolean {
    if send_command(Command::Ping) { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_dev_celestia_Core_echo(_env: JNIEnv, _class: JClass, _msg: JString) -> jboolean {
    if send_command(Command::Echo("from_jni".into())) { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_dev_celestia_Core_shutdown(_env: JNIEnv, _class: JClass) -> jboolean {
    if send_command(Command::Shutdown) { 1 } else { 0 }
}

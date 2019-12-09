#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use crate::generate_passphrase_ffi;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "system" fn Java_com_asimihsan_flutter_1app_GeneratePassphraseFfiKt_generatePassphrase(
    env: JNIEnv,
    _: JClass,
    input: JString,
) -> jstring {
    let java_str = env.get_string(input).expect("Couldn't get Java string!");
    let java_str_ptr = java_str.as_ptr();
    let result = generate_passphrase_ffi(java_str_ptr);

    // Crucially, note that this is the same as generate_passphrase_ffi_release, except we keep
    // the result_ptr around. By wrapping result into result_ptr, we are telling Rust "You now
    // own this memory. When this goes out of scope free the memory". Unlike the iOS case, for
    // JNI we are able to return a JString so we do so, and then after the method finishes we
    // Rust releases 'result'. This is why we don't need to call generate_passphrase_ffi_release.
    let result_ptr = CString::from_raw(result);

    let result_str = result_ptr.to_str().unwrap();
    let output = env
        .new_string(result_str)
        .expect("Couldn't create a Java string!");
    output.into_inner()
}

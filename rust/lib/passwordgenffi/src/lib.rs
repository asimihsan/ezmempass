use passwordgen::generate_passphrase;
use std::ffi::CString;
use std::os::raw::c_char;

// ----------------------------------------------------------------------------
//  Unsafe FFI
// ----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn generate_passphrase_ffi(_config: *const c_char) -> *mut c_char {
    let passphrase_length = 7;
    let passphrase_result = generate_passphrase(passphrase_length).unwrap();
    let mut result = String::from("Foo: ");
    result.push_str(&passphrase_result.passphrase[0]);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn generate_passphrase_ffi_release(result: *mut c_char) {
    if result.is_null() {
        return;
    }
    CString::from_raw(result);
}
// ----------------------------------------------------------------------------

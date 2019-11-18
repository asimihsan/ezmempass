use passwordgen::generate_passphrase;
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Serialize)]
struct GeneratePassphraseFfiResult {
    prefixes: Vec<String>,
    passphrase: Vec<String>,
}

#[derive(Deserialize)]
struct GeneratePassphraseFfiInput {
    passphrase_length: i32,
}

/// Generate a passphrase
///
/// # Safety
///
/// The result is a JSON-encoded string. The memory is still owned by the library. You must
/// call generate_passphrase_ffi_release() on the result when you're done.
#[no_mangle]
pub unsafe extern "C" fn generate_passphrase_ffi(input: *const c_char) -> *mut c_char {
    let input_deser: &str = CStr::from_ptr(input).to_str().unwrap();
    let input_deser: GeneratePassphraseFfiInput = serde_json::from_str(input_deser).unwrap();
    let passphrase_length = input_deser.passphrase_length;

    let passphrase_result = generate_passphrase(passphrase_length).unwrap();

    let result = GeneratePassphraseFfiResult {
        prefixes: passphrase_result.prefixes,
        passphrase: passphrase_result.passphrase,
    };
    let result = serde_json::to_string(&result).unwrap();
    CString::new(result).unwrap().into_raw()
}

/// Release the memory allocated for a generate passphrase result.
///
/// # Safety
///
/// Call on result of generate_passphrase_ffi().
#[no_mangle]
pub unsafe extern "C" fn generate_passphrase_ffi_release(result: *mut c_char) {
    if result.is_null() {
        return;
    }
    CString::from_raw(result);
}

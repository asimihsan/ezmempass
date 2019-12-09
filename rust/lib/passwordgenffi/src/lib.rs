use miniserde::{json, Deserialize, Serialize};
use passwordgen::{generate_passphrase, GeneratePassphraseInput};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[derive(Serialize)]
struct GeneratePassphraseFfiResult {
    password: String,
    passphrase: String,
}

#[derive(Deserialize)]
struct GeneratePassphraseFfiInput {
    passphrase_length: i32,
    add_capital_letter: bool,
    add_digit: bool,
    add_symbol: bool,
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
    let input_deser: GeneratePassphraseFfiInput = json::from_str(input_deser).unwrap();
    let input = GeneratePassphraseInput {
        passphrase_length: input_deser.passphrase_length,
        add_capital_letter: input_deser.add_capital_letter,
        add_digit: input_deser.add_digit,
        add_symbol: input_deser.add_symbol,
    };
    let passphrase_result = generate_passphrase(&input).unwrap();
    let result = GeneratePassphraseFfiResult {
        password: passphrase_result.password,
        passphrase: passphrase_result.passphrase,
    };
    let result = json::to_string(&result);
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

#[cfg(target_os = "android")]
mod android;

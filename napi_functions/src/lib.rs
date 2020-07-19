use std::ffi::CString;
use nodejs_sys::{
    napi_create_function,
    napi_env,
    napi_set_named_property,
    napi_value,
};

mod utils;
mod strings;

use strings::reverse_string;

#[no_mangle]
pub extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value
) -> nodejs_sys::napi_value {
    let func_name = "reverseString".to_string();
    let name_len = func_name.len();
    let c_name = CString::new(func_name).expect("CString::new failed");

    unsafe {
        let mut value: napi_value = std::mem::zeroed();

        napi_create_function(
            env,
            c_name.as_ptr(),
            name_len,
            Some(reverse_string),
            std::ptr::null_mut(),
            &mut value,
        );

        napi_set_named_property(env, exports, c_name.as_ptr(), value);
    }

    exports
}

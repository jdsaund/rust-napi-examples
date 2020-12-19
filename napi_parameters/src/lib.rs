use std::ffi::CString;
use nodejs_sys::{
    napi_callback,
    napi_create_function,
    napi_env,
    napi_set_named_property,
    napi_value,
};

#[macro_use]
mod utils;
mod native;

#[no_mangle]
pub extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value
) -> nodejs_sys::napi_value {
    let module : Vec<(String, napi_callback)>= vec![
        ("rotateArray".to_string(), Some(native::rotate_array)),
        ("rotateSharedBuffer".to_string(), Some(native::rotate_shared_buffer)),
    ];

    for (name, cb) in module {
        let name_len = name.len();
        let c_name = C_STRING!(name);

        unsafe {
            let mut value: napi_value = std::mem::zeroed();

            napi_create_function(
                env,
                c_name.as_ptr(),
                name_len,
                cb,
                std::ptr::null_mut(),
                &mut value,
            );

            napi_set_named_property(env, exports, c_name.as_ptr(), value);
        }
    }

    exports
}

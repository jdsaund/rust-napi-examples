use nodejs_sys::{
    napi_callback_info,
    napi_get_cb_info,
    napi_env,
    napi_status,
    napi_value,
};

// convenience wrapper to get single parameter
pub fn get_single_param(env: napi_env, info: napi_callback_info) -> napi_value {
    let mut buffer: [napi_value; 1] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let mut argc = 1 as u64;

    let status = unsafe { napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ) };

    assert_eq!(status, napi_status::napi_ok);

    buffer[0]
}

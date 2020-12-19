use nodejs_sys::{
    napi_callback_info,
    napi_get_undefined,
    napi_env,
    napi_status,
    napi_value,
};

use crate::utils;

pub extern "C" fn rotate_array(env: napi_env, info: napi_callback_info) -> napi_value {
    let (_this, params) = GET_PARAMS!(env, info, 1);

    // downcast
    let mut values: Vec<i64> = utils::downcast_array(env, params[0], utils::downcast_i64);

    // rotate
    values.rotate_right(1);

    // upcast
    let output: napi_value = unsafe { utils::upcast_array(env, &values, utils::upcast_i64) };

    output
}

pub extern "C" fn rotate_shared_buffer(env: napi_env, info: napi_callback_info) -> napi_value {
    let (_this, params) = GET_PARAMS!(env, info, 1);

    // dereference
    let mut values: Vec<u8> = utils::downcast_buffer(env, params[0]);

    // rotate
    values.rotate_right(1);

    // dont manage buffer as this is already owned by node
    std::mem::forget(values);

    // return undefined
    unsafe {
        let mut undefined: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_undefined(env, &mut undefined));
        undefined
    }
}

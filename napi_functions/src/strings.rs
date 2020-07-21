use std::ffi::CString;
use nodejs_sys::{
    napi_callback_info,
    napi_create_string_utf8,
    napi_get_value_string_utf8,
    napi_env,
    napi_status,
    napi_value,
};

use crate::utils::get_single_param;

pub extern "C" fn reverse_string(env: napi_env, info: napi_callback_info) -> napi_value {
    let napi_val: napi_value = get_single_param(env, info);
    let bufsize: usize = 0;
    let mut num_bytes_copied: u64 = 0;

    // query node to get the bufsize
    let length_status = unsafe { napi_get_value_string_utf8(
        env,
        napi_val,
        std::ptr::null_mut(),
        0,
        &mut (bufsize as u64)
    ) };

    // check that the length query went ok
    assert_eq!(length_status, napi_status::napi_ok);

    let mut buf: Vec<u8> = Vec::with_capacity(bufsize + 1);
    let buf_ptr = buf.as_mut_ptr();

    let copy_status = unsafe { napi_get_value_string_utf8(
        env,
        napi_val,
        buf_ptr as *mut i8,
        (bufsize + 1) as u64,
        &mut num_bytes_copied
    ) };

    // check that the copy went ok
    assert_eq!(bufsize as u64, num_bytes_copied);
    assert_eq!(copy_status, napi_status::napi_ok);

    // dont manage `buf` memory as we will this responsibility to `string`
    std::mem::forget(buf);

    let string: String = unsafe { String::from_raw_parts(buf_ptr, bufsize, bufsize) };
    let reversed: String = string.chars().rev().collect();

    let mut output: napi_value = unsafe { std::mem::zeroed() };
    let c_str = CString::new(reversed).expect("CString::new failed");
    let create_status = unsafe { napi_create_string_utf8(
        env,
        c_str.as_ptr(),
        bufsize as u64,
        &mut output
    ) };

    // check the creation went ok
    assert_eq!(create_status, napi_status::napi_ok);

    output
}

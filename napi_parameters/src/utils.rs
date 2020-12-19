use nodejs_sys::{
    napi_create_array,
    napi_create_int64,
    napi_get_array_length,
    napi_get_buffer_info,
    napi_get_element,
    napi_get_value_int64,
    napi_env,
    napi_set_element,
    napi_status,
    napi_value,
};

#[macro_export]
macro_rules! C_STRING {
    ($msg:expr) => {
        CString::new($msg.to_string()).expect("CString::new failed")
    };
}

#[macro_export]
macro_rules! NAPI_CALL {
    ($napi_call:expr) => {
      assert_eq!($napi_call, napi_status::napi_ok)
    };
}

#[macro_export]
macro_rules! GET_PARAMS {
    ($env:ident, $info:ident, $argc:literal) => {{
        let mut params: [napi_value; $argc] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        let mut this: napi_value = unsafe { std::mem::zeroed() };

        unsafe {
            NAPI_CALL!(nodejs_sys::napi_get_cb_info(
                $env,
                $info,
                &mut ($argc),
                params.as_mut_ptr(),
                &mut this,
                std::ptr::null_mut(),
            ));
        }

        (this, params)
    }}
}

pub fn downcast_i64(env: napi_env, value: napi_value) -> i64 {
    let mut result: i64 = 0;

    unsafe {
        NAPI_CALL!(napi_get_value_int64(
            env,
            value,
            &mut result
        ));
    };

    result
}

pub unsafe fn upcast_i64(env: napi_env, value: &i64) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();

    NAPI_CALL!(napi_create_int64(env, *value, &mut result));

    result
}

pub fn downcast_array<T>(env: napi_env, array: napi_value, downcast_element: fn(env: napi_env, element: napi_value) -> T) -> Vec<T> {
    let mut length: u32 = 0;

    unsafe {
        NAPI_CALL!(napi_get_array_length(
            env,
            array,
            &mut length
        ));
    }

    let mut result: Vec<T> = Vec::with_capacity(length as usize);

    for i in 0..length {
        let mut element: napi_value = unsafe{ std::mem::zeroed() };
        unsafe {
            NAPI_CALL!(napi_get_element(
                env,
                array,
                i,
                &mut element
            ));
        }

        result.push(downcast_element(env, element))
    }

    result
}

pub unsafe fn upcast_array<T>(env: napi_env, array: &Vec<T>, upcast_element: unsafe fn(env: napi_env, element: &T) -> napi_value) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();

    NAPI_CALL!(napi_create_array(env, &mut result));

    for (j, element) in array.iter().enumerate() {
        let upcast_element: napi_value = upcast_element(env, element);
        NAPI_CALL!(napi_set_element(env, result, j as u32, upcast_element));
    }

    result
}

pub fn downcast_buffer(env: napi_env, value: napi_value) -> Vec<u8> {
    let mut buf_ptr: *mut u8 = std::ptr::null_mut();
    let buf_ptr_ptr: *mut *mut u8 = &mut buf_ptr;
    let mut bufsize: usize = 0;

    unsafe {
        NAPI_CALL!(napi_get_buffer_info(
            env,
            value,
            buf_ptr_ptr as *mut *mut std::ffi::c_void,
            &mut bufsize
        ));
    }

    let result: Vec<u8> = unsafe {
        Vec::from_raw_parts(buf_ptr, bufsize as usize, bufsize as usize)
    };

    result
}

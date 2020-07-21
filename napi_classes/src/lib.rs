use std::ffi::CString;
use nodejs_sys::{
    napi_callback_info,
    napi_create_double,
    napi_define_class,
    napi_env,
    napi_get_cb_info,
    napi_get_undefined,
    napi_get_value_double,
    napi_property_attributes,
    napi_property_descriptor,
    napi_set_named_property,
    napi_status,
    napi_unwrap,
    napi_value,
    napi_wrap
};

macro_rules! NAPI_CALL {
    ($napi_call:expr) => {
        assert_eq!($napi_call, napi_status::napi_ok)
    };
}

#[derive(Debug)]
struct MyObject {
  value: f64,
}

impl MyObject {
    pub fn new () -> Self {
        MyObject {
            value: 0.0
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn get(
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value {
        // attach the context to 'this'
        let mut this: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_cb_info(
            env,
            info,
            &mut 0,
            std::ptr::null_mut(),
            &mut this,
            std::ptr::null_mut(),
        ));

        // unwrap
        let mut box_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        NAPI_CALL!(napi_unwrap(env, this, &mut box_ptr));
        let native_obj = Box::from_raw(box_ptr as *mut MyObject);

        // get the native value
        let mut value: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_create_double(env, native_obj.value, &mut value));

        // dont free memory that node is still using
        std::mem::forget(native_obj);

        value
    }

    #[no_mangle]
    pub unsafe extern "C" fn set(
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value {
        // get the argument
        let mut argv: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
        let mut argc: u64 = 1;
        let mut this: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_cb_info(
            env,
            info,
            &mut argc,
            argv.as_mut_ptr(),
            &mut this,
            std::ptr::null_mut(),
        ));

        // unwrap
        let mut box_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        NAPI_CALL!(napi_unwrap(env, this, &mut box_ptr));
        let mut native_obj = Box::from_raw(box_ptr as *mut MyObject);

        // get the input value and assign it to the object
        NAPI_CALL!(napi_get_value_double(env, argv[0], &mut native_obj.value));

        // dont free memory that node is still using
        std::mem::forget(native_obj);

        // return undefined
        let mut undefined: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_undefined(env, &mut undefined));
        undefined
    }

    #[no_mangle]
    pub unsafe extern "C" fn ping(
        env: napi_env,
        _info: napi_callback_info
    ) -> napi_value {
        // reply from rust
        println!("pong");

        // return undefined
        let mut undefined: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_undefined(env, &mut undefined));
        undefined
    }

    #[no_mangle]
    pub extern "C" fn destructor(
        _env: napi_env,
        finalize_data: *mut std::ffi::c_void,
        _finalize_hint: *mut std::ffi::c_void
    ) -> () {
        // dont try to double free
        if finalize_data.is_null() {
            return;
        }

        // free the memory
        let this: Box<MyObject> = unsafe { Box::from_raw(finalize_data as *mut MyObject) };
        std::mem::drop(this);
    }

    #[no_mangle]
    pub unsafe extern "C" fn constructor(
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value {
        // attach the context to 'this'
        let mut this: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_cb_info(
            env,
            info,
            &mut 0, // argc - not used
            std::ptr::null_mut(), // argv - not used
            &mut this,
            std::ptr::null_mut(),
        ));

        // create the instance and put it on the heap
        let native_object: *mut MyObject = Box::into_raw(Box::new(MyObject::new()));
        NAPI_CALL!(napi_wrap(
            env,
            this,
            native_object as *mut std::ffi::c_void,
            Some(MyObject::destructor),
            std::ptr::null_mut(),
            std::ptr::null_mut()
        ));

        // tell rust to not manage the native_object memory
        std::mem::forget(native_object);

        this
    }
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value
) -> nodejs_sys::napi_value {
    // define the properties
    let instance_value_name = CString::new("value".to_string()).expect("CString::new failed");
    let instance_method_name = CString::new("ping".to_string()).expect("CString::new failed");
    let properties: [napi_property_descriptor; 2] = [
        // describes an instance value with getter and setter
        napi_property_descriptor {
            utf8name: instance_value_name.as_ptr() as * const std::os::raw::c_char,
            name: std::ptr::null_mut(),
            method: None,
            getter: Some(MyObject::get),
            setter: Some(MyObject::set),
            value: std::ptr::null_mut(),
            attributes: napi_property_attributes::napi_default,
            data: std::ptr::null_mut()
        },
        // describes an instance method
        napi_property_descriptor {
            utf8name: instance_method_name.as_ptr() as * const std::os::raw::c_char,
            name: std::ptr::null_mut(),
            method: Some(MyObject::ping),
            getter: None,
            setter: None,
            value: std::ptr::null_mut(),
            attributes: napi_property_attributes::napi_default,
            data: std::ptr::null_mut()
        }
    ];

    // define the class
    let class_name = "MyObject".to_string();
    let name_len = class_name.len() as u64;
    let c_name = CString::new(class_name).expect("CString::new failed");
    let mut cons: napi_value = std::mem::zeroed();
    NAPI_CALL!(napi_define_class(
        env,
        c_name.as_ptr(),
        name_len,
        Some(MyObject::constructor),
        std::ptr::null_mut(),
        properties.len() as u64,
        properties.as_ptr(),
        &mut cons
    ));

    // attach the constructor to the module
    NAPI_CALL!(napi_set_named_property(env, exports, c_name.as_ptr(), cons));

    exports
}

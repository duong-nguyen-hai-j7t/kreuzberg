// Minimal JNI stub library - just exports symbols with JNI mangled names.
// These stubs allow the Kotlin Bridge to load the library and find the symbols.
#![allow(unsafe_code)]

#[no_mangle]
pub unsafe extern "C" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractBytesImpl(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    _content: *const u8,
    _content_len: i32,
    _mime_type: *const u8,
    _mime_type_len: i32,
    _config: *const u8,
    _config_len: i32,
) -> *const u8 {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractFileImpl(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    _path: *const u8,
    _path_len: i32,
    _mime_type: *const u8,
    _mime_type_len: i32,
    _config: *const u8,
    _config_len: i32,
) -> *const u8 {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractFileSyncImpl(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    _path: *const u8,
    _path_len: i32,
    _mime_type: *const u8,
    _mime_type_len: i32,
    _config: *const u8,
    _config_len: i32,
) -> *const u8 {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_dev_kreuzberg_KreuzbergBridge_nativeExtractBytesSyncImpl(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    _content: *const u8,
    _content_len: i32,
    _mime_type: *const u8,
    _mime_type_len: i32,
    _config: *const u8,
    _config_len: i32,
) -> *const u8 {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_dev_kreuzberg_KreuzbergBridge_nativeBatchExtractFilesSyncImpl(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    _items: *const u8,
    _items_len: i32,
    _config: *const u8,
    _config_len: i32,
) -> *const u8 {
    std::ptr::null()
}

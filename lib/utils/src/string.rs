#[macro_export]
macro_rules! str_to_c_str_ptr {
    ($uniform_name:expr) => {
        std::ffi::CString::new($uniform_name.as_bytes())
            .unwrap()
            .as_c_str()
            .as_ptr()
    };
}

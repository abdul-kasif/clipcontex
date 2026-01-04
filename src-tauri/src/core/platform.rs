#[cfg(target_os = "linux")]
pub use malloc_trim_support::malloc_trim_now;

#[cfg(target_os = "linux")]
mod malloc_trim_support {
    use std::ffi::c_int;
    extern "C" {
        pub fn malloc_trim(__pad: c_int) -> c_int;
    }
    #[inline]
    pub fn malloc_trim_now() {
        unsafe {
            malloc_trim(0);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn malloc_trim_now() {
    //nothing to do for windows
}

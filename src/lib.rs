use std::ffi::CStr;

mod log;
mod mem;
pub use log::*;
pub use mem::*;

#[allow(improper_ctypes)]
#[link(wasm_import_module = "env")]
extern "C" {
    pub fn hc_log(level: i32, file: &str, line: u32, function: &str, code: i32, msg: &str);
    fn hc_tls_get_cert_serial() -> *mut u8;
    fn hc_tls_get_sni() -> *mut u8;
// fn SCFlowAppLayerProto() -> *mut u8;
// pub fn SCLogInfo(s: &str);
// fn SCPacketTimestamp(sec: &mut u64, usec: &mut u64);
}

pub fn tls_get_cert_serial() -> Option<String> {
    unsafe {
        let ptr = hc_tls_get_cert_serial();
        if ptr.is_null() {
            return None;
        }
        let s = CStr::from_ptr(ptr as *const i8);
        sc_free(ptr);
        s.to_str().ok().map(|s| s.to_owned())
    }
}

pub fn tls_get_sni() -> Option<String> {
    unsafe {
        let ptr = hc_tls_get_sni();
        if ptr.is_null() {
            return None;
        }
        let s = CStr::from_ptr(ptr as *const i8);
        sc_free(ptr);
        s.to_str().ok().map(|s| s.to_owned())
    }
}

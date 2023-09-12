use std::env;
use std::ffi::CStr;
use std::str;

#[link(name = "vomid")]
extern {
    fn vmd_sleep(time: f64);
    fn vmd_enum_devices(device_type: isize,
                        clb: extern fn(*const i8, *const i8, *const i8),
                        arg: *const i8);
}

fn sleep(time: f64) {
    unsafe {
        vmd_sleep(time);
    }
}

fn from_utf8(utf: *const i8) -> String {
    let cstr = unsafe { CStr::from_ptr(utf) };
    return match str::from_utf8(cstr.to_bytes()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    }.to_owned();
}

extern "C" fn output_callback(id: *const i8, name: *const i8, _arg: *const i8) {
    let id = from_utf8(id);
    let name = from_utf8(name);
    println!("Device '{}': {}", id, name);
}

fn main() {
    const ARG : [i8; 1] = [0];
    unsafe {
        vmd_enum_devices(1, output_callback, ARG.as_ptr());
    }
    sleep(0.1);
}

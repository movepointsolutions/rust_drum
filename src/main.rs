use std::env;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;

#[link(name = "vomid")]
extern {
    fn vmd_sleep(time: f64);
    fn vmd_enum_devices(device_type: isize,
                        clb: extern fn(*const i8, *const i8, *const i8),
                        arg: *const i8);
    fn vmd_set_device(device_type: isize, id: *const i8);
    fn vmd_output(event: *const u8);
    fn vmd_flush_output();
}

const VMD_OUTPUT_DEVICE : isize = 1;
const VMD_DRUM_CHANNEL : u8 = 9;
//const VMD_VOICE_NOTEOFF : u8 = 0x80;
const VMD_VOICE_NOTEON  : u8 = 0x90;

fn beat() {
    let event = [VMD_VOICE_NOTEON + VMD_DRUM_CHANNEL, 35, 100];
    unsafe {
        vmd_output(event.as_ptr());
        vmd_flush_output();
    }
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

fn enum_devices() {
    const ARG : [i8; 1] = [0];
    unsafe {
        vmd_enum_devices(VMD_OUTPUT_DEVICE, output_callback, ARG.as_ptr());
    }
}

fn set_device(arg: String) {
    let arg = CString::new(arg).unwrap();
    unsafe {
        vmd_set_device(VMD_OUTPUT_DEVICE, arg.as_ptr());
    }
}

fn main() {
    if env::args().count() < 2 {
        panic!("Usage: rust_drum [-L|device]");
    }
    let arg = env::args().nth(1).expect("no arg?");
    if arg == "-L" {
        enum_devices();
    } else {
        set_device(arg);
        for _i in 0..100 {
            beat();
            sleep(1.0);
        }
    }
}

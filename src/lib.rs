use chiter::make_entrypoint;

use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::thread;
use winapi::um::winuser::SetWindowsHookExA;
use winapi::um::winuser::{MessageBoxW, MB_HELP};
use volatile::Volatile;

extern crate libc;
extern crate winapi;

//pub fn print_message(msg: &str, caption: &str) -> Result<i32, Error> {
//    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
//    let msg_caption: Vec<u16> = OsStr::new(caption).encode_wide().chain(once(0)).collect();
//
//    let ret = unsafe { MessageBoxW(null_mut(), wide.as_ptr(), msg_caption.as_ptr(), MB_HELP) };
//
//    print!("{}", ret);
//
//    if ret == 0 {
//        Err(Error::last_os_error())
//    } else {
//        Ok(ret)
//    }
//}

struct Player<'a> {
    ammo: &'a u32,
}

pub fn refresh_ammo(ammo_ptr: &u32) {
    let new_player =
    //let *ammo = new_player.ammo;

    println!("{:?}", new_player.ammo);

    //*new_player.ammo = 32;
    //new_player.ammo = 64;

    println!("{:?}", new_player.ammo);
}
fn entry_point() {

    unsafe {
        winapi::um::consoleapi::AllocConsole();
    }
    println!("Injected!");

    let entity_list = 0xFEE00000 as *mut u32;
    let entity_list_ptr = entity_list as *mut Volatile<u32>;

}

make_entrypoint!(entry_point);

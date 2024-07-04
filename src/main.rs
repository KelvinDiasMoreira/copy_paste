use std::ffi::{c_void, CStr};
use windows::core::Result;

use windows::Win32::Foundation::{HGLOBAL, HWND};
use windows::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

fn main() -> Result<()> {
    loop {
        unsafe {
            let control_is_press = GetAsyncKeyState(17);
            let p_is_press = GetAsyncKeyState(80);

            if control_is_press != 0 && p_is_press != 0 {
                if OpenClipboard(HWND(0)).is_ok() {
                    let handle = GetClipboardData(1)?;
                    if !handle.is_invalid() {
                        let hglobal: HGLOBAL = HGLOBAL(handle.0 as *mut c_void);
                        let ptr = GlobalLock(hglobal);
                        if !ptr.is_null() {
                            let string = CStr::from_ptr(ptr as *const i8);
                            println!("{:?}", string);
                            let _ = GlobalUnlock(hglobal);
                        } else {
                            println!("Failed to lock Memory");
                        }
                    } else {
                        println!("handle is invalid");
                    }
                } else {
                    println!("Failed to open clipboard");
                }
                let _ = CloseClipboard();
            }
        }
    }
}

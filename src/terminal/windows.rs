use std::io::Error;
use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};

/// Gets the width of the current terminal.
/// # Returns
/// Returns a Option<i16>.
pub fn get_terminal_width() -> Result<i16, Error> {
    unsafe {
        let mut console_info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
        let handle = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);

        if GetConsoleScreenBufferInfo(handle, &mut console_info) != 0 {
            Ok(console_info.dwSize.X)
        } else {
            Err(Error::last_os_error())
        }
    }
}

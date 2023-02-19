use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};

/// Gets the width of the current terminal.
/// # Returns
/// Returns a Option<u16>.
pub fn get_terminal_width() -> Option<u16> {
    unsafe {
        let mut console_info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
        let handle = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);

        if GetConsoleScreenBufferInfo(handle, &mut console_info) != 0 {
            Some(console_info.dwSize.X as u16)
        } else {
            None
        }
    }
}

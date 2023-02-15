use libc::{ioctl, winsize, TIOCGWINSZ};
use std::os::unix::io::AsRawFd;

/// Gets the width of the current terminal.
/// # Returns
/// Returns a Option<u16>.
pub fn get_terminal_width() -> Option<u16> {
    let stdout_fd: i32 = std::io::stdout().as_raw_fd();
    let mut size: winsize = unsafe { std::mem::zeroed() };

    if unsafe { ioctl(stdout_fd, TIOCGWINSZ, &mut size) } != -1 {
        let width: u16 = size.ws_col;
        return Some(width);
    }

    return None;
}

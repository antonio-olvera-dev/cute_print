#[cfg(test)]
mod unix_test {
    use crate::get_terminal_width;

    #[test]
    fn get_terminal_width_test() {
        let width: u16 = match get_terminal_width() {
            Some(width) => width,
            None => 0,
        };
        assert_ne!(width, 0);
    }
}

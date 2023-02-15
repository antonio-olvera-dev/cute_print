#[cfg(test)]
mod unix_test {
    use crate::get_terminal_width;

    #[test]
    fn get_terminal_width_test() {
        match get_terminal_width() {
            Some(width) => {
                assert_ne!(width, 0);
                width
            }
            _ => 0,
        };
    }
}

#[cfg(test)]
mod example_colors_test {
    use crate::example_colors::print_colors;

    #[test]
    fn test_print_colors() {
        let print_colors: () = print_colors();
        assert_eq!(print_colors, ());
    }
}

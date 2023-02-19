#[cfg(test)]
mod style_list_test {
    use crate::StyleList;

    #[test]
    fn style_list_test() {
        let style_list: StyleList = StyleList::new();

        assert_eq!(style_list.bold, "\x1b[1m");
        assert_eq!(style_list.dim, "\x1b[2m");
        assert_eq!(style_list.underline, "\x1b[4m");
        assert_eq!(style_list.blink, "\x1b[5m");
        assert_eq!(style_list.reverse, "\x1b[7m");
        assert_eq!(style_list.hidden, "\x1b[8m");
    }
}

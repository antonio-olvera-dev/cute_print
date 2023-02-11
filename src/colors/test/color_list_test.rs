#[cfg(test)]
mod color_list_test {
    use crate::ColorList;

    #[test]
    fn test_print_colors() {
        let color_list: ColorList = ColorList::new();

        assert_eq!(color_list.reset, "\x1b[0m");
        assert_eq!(color_list.black_fg, "\x1b[30m");
        assert_eq!(color_list.red_fg, "\x1b[31m");
        assert_eq!(color_list.green_fg, "\x1b[32m");
        assert_eq!(color_list.yellow_fg, "\x1b[33m");
        assert_eq!(color_list.blue_fg, "\x1b[34m");
        assert_eq!(color_list.magenta_fg, "\x1b[35m");
        assert_eq!(color_list.cyan_fg, "\x1b[36m");
        assert_eq!(color_list.white_fg, "\x1b[37m");
        assert_eq!(color_list.bright_black_fg, "\x1b[90m");
        assert_eq!(color_list.bright_red_fg, "\x1b[91m");
        assert_eq!(color_list.bright_green_fg, "\x1b[92m");
        assert_eq!(color_list.bright_yellow_fg, "\x1b[93m");
        assert_eq!(color_list.bright_blue_fg, "\x1b[94m");
        assert_eq!(color_list.bright_magenta_fg, "\x1b[95m");
        assert_eq!(color_list.bright_cyan_fg, "\x1b[96m");
        assert_eq!(color_list.bright_white_fg, "\x1b[97m");
        assert_eq!(color_list.black_bg, "\x1b[40m");
        assert_eq!(color_list.red_bg, "\x1b[41m");
        assert_eq!(color_list.green_bg, "\x1b[42m");
        assert_eq!(color_list.yellow_bg, "\x1b[43m");
        assert_eq!(color_list.blue_bg, "\x1b[44m");
        assert_eq!(color_list.magenta_bg, "\x1b[45m");
        assert_eq!(color_list.cyan_bg, "\x1b[46m");
        assert_eq!(color_list.white_bg, "\x1b[47m");
        assert_eq!(color_list.bright_black_bg, "\x1b[100m");
        assert_eq!(color_list.bright_red_bg, "\x1b[101m");
        assert_eq!(color_list.bright_green_bg, "\x1b[102m");
        assert_eq!(color_list.bright_yellow_bg, "\x1b[103m");
        assert_eq!(color_list.bright_blue_bg, "\x1b[104m");
        assert_eq!(color_list.bright_magenta_bg, "\x1b[105m");
        assert_eq!(color_list.bright_cyan_bg, "\x1b[106m");
        assert_eq!(color_list.bright_white_bg, "\x1b[107m");
    }
}

pub struct ColorList {
    pub reset: &'static str,
    pub black_fg: &'static str,
    pub red_fg: &'static str,
    pub green_fg: &'static str,
    pub yellow_fg: &'static str,
    pub blue_fg: &'static str,
    pub magenta_fg: &'static str,
    pub cyan_fg: &'static str,
    pub white_fg: &'static str,
    pub bright_black_fg: &'static str,
    pub bright_red_fg: &'static str,
    pub bright_green_fg: &'static str,
    pub bright_yellow_fg: &'static str,
    pub bright_blue_fg: &'static str,
    pub bright_magenta_fg: &'static str,
    pub bright_cyan_fg: &'static str,
    pub bright_white_fg: &'static str,
    pub black_bg: &'static str,
    pub red_bg: &'static str,
    pub green_bg: &'static str,
    pub yellow_bg: &'static str,
    pub blue_bg: &'static str,
    pub magenta_bg: &'static str,
    pub cyan_bg: &'static str,
    pub white_bg: &'static str,
    pub bright_black_bg: &'static str,
    pub bright_red_bg: &'static str,
    pub bright_green_bg: &'static str,
    pub bright_yellow_bg: &'static str,
    pub bright_blue_bg: &'static str,
    pub bright_magenta_bg: &'static str,
    pub bright_cyan_bg: &'static str,
    pub bright_white_bg: &'static str,
}
impl ColorList {
    pub fn new() -> Self {
        ColorList {
            reset: "\x1b[0m",
            black_fg: "\x1b[30m",
            red_fg: "\x1b[31m",
            green_fg: "\x1b[32m",
            yellow_fg: "\x1b[33m",
            blue_fg: "\x1b[34m",
            magenta_fg: "\x1b[35m",
            cyan_fg: "\x1b[36m",
            white_fg: "\x1b[37m",
            bright_black_fg: "\x1b[90m",
            bright_red_fg: "\x1b[91m",
            bright_green_fg: "\x1b[92m",
            bright_yellow_fg: "\x1b[93m",
            bright_blue_fg: "\x1b[94m",
            bright_magenta_fg: "\x1b[95m",
            bright_cyan_fg: "\x1b[96m",
            bright_white_fg: "\x1b[97m",
            black_bg: "\x1b[40m",
            red_bg: "\x1b[41m",
            green_bg: "\x1b[42m",
            yellow_bg: "\x1b[43m",
            blue_bg: "\x1b[44m",
            magenta_bg: "\x1b[45m",
            cyan_bg: "\x1b[46m",
            white_bg: "\x1b[47m",
            bright_black_bg: "\x1b[100m",
            bright_red_bg: "\x1b[101m",
            bright_green_bg: "\x1b[102m",
            bright_yellow_bg: "\x1b[103m",
            bright_blue_bg: "\x1b[104m",
            bright_magenta_bg: "\x1b[105m",
            bright_cyan_bg: "\x1b[106m",
            bright_white_bg: "\x1b[107m",
        }
    }
}

#[cfg(test)]
mod tests {
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

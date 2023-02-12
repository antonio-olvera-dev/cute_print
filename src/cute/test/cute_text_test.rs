#[cfg(test)]
mod cute_text_test {
    use crate::{ColorList, CuteText};
    use std::fmt::Debug;

    use std::fmt;

    impl Debug for ColorList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ColorList")
                .field("reset", &self.reset)
                .field("black_fg", &self.black_fg)
                .field("red_fg", &self.red_fg)
                .field("green_fg", &self.green_fg)
                .field("yellow_fg", &self.yellow_fg)
                .field("blue_fg", &self.blue_fg)
                .field("magenta_fg", &self.magenta_fg)
                .field("cyan_fg", &self.cyan_fg)
                .field("white_fg", &self.white_fg)
                .field("bright_black_fg", &self.bright_black_fg)
                .field("bright_red_fg", &self.bright_red_fg)
                .field("bright_green_fg", &self.bright_green_fg)
                .field("bright_yellow_fg", &self.bright_yellow_fg)
                .field("bright_blue_fg", &self.bright_blue_fg)
                .field("bright_magenta_fg", &self.bright_magenta_fg)
                .field("bright_cyan_fg", &self.bright_cyan_fg)
                .field("bright_white_fg", &self.bright_white_fg)
                .field("black_bg", &self.black_bg)
                .field("red_bg", &self.red_bg)
                .field("green_bg", &self.green_bg)
                .field("yellow_bg", &self.yellow_bg)
                .field("blue_bg", &self.blue_bg)
                .field("magenta_bg", &self.magenta_bg)
                .field("cyan_bg", &self.cyan_bg)
                .field("white_bg", &self.white_bg)
                .field("bright_black_bg", &self.bright_black_bg)
                .field("bright_red_bg", &self.bright_red_bg)
                .field("bright_green_bg", &self.bright_green_bg)
                .field("bright_yellow_bg", &self.bright_yellow_bg)
                .field("bright_blue_bg", &self.bright_blue_bg)
                .field("bright_magenta_bg", &self.bright_magenta_bg)
                .field("bright_cyan_bg", &self.bright_cyan_bg)
                .field("bright_white_bg", &self.bright_white_bg)
                .finish()
        }
    }

    impl Debug for CuteText {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CuteText")
                .field("text", &self.text)
                .field("color_list", &self.color_list)
                .finish()
        }
    }

    impl PartialEq for CuteText {
        fn eq(&self, other: &Self) -> bool {
            self.text == other.text && self.color_list.red_bg == other.color_list.red_bg
        }
    }

    #[test]
    fn test_cute_text() {
        let cute_text: CuteText = CuteText {
            text: String::new(),
            color_list: ColorList::new(),
            prev_len: 0
        };

        let cute_text_new: CuteText = CuteText::new();
        assert_eq!(cute_text, cute_text_new);
    }
}

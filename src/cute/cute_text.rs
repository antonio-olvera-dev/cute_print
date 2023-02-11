use super::super::ColorList;

pub struct CuteText {
    pub text: String,
    pub color_list: ColorList,
}

impl CuteText {
    pub fn new() -> Self {
        CuteText {
            text: String::new(),
            color_list: ColorList::new(),
        }
    }

    pub fn black(&mut self) -> &mut Self {
        self.text = self.color_list.black_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn red(&mut self) -> &mut Self {
        self.text = self.color_list.red_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn green(&mut self) -> &mut Self {
        self.text = self.color_list.green_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn yellow(&mut self) -> &mut Self {
        self.text = self.color_list.yellow_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn blue(&mut self) -> &mut Self {
        self.text = self.color_list.blue_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn magenta(&mut self) -> &mut Self {
        self.text = self.color_list.magenta_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn cyan(&mut self) -> &mut Self {
        self.text = self.color_list.cyan_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn white(&mut self) -> &mut Self {
        self.text = self.color_list.white_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_black(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_black_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_red(&mut self) -> &mut Self {
        self.text = self.color_list.bright_red_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_green(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_green_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_yellow(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_yellow_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_blue(&mut self) -> &mut Self {
        self.text = self.color_list.bright_blue_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_magenta(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_magenta_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_cyan(&mut self) -> &mut Self {
        self.text = self.color_list.bright_cyan_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn bright_white(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_white_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    pub fn on_black(&mut self) -> &mut Self {
        self.text = self.color_list.black_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_red(&mut self) -> &mut Self {
        self.text = self.color_list.red_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_green(&mut self) -> &mut Self {
        self.text = self.color_list.green_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_yellow(&mut self) -> &mut Self {
        self.text = self.color_list.yellow_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_blue(&mut self) -> &mut Self {
        self.text = self.color_list.blue_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_magenta(&mut self) -> &mut Self {
        self.text = self.color_list.magenta_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_cyan(&mut self) -> &mut Self {
        self.text = self.color_list.cyan_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_white(&mut self) -> &mut Self {
        self.text = self.color_list.white_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_black(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_black_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_red(&mut self) -> &mut Self {
        self.text = self.color_list.bright_red_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_green(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_green_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_yellow(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_yellow_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_blue(&mut self) -> &mut Self {
        self.text = self.color_list.bright_blue_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_magenta(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_magenta_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_cyan(&mut self) -> &mut Self {
        self.text = self.color_list.bright_cyan_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
    pub fn on_bright_white(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_white_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use super::CuteText;
    use crate::ColorList;

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
        };

        let cute_text_new: CuteText = CuteText::new();
        assert_eq!(cute_text, cute_text_new);
    }
}

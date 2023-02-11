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

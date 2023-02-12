use super::super::ColorList;

/// The `CuteText` struct contains a string of text and a reference to a `ColorList`.
pub struct CuteText {
    pub text: String,
    pub color_list: ColorList,
}

impl CuteText {
    /// This function creates a new instance of the `CuteText` struct.
    pub fn new() -> Self {
        CuteText {
            text: String::new(),
            color_list: ColorList::new(),
        }
    }

    /// This function sets the text color to black.
    pub fn black(&mut self) -> &mut Self {
        self.text = self.color_list.black_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to red.
    pub fn red(&mut self) -> &mut Self {
        self.text = self.color_list.red_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to green.
    pub fn green(&mut self) -> &mut Self {
        self.text = self.color_list.green_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to yellow.
    pub fn yellow(&mut self) -> &mut Self {
        self.text = self.color_list.yellow_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to blue.
    pub fn blue(&mut self) -> &mut Self {
        self.text = self.color_list.blue_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to magenta.
    pub fn magenta(&mut self) -> &mut Self {
        self.text = self.color_list.magenta_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to cyan.
    pub fn cyan(&mut self) -> &mut Self {
        self.text = self.color_list.cyan_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to white.
    pub fn white(&mut self) -> &mut Self {
        self.text = self.color_list.white_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright black.
    pub fn bright_black(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_black_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright red.
    pub fn bright_red(&mut self) -> &mut Self {
        self.text = self.color_list.bright_red_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright green.
    pub fn bright_green(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_green_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright yellow.
    pub fn bright_yellow(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_yellow_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright blue.
    pub fn bright_blue(&mut self) -> &mut Self {
        self.text = self.color_list.bright_blue_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright magenta.
    pub fn bright_magenta(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_magenta_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright cyan.
    pub fn bright_cyan(&mut self) -> &mut Self {
        self.text = self.color_list.bright_cyan_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the text color to bright white.
    pub fn bright_white(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_white_fg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to on_black.
    pub fn on_black(&mut self) -> &mut Self {
        self.text = self.color_list.black_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to red.
    pub fn on_red(&mut self) -> &mut Self {
        self.text = self.color_list.red_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to green.
    pub fn on_green(&mut self) -> &mut Self {
        self.text = self.color_list.green_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to yellow.
    pub fn on_yellow(&mut self) -> &mut Self {
        self.text = self.color_list.yellow_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to blue.
    pub fn on_blue(&mut self) -> &mut Self {
        self.text = self.color_list.blue_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to magenta.
    pub fn on_magenta(&mut self) -> &mut Self {
        self.text = self.color_list.magenta_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to cyan.
    pub fn on_cyan(&mut self) -> &mut Self {
        self.text = self.color_list.cyan_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to white.
    pub fn on_white(&mut self) -> &mut Self {
        self.text = self.color_list.white_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color bright black.
    pub fn on_bright_black(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_black_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright red.
    pub fn on_bright_red(&mut self) -> &mut Self {
        self.text = self.color_list.bright_red_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright green.
    pub fn on_bright_green(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_green_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright yellow.
    pub fn on_bright_yellow(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_yellow_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright blue.
    pub fn on_bright_blue(&mut self) -> &mut Self {
        self.text = self.color_list.bright_blue_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright magenta.
    pub fn on_bright_magenta(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_magenta_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright cyan.
    pub fn on_bright_cyan(&mut self) -> &mut Self {
        self.text = self.color_list.bright_cyan_bg.to_string() + &self.text + self.color_list.reset;
        self
    }

    /// This function sets the background color to bright white.
    pub fn on_bright_white(&mut self) -> &mut Self {
        self.text =
            self.color_list.bright_white_bg.to_string() + &self.text + self.color_list.reset;
        self
    }
}

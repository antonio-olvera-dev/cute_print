use crate::StyleList;

use super::super::ColorList;

/// The `CuteText` struct contains a string of text, a reference to a `ColorList`
/// prev_len that is length of text before run add_text, and color_before_text_length is the length of the color before the text.
pub struct CuteText {
    pub text: String,
    pub color_list: ColorList,
    pub prev_len: usize,
    pub color_before_text_length: usize,
    pub style_list: StyleList,
}

impl CuteText {
    /// Adds the color passed by parameter to `self.text` and updates its text.
    ///
    /// Equivalent to calling `add_style_or_color(color)`.
    ///
    /// # Arguments
    ///
    /// * `color: &str` - The color that will be added to the `self.text`.
    ///
    /// # Returns
    ///
    /// A mutable reference to the updated `self.text`.
    fn add_color(&mut self, color: &str) -> &mut Self {
        self.add_style_or_color(color)
    }

    /// Adds the style passed by parameter to `self.text` and updates its text.
    ///
    /// Equivalent to calling `add_style_or_color(style)`.
    ///
    /// # Arguments
    ///
    /// * `style: &str` - The style that will be added to the `self.text`.
    ///
    /// # Returns
    ///
    /// A mutable reference to the updated `self.text`.
    fn add_style(&mut self, style: &str) -> &mut Self {
        self.add_style_or_color(style)
    }

    /// Adds the style or color or color passed by parameter to `self.text` and updates its text.
    ///
    /// If `self.prev_len` is greater than 0 it extracts the text that was there before using the `add_text` function 
    /// and adds the style or color or color to the text that was added when using the `add_text` function.
    ///
    /// # Arguments
    ///
    /// * `style or color: &str` - The style or color or color that will be added to the `self.text`.
    ///
    /// # Returns
    ///
    /// A mutable reference to the updated `self.text`.
    fn add_style_or_color(&mut self, style_or_color: &str) -> &mut Self {
        if self.prev_len > 0 {
            let before_text: &str = &self.text[0..self.prev_len];
            let after_text: &str = &self.text[self.prev_len..self.text.len()];
            let modified_text: String = before_text.to_owned() + style_or_color + after_text;
            self.text = modified_text;
        }

        if self.prev_len == 0 {
            self.color_before_text_length += style_or_color.len();
            self.text = style_or_color.to_string() + &self.text;
        }
        self.text = self.text.to_owned() + self.color_list.reset;
        self
    }
}

impl CuteText {
    /// This function creates a new instance of the `CuteText` struct.
    pub fn new() -> Self {
        CuteText {
            text: String::new(),
            color_list: ColorList::new(),
            prev_len: 0,
            color_before_text_length: 0,
            style_list: StyleList::new(),
        }
    }

    /// This function sets the text color to black.
    pub fn black(&mut self) -> &mut Self {
        self.add_color(self.color_list.black_fg);
        self
    }

    /// This function sets the text color to red.
    pub fn red(&mut self) -> &mut Self {
        self.add_color(self.color_list.red_fg);
        self
    }

    /// This function sets the text color to green.
    pub fn green(&mut self) -> &mut Self {
        self.add_color(self.color_list.green_fg);
        self
    }

    /// This function sets the text color to yellow.
    pub fn yellow(&mut self) -> &mut Self {
        self.add_color(self.color_list.yellow_fg);
        self
    }

    /// This function sets the text color to blue.
    pub fn blue(&mut self) -> &mut Self {
        self.add_color(self.color_list.blue_fg);
        self
    }

    /// This function sets the text color to magenta.
    pub fn magenta(&mut self) -> &mut Self {
        self.add_color(self.color_list.magenta_fg);
        self
    }

    /// This function sets the text color to cyan.
    pub fn cyan(&mut self) -> &mut Self {
        self.add_color(self.color_list.cyan_fg);
        self
    }

    /// This function sets the text color to white.
    pub fn white(&mut self) -> &mut Self {
        self.add_color(self.color_list.white_fg);
        self
    }

    /// This function sets the text color to bright black.
    pub fn bright_black(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_black_fg);
        self
    }

    /// This function sets the text color to bright red.
    pub fn bright_red(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_red_fg);
        self
    }

    /// This function sets the text color to bright green.
    pub fn bright_green(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_green_fg);
        self
    }

    /// This function sets the text color to bright yellow.
    pub fn bright_yellow(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_yellow_fg);
        self
    }

    /// This function sets the text color to bright blue.
    pub fn bright_blue(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_blue_fg);
        self
    }

    /// This function sets the text color to bright magenta.
    pub fn bright_magenta(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_magenta_fg);
        self
    }

    /// This function sets the text color to bright cyan.
    pub fn bright_cyan(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_cyan_fg);
        self
    }

    /// This function sets the text color to bright white.
    pub fn bright_white(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_white_fg);
        self
    }

    /// This function sets the background color to on_black.
    pub fn on_black(&mut self) -> &mut Self {
        self.add_color(self.color_list.black_bg);
        self
    }

    /// This function sets the background color to red.
    pub fn on_red(&mut self) -> &mut Self {
        self.add_color(self.color_list.red_bg);
        self
    }

    /// This function sets the background color to green.
    pub fn on_green(&mut self) -> &mut Self {
        self.add_color(self.color_list.green_bg);
        self
    }

    /// This function sets the background color to yellow.
    pub fn on_yellow(&mut self) -> &mut Self {
        self.add_color(self.color_list.yellow_bg);
        self
    }

    /// This function sets the background color to blue.
    pub fn on_blue(&mut self) -> &mut Self {
        self.add_color(self.color_list.blue_bg);
        self
    }

    /// This function sets the background color to magenta.
    pub fn on_magenta(&mut self) -> &mut Self {
        self.add_color(self.color_list.magenta_bg);
        self
    }

    /// This function sets the background color to cyan.
    pub fn on_cyan(&mut self) -> &mut Self {
        self.add_color(self.color_list.cyan_bg);
        self
    }

    /// This function sets the background color to white.
    pub fn on_white(&mut self) -> &mut Self {
        self.add_color(self.color_list.white_bg);
        self
    }

    /// This function sets the background color bright black.
    pub fn on_bright_black(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_black_bg);
        self
    }

    /// This function sets the background color to bright red.
    pub fn on_bright_red(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_red_bg);
        self
    }

    /// This function sets the background color to bright green.
    pub fn on_bright_green(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_green_bg);
        self
    }

    /// This function sets the background color to bright yellow.
    pub fn on_bright_yellow(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_yellow_bg);
        self
    }

    /// This function sets the background color to bright blue.
    pub fn on_bright_blue(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_blue_bg);
        self
    }

    /// This function sets the background color to bright magenta.
    pub fn on_bright_magenta(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_magenta_bg);
        self
    }

    /// This function sets the background color to bright cyan.
    pub fn on_bright_cyan(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_cyan_bg);
        self
    }

    /// This function sets the background color to bright white.
    pub fn on_bright_white(&mut self) -> &mut Self {
        self.add_color(self.color_list.bright_white_bg);
        self
    }

    /// This function sets the bold text
    pub fn bold(&mut self) -> &mut Self {
        self.add_style(self.style_list.bold);
        self
    }

    /// This function sets the dim text
    pub fn dim(&mut self) -> &mut Self {
        self.add_style(self.style_list.dim);
        self
    }

    /// This function sets the underline text
    pub fn underline(&mut self) -> &mut Self {
        self.add_style(self.style_list.underline);
        self
    }

    /// This function sets the blink text
    pub fn blink(&mut self) -> &mut Self {
        self.add_style(self.style_list.blink);
        self
    }

    /// This function sets the reverse text
    pub fn reverse(&mut self) -> &mut Self {
        self.add_style(self.style_list.reverse);
        self
    }

    /// This function sets the hidden text
    pub fn hidden(&mut self) -> &mut Self {
        self.add_style(self.style_list.hidden);
        self
    }
    /// This function fills the variable self.prev_len with the length of the text before the new text is added
    /// and then adds the text passed as a parameter to self.text.
    ///
    /// # Arguments
    ///
    /// `text: &str` - The text that will be added to the self.text.
    ///
    /// # Returns
    ///
    /// `&mut Self ` - A mutable reference to the updated
    pub fn add_text(&mut self, text: &str) -> &mut Self {
        self.prev_len = self.text.len();
        self.text = self.text.to_string() + text;
        self
    }

    /// Adds a new text at the beginning of the current text applying its colors.
    pub fn add_text_at_the_beginning(&mut self, text_to_add: &str) -> &mut Self {
        let text: &str = &self.text[self.color_before_text_length..self.text.len()];
        let color: &str = &self.text[0..self.color_before_text_length];
        let new_text: String = color.to_owned() + text_to_add + text;
        self.text = new_text;
        self
    }
}

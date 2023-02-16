use super::cute_text::CuteText;
use crate::get_terminal_width;

/// CutePrint struct is used to store and print a list of `CuteText` objects.
pub struct CutePrint {
    /// A vector that contains `CuteText` objects.
    pub cute_text_list: Vec<CuteText>,
}

impl CutePrint {
    /// Creates a new instance of the `CutePrint` struct.

    pub fn new() -> Self {
        CutePrint {
            cute_text_list: Vec::new(),
        }
    }

    /// Adds a new line of text to the `cute_text_list`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text that will be added to the list.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the newly added `CuteText` object.
    pub fn add_line(&mut self, text: &str) -> &mut CuteText {
        let mut cute_text = CuteText::new();

        cute_text.text = text.to_string();
        self.cute_text_list.push(cute_text);

        let length = self.cute_text_list.len() - 1;
        return &mut self.cute_text_list[length];
    }

    /// Prints all lines in the `cute_text_list`.
    pub fn print(&self) {
        for line in &self.cute_text_list {
            println!("{}", line.text)
        }
    }
}

impl CutePrint {
    /// Transforms the text into a numbered list
    pub fn to_numbered_list(&mut self) {
        let point: &str = ".";
        let space: &str = " ";
        let space_more_point: String = point.to_owned() + space;

        for (index, cute_text) in &mut self.cute_text_list.iter_mut().enumerate() {
            let text_to_add: String = (index + 1).to_string() + &space_more_point;
            cute_text.add_text_at_the_beginning(&text_to_add);
        }
    }

    /// Adds a new line with the specified character
    pub fn split(&mut self, character_for_print: char) {
        let type_split: &str = &character_for_print.to_string();
        let width_terminal: u16 = match get_terminal_width() {
            Some(width) => width,
            None => 3,
        };

        self.add_line(&type_split.repeat(width_terminal as usize));
    }
}

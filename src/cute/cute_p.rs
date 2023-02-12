use super::cute_text::CuteText;

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

use super::cute_text::CuteText;
use crate::{get_terminal_width, ColorList};

/// CutePrint struct is used to store and print a list of `CuteText` objects.
pub struct CutePrint {
    /// A vector that contains `CuteText` objects.
    pub cute_text_list: Vec<CuteText>,
}

impl CutePrint {
    fn get_last_cute_text(&mut self) -> &mut CuteText {
        let length: usize = self.cute_text_list.len() - 1;
        return &mut self.cute_text_list[length];
    }

    fn add_cute_text_with_repeated_character(
        &mut self,
        character_for_print: char,
        repetitions: u16,
    ) {
        let mut cute_text: CuteText = CuteText::new();

        cute_text.text = character_for_print.to_string().repeat(repetitions as usize);
        self.cute_text_list.push(cute_text);
    }
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

        return self.get_last_cute_text();
    }

    /// Prints all lines in the `cute_text_list`.
    pub fn print(&self) {
        for line in &self.cute_text_list {
            println!("{}", line.text)
        }
    }

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

    /// Transforms the text into a custom numbered list.
    /// 
    /// # Arguments
    /// 
    /// * `cute_text_for_list: CuteText` - The cute_text that will be use for added to custom list.
    pub fn to_custom_numbered_list(&mut self, cute_text_for_list: CuteText) {
        let color_list: ColorList = ColorList::new();
        let point: &str = ".";
        let space: &str = " ";

        let cute_text_for_list_without_reset: String =
            cute_text_for_list.text.replace(&color_list.reset, "");

        for (index, cute_text) in &mut self.cute_text_list.iter_mut().enumerate() {
            let text_to_add: String = cute_text_for_list_without_reset.to_owned()
                + &(index + 1).to_string()
                + &point
                + color_list.reset
                + space;
            cute_text.text = text_to_add + &cute_text.text;
        }
    }

    /// Adds a new cute line with the specified character and repeat it whit the specificated number.
    /// If the number is equal to None the line  repeat to fill terminal width.
    ///
    /// # Arguments
    ///
    /// * `character_for_print: char` - The char that will be added to the list and repeat.
    /// * `number_of_repetitions: Option<u16>` - The repetition number of char.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the newly added `CuteText` object.
    pub fn split(
        &mut self,
        character_for_print: char,
        number_of_repetitions: Option<u16>,
    ) -> &mut CuteText {
        let width_terminal: u16 = match get_terminal_width() {
            Some(width) => width,
            None => 3,
        };

        let repetitions: u16 = match number_of_repetitions {
            Some(rep) => rep,
            None => width_terminal,
        };

        self.add_cute_text_with_repeated_character(character_for_print, repetitions);

        return self.get_last_cute_text();
    }

    /// Add a CuteText with empty text for line break.
    pub fn line_break(&mut self) {
        let cute_text: CuteText = CuteText::new();
        self.cute_text_list.push(cute_text);
    }
}

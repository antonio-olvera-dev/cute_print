use super::cute_text::CuteText;
pub struct CutePrint {
    pub cute_text_list: Vec<CuteText>,
}

impl CutePrint {
    pub fn new() -> Self {
        CutePrint {
            cute_text_list: Vec::new(),
        }
    }

    pub fn add_line(&mut self, text: &str) -> &mut CuteText {
        let mut cute_text = CuteText::new();

        cute_text.text = text.to_string();
        self.cute_text_list.push(cute_text);

        let length = self.cute_text_list.len() - 1;
        return &mut self.cute_text_list[length];
    }

    pub fn print(&self) {
        for line in &self.cute_text_list {
            println!("{}", line.text)
        }
    }
}

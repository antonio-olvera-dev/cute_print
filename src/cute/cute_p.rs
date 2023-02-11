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

#[cfg(test)]
mod tests {
    use crate::ColorList;

    use super::CutePrint;

    impl PartialEq for CutePrint {
        fn eq(&self, other: &Self) -> bool {
            self.cute_text_list.len() == other.cute_text_list.len()
                && self.cute_text_list[0].text == other.cute_text_list[0].text
        }
    }

    #[test]
    fn test_cute_print_new() {
        let mut cute_print_new: CutePrint = CutePrint::new();
        let mut cute_print: CutePrint = CutePrint {
            cute_text_list: Vec::new(),
        };

        cute_print.add_line("red on_black").red().on_black();
        cute_print_new.add_line("red on_black").red().on_black();

        let cute_print_eq = cute_print == cute_print_new;
        assert!(cute_print_eq);
    }

    #[test]
    fn test_cute_print_add_line() {
        let mut cute_print: CutePrint = CutePrint::new();

        assert_eq!(cute_print.cute_text_list.len(), 0);

        cute_print.add_line("red on_black").red().on_black();
        let color_list: ColorList = ColorList::new();

        assert_eq!(cute_print.cute_text_list.len(), 1);
        assert_eq!(
            cute_print.cute_text_list[0].text,
            color_list.black_bg.to_owned()
                + color_list.red_fg
                + "red on_black"
                + color_list.reset
                + color_list.reset
        );
    }

    #[test]
    fn test_cute_print_print() {
        let mut cute_print: CutePrint = CutePrint::new();
        cute_print.add_line("red on_black").red().on_black();
        assert_eq!(cute_print.print(), ());
    }
}

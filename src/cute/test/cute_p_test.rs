#[cfg(test)]
mod tests {
    use crate::{ColorList, CutePrint};

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

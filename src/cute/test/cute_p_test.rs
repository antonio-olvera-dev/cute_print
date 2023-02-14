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
    fn add_text_at_the_beginning() {
        let color_list: ColorList = ColorList::new();
        let mut cute_print: CutePrint = CutePrint::new();

        let text_for_mock_line: String = String::from("Initial Text");
        let text_for_add: &str = "Initial Text 2";

        cute_print.add_line(&text_for_mock_line).red().on_green();
        cute_print
            .add_line(&text_for_mock_line)
            .red()
            .on_green()
            .add_text(text_for_add)
            .blue();
        cute_print.to_numbered_list();

        let text_at_the_beginning: &str = "1. ";
        let text_at_the_beginning2: &str = "2. ";
        let expected_text: String = color_list.green_bg.to_owned()
            + color_list.red_fg
            + text_at_the_beginning
            + &text_for_mock_line
            + color_list.reset
            + color_list.reset;

        let expected_text2: String = color_list.green_bg.to_owned()
            + color_list.red_fg
            + text_at_the_beginning2
            + &text_for_mock_line
            + color_list.reset
            + color_list.reset
            + color_list.blue_fg
            + text_for_add
            + color_list.reset;

        assert_eq!(expected_text, cute_print.cute_text_list[0].text);
        assert_eq!(expected_text2, cute_print.cute_text_list[1].text);
    }

    #[test]
    fn test_cute_print_print() {
        let mut cute_print: CutePrint = CutePrint::new();
        cute_print.add_line("red on_black").red().on_black();
        assert_eq!(cute_print.print(), ());
    }
}

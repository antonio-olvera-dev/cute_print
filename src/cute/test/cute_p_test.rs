#[cfg(test)]
mod cute_tests {
    use crate::{ColorList, CutePrint, CuteText, StyleList};

    impl PartialEq for CutePrint {
        fn eq(&self, other: &Self) -> bool {
            self.cute_text_list.len() == other.cute_text_list.len()
                && self.cute_text_list[0].text == other.cute_text_list[0].text
        }
    }

    #[test]
    fn test_new() {
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
    fn test_add_line() {
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
    fn test_add_text() {
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
    fn test_print() {
        let mut cute_print: CutePrint = CutePrint::new();
        cute_print.add_line("red on_black").red().on_black();
        assert_eq!(cute_print.print(), ());
    }

    #[test]
    fn test_split() {
        let color_list: ColorList = ColorList::new();
        let mut cute_print: CutePrint = CutePrint::new();
        let mut character_for_print: char = '-';
        let mut number_of_repetitions: Option<u16> = None;
        let mut expected_text: &str = "---";

        cute_print.split(character_for_print, number_of_repetitions);
        assert!(cute_print.cute_text_list[0].text.contains(expected_text));

        number_of_repetitions = Some(4);
        cute_print.split(character_for_print, number_of_repetitions);
        expected_text = "----";
        assert_eq!(cute_print.cute_text_list[1].text, expected_text);

        character_for_print = '=';
        number_of_repetitions = Some(2);
        let expected_text_binding: String = color_list.blue_fg.to_owned() + "==" + color_list.reset;
        expected_text = &expected_text_binding;
        cute_print
            .split(character_for_print, number_of_repetitions)
            .blue();
        assert_eq!(cute_print.cute_text_list[2].text, expected_text);

        let expected_text_binding: String = color_list.yellow_bg.to_owned()
            + color_list.blue_fg
            + "=="
            + &color_list.reset.repeat(2);
        expected_text = &expected_text_binding;
        cute_print
            .split(character_for_print, number_of_repetitions)
            .blue()
            .on_yellow();
        assert_eq!(cute_print.cute_text_list[3].text, expected_text);

        let expected_text_binding: String = color_list.yellow_bg.to_owned()
            + color_list.blue_fg
            + "=="
            + &color_list.reset.repeat(2)
            + color_list.red_fg
            + "text"
            + color_list.reset;
        expected_text = &expected_text_binding;
        cute_print
            .split(character_for_print, number_of_repetitions)
            .blue()
            .on_yellow()
            .add_text("text")
            .red();
        assert_eq!(cute_print.cute_text_list[4].text, expected_text);
    }

    #[test]
    fn test_line_break() {
        let mut cute_print: CutePrint = CutePrint::new();
        let expected_text: &str = "text 1";
        let expected_text2: &str = "";
        let expected_text3: &str = "text 2";
        cute_print.add_line(expected_text);
        cute_print.line_break();
        cute_print.add_line(expected_text3);

        assert_eq!(cute_print.cute_text_list[0].text, expected_text);
        assert_eq!(cute_print.cute_text_list[1].text, expected_text2);
        assert_eq!(cute_print.cute_text_list[2].text, expected_text3);
    }

    #[test]
    fn test_styles() {
        let style_list: StyleList = StyleList::new();
        let color_list: ColorList = ColorList::new();
        let mut cute_print: CutePrint = CutePrint::new();
        let text_1: &str = "text 1";
        let text_2: &str = "text 2";

        let expected_1: String = style_list.bold.to_owned() + text_1 + color_list.reset;
        let expected_2: String =
            style_list.dim.to_owned() + color_list.red_fg + text_2 + &color_list.reset.repeat(2);

        cute_print.add_line(text_1).bold();
        assert_eq!(cute_print.cute_text_list[0].text, expected_1);

        cute_print.add_line(text_2).red().dim();
        assert_eq!(cute_print.cute_text_list[1].text, expected_2);
    }

    #[test]
    fn test_to_custom_numbered_list() {
        let style_list: StyleList = StyleList::new();
        let color_list: ColorList = ColorList::new();
        let mut cute_print: CutePrint = CutePrint::new();
        let text: &str = "text";

        let expected_1: String =
            style_list.bold.to_owned() + color_list.red_fg + "1." + color_list.reset + " " + text;
        let expected_2: String = style_list.bold.to_owned()
            + color_list.red_fg
            + "2."
            + color_list.reset
            + " "
            + color_list.blue_fg
            + text
            + &color_list.reset.repeat(1);

        cute_print.add_line(text);
        cute_print.add_line(text).blue();

        let mut cute_text_for_list: CuteText = CuteText::new();
        cute_text_for_list.red().bold();
        cute_print.to_custom_numbered_list(cute_text_for_list);

        assert_eq!(cute_print.cute_text_list[0].text, expected_1);
        assert_eq!(cute_print.cute_text_list[1].text, expected_2);
    }
}

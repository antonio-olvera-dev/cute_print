use cute_print::{example_colors::print_colors, CutePrint, CuteText};
fn main() {
    print_colors();

    let mut cute_print: CutePrint = CutePrint::new();

    cute_print
        .add_line("yellow on bright black, line 1")
        .yellow()
        .on_bright_black();

    cute_print
        .add_line("red on blue, line 2.")
        .red()
        .on_blue()
        .add_text(" added green text, ")
        .green()
        .add_text("added yellow text.")
        .yellow();

    cute_print.add_line("normal text");
    cute_print.add_line("text");

    cute_print.to_numbered_list();
    cute_print.split('-');

    let mut split_cute: CuteText = CuteText::new();
    split_cute.add_text("=").red();
    cute_print.split_cute(split_cute);
    let mut split_cute: CuteText = CuteText::new();
    split_cute.add_text("=").yellow();
    cute_print.split_cute(split_cute);
    let mut split_cute: CuteText = CuteText::new();
    split_cute.add_text("=").red();
    cute_print.split_cute(split_cute);

    cute_print.print();
}

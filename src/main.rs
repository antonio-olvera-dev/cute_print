use cute_print::{example_colors::print_colors, CutePrint};
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

    cute_print.split('-', None);

    cute_print.split('=', None).red();
    cute_print.split('=', None).yellow();
    cute_print.split('=', None).red();

    cute_print.split(':', Some(4)).green();
    cute_print.split(':', Some(8)).white();
    cute_print.split(':', None).green();
    cute_print.print();
}

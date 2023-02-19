use cute_print::{example_colors::print_colors, CutePrint};
fn main() {
    print_colors();

    let mut cute_print_title: CutePrint = CutePrint::new();
    let mut cute_print: CutePrint = CutePrint::new();

    cute_print_title
        .add_line("Choose an option")
        .magenta();
    cute_print_title.line_break();
    cute_print_title.print();

    cute_print.add_line("bold text").bold();
    cute_print.add_line("dim text").dim();
    cute_print.add_line("underline text").underline();
    cute_print.add_line("blink text").blink();
    cute_print.add_line("reverse text").reverse();
    cute_print.add_line("hidden text").hidden();
    cute_print.to_numbered_list();

    cute_print.split('-', None);
    cute_print.line_break();

    cute_print.split('=', None).red();
    cute_print.split('=', None).yellow();
    cute_print.split('=', None).red();
    cute_print.line_break();


    cute_print.split(':', Some(4)).green();
    cute_print.split(':', Some(8)).white();
    cute_print.split(':', None).green();
    cute_print.print();
}

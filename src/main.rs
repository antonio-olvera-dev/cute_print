use cute_print::{example_colors::print_colors, CutePrint};
fn main() {
    print_colors();

    let mut cute_print_title: CutePrint = CutePrint::new();
    let mut cute_print: CutePrint = CutePrint::new();

    cute_print_title
        .add_line("Choose an option")
        .magenta();
    cute_print_title.add_line("");
    cute_print_title.print();

    cute_print.add_line("read file");
    cute_print.add_line("delete file");
    cute_print.add_line("write file");
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

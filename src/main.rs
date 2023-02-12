use cute_print::{example_colors::print_colors, CutePrint};
fn main() {
    print_colors();

    let mut cute_print: CutePrint = CutePrint::new();
    cute_print
        .add_line("yellow on bright black")
        .yellow()
        .on_bright_black();
    cute_print.print();
}

use cute_print::{examples::example_colors, CutePrint};
fn main() {
    example_colors::print_colors();
    let mut  c = CutePrint::new();
    c.add_line("example").yellow().on_bright_black();
    c.print();

}

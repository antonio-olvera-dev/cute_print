use cute_print::{example_colors::print_colors, CutePrint, CuteText};
fn main() {
    print_colors();

    let mut cute_print_title: CutePrint = CutePrint::new();
    let mut cute_print: CutePrint = CutePrint::new();

    cute_print_title.add_line("Choose an option").magenta();
    cute_print_title.line_break();
    cute_print_title.print();

    cute_print.add_line("normal text");
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

    let mut custom_numbered_list: CutePrint = CutePrint::new();
    custom_numbered_list.add_line("Apple");
    custom_numbered_list.add_line("Banana");
    custom_numbered_list.add_line("Peach");
    custom_numbered_list.add_line("Pear");
    custom_numbered_list.add_line("Cherry");

    let mut cute_text_for_list: CuteText = CuteText::new();
    cute_text_for_list.red().bold();
    custom_numbered_list.to_custom_numbered_list(cute_text_for_list);
    custom_numbered_list.print();
}

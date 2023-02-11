use super::super::CutePrint;

pub fn print_colors() {
    let mut cute: CutePrint = CutePrint::new();

    cute.add_line("black").black();
    cute.add_line("red").red();
    cute.add_line("green").green();
    cute.add_line("yellow").yellow();
    cute.add_line("blue").blue();
    cute.add_line("magenta").magenta();
    cute.add_line("cyan").cyan();
    cute.add_line("white").white();
    cute.add_line("bright_black").bright_black();
    cute.add_line("bright_red").bright_red();
    cute.add_line("bright_green").bright_green();
    cute.add_line("bright_yellow").bright_yellow();
    cute.add_line("bright_blue").bright_blue();
    cute.add_line("bright_magenta").bright_magenta();
    cute.add_line("bright_cyan").bright_cyan();
    cute.add_line("bright_white").bright_white();

    cute.add_line("BG black_bg").on_black();
    cute.add_line("BG red_bg").on_red();
    cute.add_line("BG green_bg").on_green();
    cute.add_line("BG yellow_bg").on_yellow();
    cute.add_line("BG blue_bg").on_blue();
    cute.add_line("BG magenta_bg").on_magenta();
    cute.add_line("BG cyan_bg").on_cyan();
    cute.add_line("BG white_bg").on_white();
    cute.add_line("BG bright_black_bg").on_bright_black();
    cute.add_line("BG bright_red_bg").on_bright_red();
    cute.add_line("BG bright_green_bg").on_bright_green();
    cute.add_line("BG bright_yellow_bg").on_bright_yellow();
    cute.add_line("BG bright_blue_bg").on_bright_blue();
    cute.add_line("BG bright_magenta_bg").on_bright_magenta();
    cute.add_line("BG bright_cyan_bg").on_bright_cyan();
    cute.add_line("BG bright_white_bg").on_bright_white();

    cute.add_line("Normal");
    cute.add_line("Green on White").green().on_white();
    cute.add_line("Blue on Red").blue().on_red();

    cute.print();
}

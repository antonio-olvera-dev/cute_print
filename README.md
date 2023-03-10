# CUTE_PRINT

CUTE_PRINT allows users to print text with different background colours and styles. The main class, CutePrint, has a number of methods that allow users to set the background colour and style of the text.

## Simple example to print a line

```rust
use cute_print::CutePrint;

fn main() {
    let mut cute_print = CutePrint::new();
    cute_print.add_line("yellow on bright black").yellow().on_bright_black();
    cute_print.print();
}
```

## Simple example to print text styles

```rust
use cute_print::CutePrint;

fn main() {
    let mut cute_print: CutePrint = CutePrint::new();

    cute_print.add_line("normal text");
    cute_print.add_line("bold text").bold();
    cute_print.add_line("dim text").dim();
    cute_print.add_line("underline text").underline();
    cute_print.add_line("blink text").blink();
    cute_print.add_line("reverse text").reverse();
    cute_print.add_line("hidden text").hidden();
    cute_print.to_numbered_list();

    cute_print.print();
}
```

## Simple example to print line break

```rust
use cute_print::CutePrint;

fn main() {
    let mut cute_print = CutePrint::new();
    cute_print.add_line("Line 1").magenta();
    cute_print.line_break();
    cute_print.add_line("Line 2").cyan();
    cute_print.print();
}
```

## Simple example to print a line and add text with different color

```rust
use cute_print::CutePrint;

fn main() {
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

    cute_print.print();
}
```

## Simple example to print a numbered list

```rust
use cute_print::CutePrint;

fn main() {
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
    
    cute_print.to_numbered_list();
    cute_print.print();
}
```

## Simple example to print a custom numbered list

```rust
use cute_print::{CutePrint, CuteText};

fn main() {
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
```

## Simple example to print a split lines

```rust
use cute_print::CutePrint;

fn main() {

    let mut cute_print: CutePrint = CutePrint::new();

    cute_print.split('-', None);

    cute_print.split('=', None).red();
    cute_print.split('=', None).yellow();
    cute_print.split('=', None).red();

    cute_print.split(':', Some(4)).green();
    cute_print.split(':', Some(8)).white();
    cute_print.split(':', None).green();
    cute_print.print();
}
```

## Example to see all text colours and backgrounds 

```rust
use cute_print::{examples::example_colors};

fn main() {
    example_colors::print_colors();
}
```

## The above example executes this function

```rust
/// Prints a demonstration of all the color and background color combinations
/// supported by the `CutePrint` struct.
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
```

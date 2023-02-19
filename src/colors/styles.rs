pub struct StyleList {
    pub bold: &'static str,
    pub dim: &'static str,
    pub underline: &'static str,
    pub blink: &'static str,
    pub reverse: &'static str,
    pub hidden: &'static str,
}

impl StyleList {
    pub fn new() -> Self {
        StyleList {
            bold: "\x1b[1m",
            dim: "\x1b[2m",
            underline: "\x1b[4m",
            blink: "\x1b[5m",
            reverse: "\x1b[7m",
            hidden: "\x1b[8m",
        }
    }
}

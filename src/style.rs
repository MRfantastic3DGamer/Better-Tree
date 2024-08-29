pub struct Style {
    pub tl: &'static str,
    pub tr: &'static str,
    pub bl: &'static str,
    pub br: &'static str,
    pub is: &'static str,
    pub h : &'static str,
    pub v : &'static str,
}

pub static BASIC_STYLE: Style = Style {
    tl: "╭", tr: "╮", bl: "╰", br: "╯", is: "├", h: "─", v: "│",
};

//pub static DOUBLE_LINE_STYLE: Style = Style {
//    tl: "╔", tr: "╗", bl: "╚", br: "╝", is: "═", h: "═", v: "║",
//};

pub static HEAVY_STYLE: Style = Style {
    tl: "┏", tr: "┓", bl: "┗", br: "┛", is: "┣", h: "━", v: "┃",
};

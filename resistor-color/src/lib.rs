use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, IntEnum, Clone, Copy, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    let all_color = all::<ResistorColor>().collect::<Vec<_>>();
    for color in all_color {
        if color == _color {
            return color.int_value();
        }
    }
    0
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(x) => format!("{:?}", x),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}

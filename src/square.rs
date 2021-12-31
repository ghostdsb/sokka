#[derive(Copy, Clone, PartialEq)]
pub enum Color{
    BLACK,
    WHITE
}

pub struct Square{
    pub x: u32,
    pub y: u32,
    pub col: Color
}

impl Square{
    pub fn new(x: u32, y: u32, col: Color)  -> Self{
        Self{x,y,col}
    }
}

impl Color{
    pub fn invert(col: Color) -> Color{
        match col {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK
        }
    }
}
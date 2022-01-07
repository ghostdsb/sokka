use crate::piece::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Color{
    BLACK,
    WHITE
}

pub struct Square{
    pub x: u32,
    pub y: u32,
    pub col: Color,
    pub piece: Option<Piece>
}

impl Square{
    pub fn new(x: u32, y: u32, col: Color, piece: Option<Piece>)  -> Self{
        Self{x,y,col, 
            piece: piece}
    }
    pub fn place(&mut self, piece: Piece) {
        self.piece = Some(piece);
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
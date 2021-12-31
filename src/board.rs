use crate::square::*;

pub struct Board{
    pub squares: Vec<Square>,
    pub cell_width: u32,
}

impl Board{
    pub fn new() -> Self{
        let mut squares: Vec<Square> = vec![];
        let cell_width = 100;
        let mut col = Color::WHITE;
        for i in 0..8{
            for j in 0..8{
                let x = j*cell_width;
                let y = i*cell_width;
                squares.push(Square::new(x, y, col));
                col = Color::invert(col);
            }
            col = Color::invert(col);
        }

        Self{
            squares,
            cell_width
        }
    }
}
use crate::square;
use crate::piece;

pub struct Board{
    pub squares: Vec<square::Square>,
    pub cell_width: u32,
}

fn fen_to_grid(fen_string: &str) -> Vec<Option<String>>{
    let mut piece_vector: Vec<Option<String>> = vec![];
    let mut piece_placement_str = fen_string.split_whitespace().take(1);
    for line in piece_placement_str.next().unwrap().split("/").collect::<Vec<&str>>(){
        for p in line.chars(){
            if p.is_digit(10) {
                for _ in 0..p.to_digit(10).unwrap(){
                    piece_vector.push(None);
                }
            }else{
                let piece = p.to_string();
                piece_vector.push(Some(piece));

            }
        }
    }
    piece_vector
}

impl Board{
    
    pub fn new(state: &str) -> Self{
        let mut squares: Vec<square::Square> = vec![];
        let cell_width = 100_u32;
        let mut col = square::Color::WHITE;
        let pieces = fen_to_grid(state);
        for i in 0..8{
            for j in 0..8{
                let x = j*cell_width;
                let y = i*cell_width;
                let piece = match pieces.iter().nth((i*8+j) as usize).unwrap(){
                    Some(p_str) => Some(piece::Piece::new(p_str)),
                    None => None
                };
                squares.push(square::Square::new(x as u32, y as u32, col, piece));
                col = square::Color::invert(col);
            }
            col = square::Color::invert(col);
        }

        Self{
            squares,
            cell_width
        }
    }

}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color{
    BLACK,
    WHITE
}

#[derive(Copy, Clone, Debug)]
pub struct Piece{
    p_type: (Color, PieceType),
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType{
    KING,
    QUEEN,
    BISHOP,
    KNIGHT,
    ROOK,
    PAWN
}

impl Piece{
    pub fn new(piece_type: &str) -> Self{
        let piece = match piece_type {
            "k" => (Color::BLACK, PieceType::KING),
            "q" => (Color::BLACK, PieceType::QUEEN),
            "r" => (Color::BLACK, PieceType::ROOK),
            "b" => (Color::BLACK, PieceType::BISHOP),
            "n" => (Color::BLACK, PieceType::KNIGHT),
            "p" => (Color::BLACK, PieceType::PAWN),
            "K" => (Color::WHITE, PieceType::KING),
            "Q" => (Color::WHITE, PieceType::QUEEN),
            "R" => (Color::WHITE, PieceType::ROOK),
            "B" => (Color::WHITE, PieceType::BISHOP),
            "N" => (Color::WHITE, PieceType::KNIGHT),
            "P" => (Color::WHITE, PieceType::PAWN),
            _ => unreachable!()
        };

        Piece{
            p_type: piece
        }
    }

    pub fn get_color(self) -> Color{
        self.p_type.0
    }

    pub fn get_rank(self) -> PieceType{
        self.p_type.1
    }

    pub fn get_rank_str(self) -> String{
        let rank_str = match self.p_type{
            (Color::BLACK, PieceType::KING) => "k" ,
            (Color::BLACK, PieceType::QUEEN) => "q" ,
            (Color::BLACK, PieceType::ROOK) => "r" ,
            (Color::BLACK, PieceType::BISHOP) => "b" ,
            (Color::BLACK, PieceType::KNIGHT) => "n" ,
            (Color::BLACK, PieceType::PAWN) => "p" ,
            (Color::WHITE, PieceType::KING) => "K" ,
            (Color::WHITE, PieceType::QUEEN) => "Q" ,
            (Color::WHITE, PieceType::ROOK) => "R" ,
            (Color::WHITE, PieceType::BISHOP) => "B" ,
            (Color::WHITE, PieceType::KNIGHT) => "N" ,
            (Color::WHITE, PieceType::PAWN) => "P" ,
            _ => unreachable!()
        };
        String::from(rank_str)
    }
}
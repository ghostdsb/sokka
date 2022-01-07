use macroquad::prelude::*;
mod board;
mod square;
mod piece;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        let initial_state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq";
        let initial_state = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let initial_state = "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1.";

        let board = board::Board::new(initial_state);

        for square in board.squares{
            let col = match square.col {
                square::Color::BLACK => BLACK,
                square::Color::WHITE => WHITE,
            };
            draw_rectangle(square.x as f32, square.y as f32, board.cell_width as f32, board.cell_width as f32, col);
            match square.piece{
                Some(piece) => {
                    let color = match piece.get_color(){
                        piece::Color::BLACK => GREEN,
                        piece::Color::WHITE => RED,
                    };
                    draw_text(&piece.get_rank_str(), square.x as f32 + board.cell_width as f32/2.0, square.y as f32 + board.cell_width as f32/2.0, 75.0, color);
                },
                None => {
                    draw_text("", square.x as f32 + board.cell_width as f32/2.0, square.y as f32 + board.cell_width as f32/2.0, 75.0, BROWN);

                }
            };
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "SOKKA".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}
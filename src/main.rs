use macroquad::prelude::*;
mod board;
mod square;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        let board = board::Board::new();

        for square in board.squares{
            let col = match square.col {
                square::Color::BLACK => BLACK,
                square::Color::WHITE => GRAY,
            };
            draw_rectangle(square.x as f32, square.y as f32, board.cell_width as f32, board.cell_width as f32, col)
        }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}
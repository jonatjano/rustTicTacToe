mod board;

use crate::board::*;

fn main() {
    let mut board: Board = Board {
        ..Default::default()
    };
    while !board.ended() {
        print!("{}", board);
        board.play();
    }
    print!("{}", board);
    println!(
        "Winner is {}",
        match board.winner() {
            CellState::Empty => "Nobody",
            CellState::Circle => "Circle",
            CellState::Cross => "Cross",
        }
    );
}

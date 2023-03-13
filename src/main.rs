mod board;

use crate::board::*;

fn main() {
    let mut board: Board = Board{..Default::default()};
    while ! board.ended() {
        print!("{}", board);
        board.play();
    }
    print!("{}", board);
    println!("Winner is {}", match board.winner() {
        CellState::EMPTY => "Nobody",
        CellState::CIRCLE => "Circle",
        CellState::CROSS => "Cross"
    });
}

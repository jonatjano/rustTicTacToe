use std::fmt::{Display, Formatter};

#[derive(PartialEq, Copy, Clone)]
pub enum CellState {
    Empty,
    Circle,
    Cross,
}

impl CellState {
    fn to_str(&self) -> &'static str {
        match self {
            CellState::Empty => " ",
            CellState::Circle => "O",
            CellState::Cross => "X",
        }
    }
}

pub struct Board {
    pub board: [CellState; 9],
    pub first_player_turn: bool,
}

impl Board {
    pub(crate) fn ended(&self) -> bool {
        return self.winner() != CellState::Empty
            || self.board.iter().all(|cell| cell != &CellState::Empty);
    }

    pub(crate) fn winner(&self) -> CellState {
        // row
        if self.board[0] == self.board[1] && self.board[0] == self.board[2] {
            self.board[0]
        } else if self.board[3] == self.board[4] && self.board[3] == self.board[5] {
            self.board[3]
        } else if self.board[6] == self.board[7] && self.board[6] == self.board[8] {
            self.board[6]
        }
        // col
        else if self.board[0] == self.board[3] && self.board[0] == self.board[6] {
            self.board[0]
        } else if self.board[1] == self.board[4] && self.board[1] == self.board[7] {
            self.board[3]
        } else if self.board[2] == self.board[5] && self.board[2] == self.board[8] {
            self.board[6]
        }
        //diag
        else if self.board[0] == self.board[4] && self.board[0] == self.board[8] {
            self.board[0]
        } else if self.board[2] == self.board[4] && self.board[2] == self.board[6] {
            self.board[2]
        } else {
            CellState::Empty
        }
    }

    pub(crate) fn play(&mut self) {
        use std::io;
        let mut i = 0;
        println!(
            "{}'s turn",
            if self.first_player_turn {
                "Cross"
            } else {
                "Circle"
            }
        );
        println!("Select where to play (1-9)");
        loop {
            let stdin = io::stdin();
            let mut user_input = String::new();
            match stdin.read_line(&mut user_input) {
                Ok(_n) => {}
                Err(e) => println!("{}", e),
            };
            match user_input.trim().parse() {
                Ok(n) => {
                    i = n;
                }
                Err(e) => println!("{}", e),
            };
            if !(1..=9).contains(&i) {
                continue;
            }
            if self.board[i - 1] == CellState::Empty {
                break;
            }
            println!("This cell is already used select another one (1-9)");
        }
        println!("Playing on cell {}", i);
        self.board[i - 1] = if self.first_player_turn {
            CellState::Cross
        } else {
            CellState::Circle
        };
        self.first_player_turn = !self.first_player_turn;
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: [
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
            ],
            first_player_turn: true,
        }
    }
}

impl Display for Board {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "+---+---+---+")?;
        for (i, cell) in self.board.iter().enumerate() {
            write!(fmt, "| {} ", cell.to_str())?;
            if i % 3 == 2 {
                fmt.write_str("|\n+---+---+---+\n")?;
            }
        }
        Ok(())
    }
}

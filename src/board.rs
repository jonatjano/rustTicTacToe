#[derive(PartialEq, Copy, Clone)]
pub enum CellState {
    EMPTY,
    CIRCLE,
    CROSS
}

impl CellState {
    pub fn to_string(&self) -> String {
        match self {
            CellState::EMPTY => String::from(" "),
            CellState::CIRCLE => String::from("O"),
            CellState::CROSS => String::from("X")
        }
    }
}

pub struct Board {
    pub board: [CellState; 9],
    pub first_player_turn: bool
}

impl Board {
    pub(crate) fn ended(&self) -> bool {
        return self.winner() != CellState::EMPTY || self.board.iter().all(|cell| cell != &CellState::EMPTY)
    }

    pub(crate) fn winner(&self) -> CellState {
        // row
        if self.board[0] == self.board[1] && self.board[0] == self.board[2] { self.board[0] }
        else if self.board[3] == self.board[4] && self.board[3] == self.board[5] { self.board[3] }
        else if self.board[6] == self.board[7] && self.board[6] == self.board[8] { self.board[6] }
        // col
        else if self.board[0] == self.board[3] && self.board[0] == self.board[6] { self.board[0] }
        else if self.board[1] == self.board[4] && self.board[1] == self.board[7] { self.board[3] }
        else if self.board[2] == self.board[5] && self.board[2] == self.board[8] { self.board[6] }
        //diag
        else if self.board[0] == self.board[4] && self.board[0] == self.board[8] { self.board[0] }
        else if self.board[2] == self.board[4] && self.board[2] == self.board[6] { self.board[2] }

        else { CellState::EMPTY }
    }

    pub(crate) fn play(&mut self) {
        use std::io;
        let mut i = 0;
        println!("{}'s turn", if self.first_player_turn { "Cross" } else { "Circle" });
        println!("Select where to play (1-9)");
        loop {
            let stdin = io::stdin();
            let mut user_input = String::new();
            match stdin.read_line(&mut user_input) {
                Ok(_n) => {}
                Err(e) => println!("{}", e)
            };
            match user_input.trim().parse() {
                Ok(n) => { i = n; }
                Err(e) => println!("{}", e)
            };
            if i < 1 || i > 9 { continue; }
            if self.board[i-1] == CellState::EMPTY { break; }
            println!("This cell is already used select another one (1-9)");
        }
        println!("Playing on cell {}", i);
        self.board[i-1] = if self.first_player_turn { CellState::CROSS } else { CellState::CIRCLE };
        self.first_player_turn = ! self.first_player_turn;
    }

    fn to_string(&self) -> String {
        let mut str = String::from("+---+---+---+\n");
        for (i, cell) in self.board.iter().enumerate() {
            str += &(String::from("| ") + cell.to_string().as_str() + " ");
            if i % 3 == 2 {
                str += "|\n+---+---+---+\n";
            }
        }
        return str.to_string();
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            board: [
                CellState::EMPTY, CellState::EMPTY, CellState::EMPTY,
                CellState::EMPTY, CellState::EMPTY, CellState::EMPTY,
                CellState::EMPTY, CellState::EMPTY, CellState::EMPTY
            ],
            first_player_turn: true
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.to_string())
    }
}
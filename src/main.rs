struct Board {
    board: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Board {
    fn dead_state(width: usize, height: usize) -> Board {
        let mut board = Vec::new();

        for _row in 0..height {
            board.push(vec![false; width]);
        }

        Board {
            board,
            width,
            height,
        }
    }

    fn random_state(width: usize, height: usize) -> Board {
        let mut board = Vec::new();

        for _row in 0..height {
            let mut board_row: Vec<bool> = Vec::new();
            for _cell in 0..width {
                board_row.push(rand::random());
            }
            board.push(board_row);
        }

        Board {
            board,
            width,
            height,
        }
    }

    fn render(&self) {
        println!("+{:-<1$}+", "", self.width);
        for row in 0..self.height {
            print!("|");
            for cell in &self.board[row] {
                match cell {
                    true => print!("#"),
                    false => print!(" "),
                    _ => (),
                };
            }
            print!("|\n");
        }
        println!("+{:-<1$}+", "", self.width);
    }
}

fn main() {
    let board = Board::random_state(80, 30);

    board.render();
}

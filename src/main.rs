use std::{thread, time};

pub struct Board {
    board: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn dead_state(&self, width: usize, height: usize) -> Board {
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

    pub fn random_state(width: usize, height: usize) -> Board {
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

    pub fn render(&self) {
        println!("+{:-<1$}+", "", self.width);
        for row in 0..self.height {
            print!("|");
            for cell in &self.board[row] {
                match cell {
                    true => print!("#"),
                    false => print!(" "),
                };
            }
            print!("|\n");
        }
        println!("+{:-<1$}+", "", self.width);
    }

    pub fn next_board_state(&self) -> Board {
        let mut new_board = self.dead_state(self.width, self.height);

        for x_pos in 0..self.width {
            for y_pos in 0..self.height {
                let neighbours = get_neighbours(x_pos, y_pos, &self);
                let num_trues = neighbours.iter().filter(|x| x == &&true).count();

                if num_trues < 2 {
                    new_board.board[y_pos][x_pos] = false;
                } else if (num_trues == 2 || num_trues == 3) && self.board[y_pos][x_pos] == true {
                    new_board.board[y_pos][x_pos] = true;
                } else if num_trues == 3 && self.board[y_pos][x_pos] == false {
                    new_board.board[y_pos][x_pos] = true;
                } else if num_trues > 3 {
                    new_board.board[y_pos][x_pos] = false;
                }
            }
        }
        
        new_board
    }
}

fn main() {
    let mut board = Board::random_state(80, 30);

    for _i in 0..100 {
        board.render();
        board = board.next_board_state();
        thread::sleep(time::Duration::from_millis(250));
    }
}

fn get_neighbours(x_pos: usize, y_pos: usize, board: &Board) -> Vec<bool> {
    let neighbours: Vec<bool>;
    let width = &board.width;
    let height = &board.height;

    if x_pos == 0 && y_pos == 0 {
        neighbours = vec!(board.board[0][1], board.board[1][1], board.board[1][0])
    } else if x_pos == 0 && y_pos == height-1 {
        neighbours = vec!(board.board[y_pos-1][0], board.board[y_pos][1], board.board[y_pos-1][1])
    } else if x_pos == width-1 && y_pos == 0 {
        neighbours = vec!(board.board[1][x_pos], board.board[0][x_pos-1], board.board[1][x_pos-1])
    } else if x_pos == width-1 && y_pos == height-1 {
        neighbours = vec!(board.board[y_pos][x_pos-1], board.board[y_pos-1][x_pos], board.board[y_pos-1][x_pos-1])
    } else if x_pos == 0 {
        neighbours = vec!(board.board[y_pos-1][0], board.board[y_pos+1][0], board.board[y_pos][1], board.board[y_pos+1][1], board.board[y_pos-1][1])
    } else if x_pos == width-1 {
        neighbours = vec!(board.board[y_pos-1][x_pos], board.board[y_pos+1][x_pos], board.board[y_pos][x_pos-1], board.board[y_pos-1][x_pos-1], board.board[y_pos+1][x_pos-1])
    } else if y_pos == 0 {
        neighbours = vec!(board.board[y_pos][x_pos-1], board.board[y_pos][x_pos+1], board.board[y_pos+1][x_pos], board.board[y_pos+1][x_pos+1], board.board[y_pos+1][x_pos-1])
    } else if y_pos == height-1 {
        neighbours = vec!(board.board[y_pos][x_pos-1], board.board[y_pos][x_pos+1], board.board[y_pos-1][x_pos], board.board[y_pos-1][x_pos+1], board.board[y_pos-1][x_pos-1])
    } else {
        neighbours = vec!(board.board[y_pos-1][x_pos-1], board.board[y_pos][x_pos-1], board.board[y_pos+1][x_pos-1], board.board[y_pos-1][x_pos], board.board[y_pos+1][x_pos], board.board[y_pos-1][x_pos+1], board.board[y_pos][x_pos+1], board.board[y_pos+1][x_pos+1])
    }

    neighbours
}

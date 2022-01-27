struct Board {
    board: Vec<Vec<bool>>
}

impl Board {
    fn dead_state(width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut board = Vec::new();

        for _row in 0..height {
            board.push(vec![false; width]);
        }

        board
    }
}

fn main() {
    let board = Board::dead_state(10, 10);

    println!("{:#?}", board)
}

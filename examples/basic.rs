use sudoku;

fn main() {
    let mut board = sudoku::board::Board::empty();
    board.set_cell(0, 0, sudoku::board::Digit::new(5));
    board.set_cell(3, 3, sudoku::board::Digit::new(1));
    println!("{:?}", board.get_cell(0, 0));
    println!("{:?}", board.get_cell(1, 1));
    println!("{:?}", board);
}

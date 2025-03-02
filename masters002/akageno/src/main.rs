use proconio::input;

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub board: Vec<Vec<char>>,
}

fn input() -> Input {
    input! {
        n: usize,
        m: usize,
        c_input: [String; n],
    }

    let mut board: Vec<Vec<char>> = vec![];

    for row in c_input {
        board.push(row.chars().collect());
    }

    Input { n, m, board }
}

fn main() {
    let input = input();
    println!("{:?}", input);
}

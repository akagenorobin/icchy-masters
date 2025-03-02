use proconio::input;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub grid: Vec<Vec<char>>,
}

#[derive(Clone, Debug)]
pub struct Board {
    pub n: usize,
    pub x: usize,
    pub y: usize,
    pub hole_a: (usize, usize),
    pub hole_b: (usize, usize),
    pub hole_c: (usize, usize),
    pub grid: Vec<Vec<char>>,
}

impl Board {
    pub fn new(n: usize, x: usize, y: usize, grid: Vec<Vec<char>>) -> Board {
        let mut hole_a = (0, 0);
        let mut hole_b = (0, 0);
        let mut hole_c = (0, 0);

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 'A' {
                    hole_a = (i, j);
                }
                if grid[i][j] == 'B' {
                    hole_b = (i, j);
                }
                if grid[i][j] == 'C' {
                    hole_c = (i, j);
                }
            }
        }
        Board {n, x, y, hole_a, hole_b, hole_c, grid}
    }

    pub fn mv(&mut self, d: Direction) {
        match d {
            Direction::Right => {self.x += 1},
            Direction::Left => {self.x -= 1},
            Direction::Up => {self.y += 1},
            Direction::Down => {self.y -= 1},
        }
    }

    pub fn carry(&mut self, d: Direction) {
        match d {
            Direction::Right => {
                self.grid[self.x + 1][self.y] = self.grid[self.x][self.y];
                self.grid[self.x][self.y] = '.';
            },
            Direction::Left => {
                self.grid[self.x - 1][self.y] = self.grid[self.x][self.y];
                self.grid[self.x][self.y] = '.';
            },
            Direction::Up => {
                self.grid[self.x][self.y - 1] = self.grid[self.x][self.y];
                self.grid[self.x][self.y] = '.';
            },
            Direction::Down => {
                self.grid[self.x][self.y + 1] = self.grid[self.x][self.y];
                self.grid[self.x][self.y] = '.';
            }
        }
        self.mv(d);
    }
}

fn input() -> Input {
    input! {
        n: usize,
        m: usize,
        c_input: [String; n],
    }

    let mut grid: Vec<Vec<char>> = vec![];

    for row in c_input {
        grid.push(row.chars().collect());
    }

    Input { n, m, grid }
}

fn solve (input: Input) {
    let board = Board::new(input.n, 0, 0, input.grid);

    for _ in 0..input.n * input.n {
        let target = (0, 0);
        for i in 0..n {
            for j in 0..n {
                if board.grid[i][j] == 'A' {

                }
                let target =

            }
        }

    }
    println!("{:?}", board);
}

fn main() {
    let input = input();
    solve(input);
}

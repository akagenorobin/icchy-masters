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

        for j in 0..n {
            for i in 0..n {
                if grid[j][i] == 'A' {
                    hole_a = (i, j);
                }
                if grid[j][i] == 'B' {
                    hole_b = (i, j);
                }
                if grid[j][i] == 'C' {
                    hole_c = (i, j);
                }
            }
        }
        Board {
            n,
            x,
            y,
            hole_a,
            hole_b,
            hole_c,
            grid,
        }
    }

    pub fn now(&mut self) -> char {
        self.grid[self.y][self.x]
    }

    pub fn mv(&mut self, d: Direction) {
        match d {
            Direction::Right => {
                self.x += 1;
                println!("1 R");
            }
            Direction::Left => {
                self.x -= 1;
                println!("1 L");
            }
            Direction::Up => {
                self.y += 1;
                println!("1 U");
            }
            Direction::Down => {
                self.y -= 1;
                println!("1 D");
            }
        }
        println!("")
    }

    pub fn carry(&mut self, d: Direction) {
        match d {
            Direction::Right => {
                if self.grid[self.y][self.x + 1] == '.' {
                    self.grid[self.y][self.x + 1] = self.grid[self.y][self.x];
                }
                self.grid[self.y][self.x] = '.';
            }
            Direction::Left => {
                if self.grid[self.y][self.x - 1] == '.' {
                    self.grid[self.y][self.x - 1] = self.grid[self.y][self.x];
                }
                self.grid[self.y][self.x] = '.';
            }
            Direction::Up => {
                if self.grid[self.y - 1][self.x] == '.' {
                    self.grid[self.y - 1][self.x] = self.grid[self.y][self.x];
                }
                self.grid[self.y][self.x] = '.';
            }
            Direction::Down => {
                if self.grid[self.y + 1][self.x] == '.' {
                    self.grid[self.y + 1][self.x] = self.grid[self.y][self.x];
                }
                self.grid[self.y][self.x] = '.';
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

fn solve(input: Input) {
    let mut board = Board::new(input.n, 0, 0, input.grid);

    for _ in 0..10000 {
        println!("{:?} {:?} {:?}", board.now(), board.x, board.y);

        if board.now() == '.' {
            let mut moved = false;
            let mut d = Direction::Right;

            if board.x < input.n - 1 && !moved {
                if board.grid[board.y][board.x + 1] == 'a' {
                    d = Direction::Right;
                    moved = true;
                }
            }
            if board.x > 0 && !moved {
                if board.grid[board.y][board.x - 1] == 'a' {
                    d = Direction::Left;
                    moved = true;
                }
            }
            if board.y > 0 && !moved {
                if board.grid[board.y - 1][board.x] == 'a' {
                    d = Direction::Up;
                    moved = true;
                }
            }
            if board.y < input.n - 1 && !moved {
                if board.grid[board.y + 1][board.x] == 'a' {
                    d = Direction::Down;
                    moved = true;
                }
            }

            if board.x < input.n - 1 && !moved {
                if board.grid[board.y][board.x + 1] == '.' {
                    d = Direction::Right;
                    moved = true;
                }
            }
            if board.y < input.n - 1 && !moved {
                if board.grid[board.y + 1][board.x] == '.' {
                    d = Direction::Down;
                    moved = true;
                }
            }
            if board.x > 0 && !moved {
                if board.grid[board.y][board.x - 1] == '.' {
                    d = Direction::Left;
                    moved = true;
                }
            }
            if board.y > 0 && !moved {
                if board.grid[board.y - 1][board.x] == '.' {
                    d = Direction::Up;
                    moved = true;
                }
            }

            board.mv(d);
        }
    }
}

fn main() {
    let input = input();
    solve(input);
}

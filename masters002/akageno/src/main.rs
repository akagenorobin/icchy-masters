use proconio::input;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub enum Hole {
    A,
    B,
    C,
}

impl Direction {
    fn to_delta(self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    fn to_str(self) -> char {
        match self {
            Direction::Right => 'R',
            Direction::Left => 'L',
            Direction::Up => 'U',
            Direction::Down => 'D',
        }
    }

    fn all() -> &'static [Direction] {
        &[
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ]
    }
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
    pub num_a: usize,
    pub num_b: usize,
    pub num_c: usize,
    pub hole_a: (usize, usize),
    pub hole_b: (usize, usize),
    pub hole_c: (usize, usize),
    pub grid: Vec<Vec<char>>,
}

impl Board {
    pub fn new(n: usize, grid: Vec<Vec<char>>) -> Board {
        let mut num_a = 0;
        let mut num_b = 0;
        let mut num_c = 0;
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
                if grid[j][i] == 'a' {
                    num_a += 1;
                }
                if grid[j][i] == 'b' {
                    num_b += 1;
                }
                if grid[j][i] == 'c' {
                    num_c += 1;
                }
            }
        }
        Board {
            n,
            x: hole_a.0,
            y: hole_a.1,
            num_a,
            num_b,
            num_c,
            hole_a,
            hole_b,
            hole_c,
            grid,
        }
    }

    pub fn now(&mut self) -> char {
        self.grid[self.y][self.x]
    }

    pub fn mv(&mut self, dir: Direction) {
        let (dx, dy) = dir.clone().to_delta();
        let nx = self.x as isize + dx;
        let ny = self.y as isize + dy;

        (self.x, self.y) = (nx as usize, ny as usize);
        println!("1 {}", dir.to_str());
    }

    pub fn carry(&mut self, dir: Direction) {
        let (dx, dy) = dir.clone().to_delta();
        let nx = self.x as isize + dx;
        let ny = self.y as isize + dy;

        let (nx, ny) = (nx as usize, ny as usize);

        if self.grid[ny][nx] == '.' {
            self.grid[ny][nx] = self.grid[self.y][self.x];
        }

        if !['A', 'B', 'C'].contains(&self.grid[self.y][self.x]) {
            self.grid[self.y][self.x] = '.';
        }

        (self.x, self.y) = (nx as usize, ny as usize);

        println!("2 {}", dir.to_str());
    }

    pub fn roll(&mut self, dir: Direction) {
        self.grid[self.y][self.x] = '.';
        println!("3 {}", dir.to_str());
    }

    pub fn print(&self) {
        for row in self.grid.clone() {
            println!("{}", row.iter().collect::<String>());
        }
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

fn bfs(board: &Board, n: usize, targets: &[char]) -> Option<(char, Vec<Direction>)> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n]; n];
    let mut paths = vec![vec![None; n]; n];

    queue.push_back((board.x, board.y, vec![]));
    visited[board.y][board.x] = true;

    while let Some((x, y, path)) = queue.pop_front() {
        //println!("x: {}, y: {}, grid: {}, path: {:?}", x, y, board.grid[y][x], path);

        for dir in Direction::all() {
            let (dx, dy) = dir.clone().to_delta();
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= n as isize || ny >= n as isize {
                continue; // 範囲外
            }
            let (nx, ny) = (nx as usize, ny as usize);

            let cell = board.grid[ny][nx];
            if targets.contains(&cell) {
                let mut new_path = path.clone();
                new_path.push(dir.clone());

                return Some((cell, new_path)); // target に到達したら即終了
            }

            if board.grid[ny][nx] != '.' || visited[ny][nx] {
                continue;
            }

            visited[ny][nx] = true;
            let mut new_path = path.clone();
            new_path.push(dir.clone());

            paths[ny][nx] = Some(new_path.clone());
            queue.push_back((nx, ny, new_path));
        }
    }

    None
}

fn can_roll(board: &Board, hole: char) -> Option<Direction> {
    if hole == 'A' {
        if board.hole_a.1 == board.y {
            if board.x < board.hole_a.0 {
                let through = (board.x + 1..board.hole_a.0).all(|i| board.grid[board.y][i] == '.');
                if through {
                    return Some(Direction::Right);
                }
            } else if board.x > board.hole_a.0 {
                let through = (board.hole_a.0 + 1..board.x).all(|i| board.grid[board.y][i] == '.');
                if through {
                    return Some(Direction::Left);
                }
            }
        } else if board.hole_a.0 == board.x {
            if board.y < board.hole_a.1 {
                let through = (board.y + 1..board.hole_a.1).all(|j| board.grid[j][board.x] == '.');
                if through {
                    return Some(Direction::Down);
                }
            } else if board.y > board.hole_a.1 {
                let through = (board.hole_a.1 + 1..board.y).all(|j| board.grid[j][board.x] == '.');
                if through {
                    return Some(Direction::Up);
                }
            }
        }
    } else if hole == 'B' {
        if board.hole_b.1 == board.y {
            if board.x < board.hole_b.0 {
                let through = (board.x + 1..board.hole_b.0).all(|i| board.grid[board.y][i] == '.');
                if through {
                    return Some(Direction::Right);
                }
            } else if board.x > board.hole_b.0 {
                let through = (board.hole_b.0 + 1..board.x).all(|i| board.grid[board.y][i] == '.');
                if through {
                    return Some(Direction::Left);
                }
            }
        } else if board.hole_b.0 == board.x {
            if board.y < board.hole_b.1 {
                let through = (board.y + 1..board.hole_b.1).all(|j| board.grid[j][board.x] == '.');
                if through {
                    return Some(Direction::Down);
                }
            } else if board.y > board.hole_b.1 {
                let through = (board.hole_b.1 + 1..board.y).all(|j| board.grid[j][board.x] == '.');
                if through {
                    return Some(Direction::Up);
                }
            }
        }
    } else if hole == 'C' {
        if board.hole_c.1 == board.y {
            if board.x < board.hole_c.0 {
                let through = (board.x + 1..board.hole_c.0).all(|i| board.grid[board.y][i] == '.');
                if through {
                    return Some(Direction::Right);
                }
            } else if board.x > board.hole_c.0 {
                let through = (board.hole_c.0 + 1..board.x).all(|i| board.grid[board.y][i] == '.');
                if through {
                    return Some(Direction::Left);
                }
            }
        } else if board.hole_c.0 == board.x {
            if board.y < board.hole_c.1 {
                let through = (board.y + 1..board.hole_c.1).all(|j| board.grid[j][board.x] == '.');
                if through {
                    return Some(Direction::Down);
                }
            } else if board.y > board.hole_c.1 {
                let through = (board.hole_c.1 + 1..board.y).all(|j| board.grid[j][board.x] == '.');
                if through {
                    return Some(Direction::Up);
                }
            }
        }
    }

    None
}

fn solve(input: Input) {
    let mut board = Board::new(input.n, input.grid);
    let mut clear = 0;
    let mut steps = 0;

    loop {
        let targets = ['a', 'b', 'c'];

        if let Some((t, path)) = bfs(&board, input.n, &targets) {
            for p in path {
                board.mv(p);
                steps += 1;
                if steps == 10000 {
                    break;
                }
            }

            let targets = match t {
                'a' => ['A'],
                'b' => ['B'],
                'c' => ['C'],
                _ => unreachable!(),
            };
            let (t, path) = bfs(&board, input.n, &targets).unwrap_or(('A', vec![]));

            for p in path {
                board.carry(p);
                steps += 1;
                if steps == 10000 {
                    break;
                }

                if let Some(dir) = can_roll(&board, t) {
                    board.roll(dir);
                    steps += 1;
                    break;
                }
            }

            clear += 1;
        } else {
            if let Some((_, path)) = bfs(&board, input.n, &['@']) {
                for p in path {
                    board.mv(p);
                    steps += 1;
                    if steps == 10000 {
                        break;
                    }
                }

                let (t, path) = bfs(&board, input.n, &['A', 'B', 'C']).unwrap_or(('A', vec![]));

                for p in path {
                    board.carry(p);
                    steps += 1;
                    if steps == 10000 {
                        break;
                    }

                    if let Some(dir) = can_roll(&board, t) {
                        board.roll(dir);
                        steps += 1;
                        break;
                    }
                }
            }
        }

        if clear == board.num_a + board.num_b + board.num_c {
            break;
        }
    }
}

fn main() {
    let input = input();
    solve(input);
}

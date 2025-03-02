use proconio::input;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
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
        &[Direction::Right, Direction::Left, Direction::Up, Direction::Down]
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

        if self.grid[self.y][self.x] != 'A' {
            self.grid[self.y][self.x] = '.';
        }

        (self.x, self.y) = (nx as usize, ny as usize);

        println!("2 {}", dir.to_str());

        //for line in self.grid.clone() {
        //    println!("{}", line.iter().collect::<String>());
        //}
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

fn bfs(board: &Board, n: usize, target: char) -> Option<Vec<Direction>> {
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

            if board.grid[ny][nx] == target {
                let mut new_path = path.clone();
                new_path.push(dir.clone());

                return Some(new_path); // target に到達したら即終了
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

fn solve(input: Input) {
    let mut board = Board::new(input.n, input.grid);

    let mut steps = 0;
    for _ in 0..board.num_a {
        let paths = bfs(&board, input.n, 'a').unwrap_or(vec![]);

        for path in paths {
            board.mv(path);
            steps += 1;
            if steps == 9999 {
                break
            }
        }

        let paths = bfs(&board, input.n, 'A').unwrap_or(vec![]);

        for path in paths {
            board.carry(path);
            steps += 1;
            if steps == 9999 {
                break
            }
        }
    }
}

fn main() {
    let input = input();
    solve(input);
}

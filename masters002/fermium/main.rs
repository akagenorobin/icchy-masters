use proconio::input;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum Direction {
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
    pub fn new(n: usize, _x: usize, _y: usize, grid: Vec<Vec<char>>) -> Board {
        let mut hole_a = (0, 0);
        let mut hole_b = (0, 0);
        let mut hole_c = (0, 0);
        let mut player_pos = (0, 0);

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 'A' {
                    hole_a = (i, j);
                    player_pos = (i, j);
                }
                if grid[i][j] == 'B' {
                    hole_b = (i, j);
                }
                if grid[i][j] == 'C' {
                    hole_c = (i, j);
                }
            }
        }
        Board {
            n,
            x: player_pos.1,  // colをxに
            y: player_pos.0,  // rowをyに
            hole_a,
            hole_b,
            hole_c,
            grid
        }
    }

    // 内部用の移動メソッド（出力なし）
    fn move_to(&mut self, d: Direction) {
        match d {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    // 出力付きの移動メソッド
    pub fn mv(&mut self, d: Direction) {
        match d {
            Direction::Right => println!("1 R"),
            Direction::Left => println!("1 L"),
            Direction::Up => println!("1 U"),
            Direction::Down => println!("1 D"),
        }
        self.move_to(d);
    }

    pub fn carry(&mut self, d: Direction) {
        match d {
            Direction::Right => {
                println!("2 R");
                if self.grid[self.y][self.x + 1] == '#' {
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y][self.x + 1] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            },
            Direction::Left => {
                println!("2 L");
                if self.grid[self.y][self.x - 1] == '#' {
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y][self.x - 1] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            },
            Direction::Up => {
                println!("2 U");
                if self.grid[self.y - 1][self.x] == '#' {
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y - 1][self.x] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            },
            Direction::Down => {
                println!("2 D");
                if self.grid[self.y + 1][self.x] == '#' {
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y + 1][self.x] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            }
        }
        self.move_to(d);
    }

    // 現在位置から最も近い鉱石までの経路を探索
    pub fn find_nearest_mineral(&self) -> Option<Vec<Direction>> {
        let mut visited = vec![vec![false; self.n]; self.n];
        let mut queue = VecDeque::new();
        
        // 初期位置をキューに追加
        queue.push_back(SearchNode {
            pos: (self.y, self.x),
            path: vec![],
        });
        visited[self.y][self.x] = true;

        let directions = [
            (0, 1, Direction::Right),
            (0, -1, Direction::Left),
            (-1, 0, Direction::Up),
            (1, 0, Direction::Down),
        ];

        while let Some(node) = queue.pop_front() {
            let (row, col) = node.pos;

            // 現在位置が鉱石なら経路を返す（ただし、経路の長さが0の場合は無視）
            if self.grid[row][col].is_ascii_lowercase() && !node.path.is_empty() {
                return Some(node.path);
            }

            for &(dr, dc, ref dir) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                if new_row < 0 || new_row >= self.n as i32 || 
                   new_col < 0 || new_col >= self.n as i32 {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if !visited[new_row][new_col] 
                    && self.grid[new_row][new_col] != '@'
                    && !self.grid[new_row][new_col].is_ascii_uppercase() {
                    visited[new_row][new_col] = true;
                    let mut new_path = node.path.clone();
                    new_path.push(dir.clone());
                    queue.push_back(SearchNode {
                        pos: (new_row, new_col),
                        path: new_path,
                    });
                }
            }
        }

        None
    }

    // デバッグ用：現在の盤面状態を表示
    pub fn print_state(&self) {
        println!("\nCurrent board state:");
        println!("Player position: ({}, {})", self.y, self.x);
        println!("Grid:");
        for i in 0..self.n {
            for j in 0..self.n {
                if i == self.y && j == self.x {
                    print!("P"); // プレイヤーの位置
                } else {
                    print!("{}", self.grid[i][j]);
                }
            }
            println!();
        }
        println!();
    }
}

// 経路探索用の構造体
#[derive(Clone, Debug)]
struct SearchNode {
    pos: (usize, usize),  // 現在位置
    path: Vec<Direction>, // スタートからの経路
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
    let mut turn_count = 0;
    
    // 到達可能な鉱石がなくなるまでループ
    while let Some(path) = board.find_nearest_mineral() {
        // 経路に沿ってプレイヤーを移動
        for direction in &path {
            board.mv(direction.clone());
            turn_count += 1;
            if turn_count >= 10000 {
                return;
            }
        }
        
        // 逆順に辿って鉱石を運ぶ
        for direction in path.iter().rev() {
            let reverse_direction = match direction {
                Direction::Right => Direction::Left,
                Direction::Left => Direction::Right,
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
            };
            board.carry(reverse_direction);
            turn_count += 1;
            if turn_count >= 10000 {
                return;
            }
        }
    }
}

fn main() {
    let input = input();
    solve(input);
}

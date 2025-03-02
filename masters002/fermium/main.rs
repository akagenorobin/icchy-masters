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
                if self.grid[self.y][self.x + 1] == '#' || 
                   self.grid[self.y][self.x + 1] == 'A' ||
                   self.grid[self.y][self.x + 1] == 'B' ||
                   self.grid[self.y][self.x + 1] == 'C' {
                    // 穴の上に物体を置く場合、物体を完全に消去
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y][self.x + 1] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            },
            Direction::Left => {
                println!("2 L");
                if self.grid[self.y][self.x - 1] == '#' ||
                   self.grid[self.y][self.x - 1] == 'A' ||
                   self.grid[self.y][self.x - 1] == 'B' ||
                   self.grid[self.y][self.x - 1] == 'C' {
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y][self.x - 1] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            },
            Direction::Up => {
                println!("2 U");
                if self.grid[self.y - 1][self.x] == '#' ||
                   self.grid[self.y - 1][self.x] == 'A' ||
                   self.grid[self.y - 1][self.x] == 'B' ||
                   self.grid[self.y - 1][self.x] == 'C' {
                    self.grid[self.y][self.x] = '.';
                } else {
                    self.grid[self.y - 1][self.x] = self.grid[self.y][self.x];
                    self.grid[self.y][self.x] = '.';
                }
            },
            Direction::Down => {
                println!("2 D");
                if self.grid[self.y + 1][self.x] == '#' ||
                   self.grid[self.y + 1][self.x] == 'A' ||
                   self.grid[self.y + 1][self.x] == 'B' ||
                   self.grid[self.y + 1][self.x] == 'C' {
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
    // 鉱石の位置を全て取得
    let mut minerals = Vec::new();
    for i in 0..board.n {
        for j in 0..board.n {
            if board.grid[i][j] == 'a' {
                minerals.push((i, j));
            }
        }
    }

    // 鉱石をグループ分け
    let mut groups = Vec::new();
    let mut visited = vec![false; minerals.len()];
    
    for i in 0..minerals.len() {
        if visited[i] {
            continue;
        }
        
        let mut group = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(minerals[i]);
        visited[i] = true;
        group.push(minerals[i]);

        while let Some(current) = queue.pop_front() {
            for j in 0..minerals.len() {
                if visited[j] {
                    continue;
                }
                
                // BFSで現在の鉱石から他の鉱石に到達可能か確認
                let mut visited_cells = vec![vec![false; board.n]; board.n];
                let mut q = VecDeque::new();
                q.push_back(current);
                visited_cells[current.0][current.1] = true;
                
                let mut reachable = false;
                while let Some((cy, cx)) = q.pop_front() {
                    if (cy, cx) == minerals[j] {
                        reachable = true;
                        break;
                    }
                    
                    for (dy, dx) in [(0,1), (1,0), (0,-1), (-1,0)].iter() {
                        let ny = (cy as i32 + dy) as usize;
                        let nx = (cx as i32 + dx) as usize;
                        
                        if ny < board.n && nx < board.n && 
                           !visited_cells[ny][nx] && 
                           (board.grid[ny][nx] == '.' || board.grid[ny][nx] == 'a') {
                            visited_cells[ny][nx] = true;
                            q.push_back((ny, nx));
                        }
                    }
                }
                
                if reachable {
                    visited[j] = true;
                    group.push(minerals[j]);
                    queue.push_back(minerals[j]);
                }
            }
        }
        
        if !group.is_empty() {
            groups.push(group);
        }
    }
    // // グループの内容を出力
    // println!("鉱石グループ:");
    // for (i, group) in groups.iter().enumerate() {
    //     println!("グループ {}: {:?}", i + 1, group);
    // }
    // 各グループに対して処理
    'group_loop: for group in &groups {
        loop {  // 新しいループを追加
            // 現在位置から各鉱石グループへの最短経路を探索
            let mut min_score = std::usize::MAX;
            let mut best_path = Vec::new();
            
            // BFS用のキュー
            let mut queue = VecDeque::new();
            let mut visited = vec![vec![None; board.n]; board.n];
            
            // 現在位置をキューに追加
            queue.push_back(((board.y, board.x), Vec::new(), 0));
            visited[board.y][board.x] = Some((Vec::new(), 0));
            
            while let Some(((cy, cx), path, score)) = queue.pop_front() {
                // グループ内の鉱石に到達した場合、最小スコアを更新
                if board.grid[cy][cx] == 'a' && group.contains(&(cy, cx)) {
                    if score < min_score {
                        min_score = score;
                        best_path = path.clone();
                    }
                    continue;
                }
                
                // 4方向を探索
                for (dy, dx) in [(0,1), (1,0), (0,-1), (-1,0)].iter() {
                    let ny = (cy as i32 + dy) as usize;
                    let nx = (cx as i32 + dx) as usize;
                    
                    if ny >= board.n || nx >= board.n {
                        continue;
                    }
                    
                    // 岩を通過する場合は1000点、それ以外は1点加算
                    let new_score = score + if board.grid[ny][nx] == '@' { 1000 } else { 1 };
                    
                    // 未訪問、もしくはより少ないスコアで到達可能な場合
                    if visited[ny][nx].is_none() || 
                       visited[ny][nx].as_ref().unwrap().1 > new_score {
                        let mut new_path = path.clone();
                        new_path.push((ny, nx));
                        
                        queue.push_back(((ny, nx), new_path.clone(), new_score));
                        visited[ny][nx] = Some((new_path, new_score));
                    }
                }
            }

            // 最短経路が見つからなかった場合は次のグループへ
            if best_path.is_empty() {
                break;
            }

            // まず、経路上の全ての岩を処理
            let mut current_pos = (board.y, board.x);
            let mut path_index = 0;

            while path_index < best_path.len() {
                let next_pos = best_path[path_index];
                
                let dy = next_pos.0 as i32 - current_pos.0 as i32;
                let dx = next_pos.1 as i32 - current_pos.1 as i32;
                let direction = match (dy, dx) {
                    (0, 1) => Direction::Right,
                    (0, -1) => Direction::Left, 
                    (-1, 0) => Direction::Up,
                    (1, 0) => Direction::Down,
                    _ => continue
                };

                if board.grid[next_pos.0][next_pos.1] == '@' {
                    // 岩まで移動
                    board.mv(direction);
                    current_pos = next_pos;
                    
                    // 来た道を逆にたどって岩を運ぶ
                    let mut rock_pos = current_pos;
                    for &prev_pos in best_path[..path_index + 1].iter().rev() {
                        let dy = prev_pos.0 as i32 - rock_pos.0 as i32;
                        let dx = prev_pos.1 as i32 - rock_pos.1 as i32;
                        let direction = match (dy, dx) {
                            (0, 1) => Direction::Right,
                            (0, -1) => Direction::Left, 
                            (-1, 0) => Direction::Up,
                            (1, 0) => Direction::Down,
                            _ => continue
                        };
                        
                        board.carry(direction);
                        rock_pos = prev_pos;
                    }

                    // 最後に穴まで運ぶ
                    let hole_pos = board.hole_a;
                    let dy = hole_pos.0 as i32 - rock_pos.0 as i32;
                    let dx = hole_pos.1 as i32 - rock_pos.1 as i32;
                    let direction = if dx.abs() > dy.abs() {
                        if dx > 0 { Direction::Right } else { Direction::Left }
                    } else {
                        if dy > 0 { Direction::Down } else { Direction::Up }
                    };
                    board.carry(direction);
                    
                    // 岩を運び終わったら、最初の位置から再開
                    current_pos = (board.y, board.x);
                    path_index = 0;
                    continue;
                } else if board.grid[next_pos.0][next_pos.1] == 'a' {
                    // 鉱石に到達したら移動して運ぶ
                    board.mv(direction);
                    current_pos = next_pos;

                    // 来た道を逆にたどって鉱石を運ぶ
                    let mut mineral_pos = current_pos;
                    for &prev_pos in best_path[..path_index + 1].iter().rev() {
                        let dy = prev_pos.0 as i32 - mineral_pos.0 as i32;
                        let dx = prev_pos.1 as i32 - mineral_pos.1 as i32;
                        let direction = match (dy, dx) {
                            (0, 1) => Direction::Right,
                            (0, -1) => Direction::Left,
                            (-1, 0) => Direction::Up,
                            (1, 0) => Direction::Down,
                            _ => continue
                        };
                        board.carry(direction.clone());
                        mineral_pos = prev_pos;
                    }

                    // 最後に穴まで運ぶ
                    let hole_pos = board.hole_a;
                    let mut current = mineral_pos;
                    while board.grid[current.0][current.1] != '#' && 
                          board.grid[current.0][current.1] != 'A' &&
                          board.grid[current.0][current.1] != 'B' &&
                          board.grid[current.0][current.1] != 'C' {
                        let dy = hole_pos.0 as i32 - current.0 as i32;
                        let dx = hole_pos.1 as i32 - current.1 as i32;
                        let direction = if dx.abs() > dy.abs() {
                            if dx > 0 { Direction::Right } else { Direction::Left }
                        } else {
                            if dy > 0 { Direction::Down } else { Direction::Up }
                        };
                        board.carry(direction.clone());
                        match direction {
                            Direction::Right => current.1 += 1,
                            Direction::Left => current.1 -= 1,
                            Direction::Up => current.0 -= 1,
                            Direction::Down => current.0 += 1,
                        }
                    }
                    break;
                } else {
                    // 通常の移動
                    board.mv(direction);
                    current_pos = next_pos;
                    path_index += 1;
                }
            }

            // 経路上の岩を全て処理した後、トンネルを使って到達可能な鉱石を全て回収
            loop {
                // 現在位置から到達可能な鉱石を探索（岩は通らない）
                let mut queue = VecDeque::new();
                let mut visited = vec![vec![false; board.n]; board.n];
                let mut found_mineral = None;
                let mut found_path = Vec::new();

                queue.push_back(((board.y, board.x), Vec::new()));
                visited[board.y][board.x] = true;

                while let Some((pos, path)) = queue.pop_front() {
                    let (cy, cx) = pos;

                    // 鉱石を見つけた場合
                    if board.grid[cy][cx] == 'a' && group.contains(&(cy, cx)) {
                        found_mineral = Some(pos);
                        found_path = path;
                        break;
                    }

                    // 4方向を探索（岩は通らない）
                    for (dy, dx) in [(0,1), (1,0), (0,-1), (-1,0)].iter() {
                        let ny = (cy as i32 + dy) as usize;
                        let nx = (cx as i32 + dx) as usize;

                        if ny >= board.n || nx >= board.n {
                            continue;
                        }

                        if !visited[ny][nx] && 
                           board.grid[ny][nx] != '@' && 
                           !board.grid[ny][nx].is_ascii_uppercase() {
                            visited[ny][nx] = true;
                            let mut new_path = path.clone();
                            new_path.push((ny, nx));
                            queue.push_back(((ny, nx), new_path));
                        }
                    }
                }

                // 到達可能な鉱石がなければ終了
                if found_mineral.is_none() {
                    break;
                }

                // 鉱石まで移動
                let mut current_pos = (board.y, board.x);
                for &next_pos in &found_path {
                    let dy = next_pos.0 as i32 - current_pos.0 as i32;
                    let dx = next_pos.1 as i32 - current_pos.1 as i32;
                    let direction = match (dy, dx) {
                        (0, 1) => Direction::Right,
                        (0, -1) => Direction::Left,
                        (-1, 0) => Direction::Up,
                        (1, 0) => Direction::Down,
                        _ => continue
                    };
                    board.mv(direction);
                    current_pos = next_pos;
                }

                // 来た道を逆にたどって鉱石を運ぶ
                let mut mineral_pos = current_pos;
                for &prev_pos in found_path.iter().rev() {
                    let dy = prev_pos.0 as i32 - mineral_pos.0 as i32;
                    let dx = prev_pos.1 as i32 - mineral_pos.1 as i32;
                    let direction = match (dy, dx) {
                        (0, 1) => Direction::Right,
                        (0, -1) => Direction::Left,
                        (-1, 0) => Direction::Up,
                        (1, 0) => Direction::Down,
                        _ => continue
                    };
                    board.carry(direction.clone());
                    mineral_pos = prev_pos;
                }

                // 最後に穴まで運ぶ（ここを修正）
                let hole_pos = board.hole_a;
                let mut current = mineral_pos;
                while board.grid[current.0][current.1] != '#' && 
                      board.grid[current.0][current.1] != 'A' &&
                      board.grid[current.0][current.1] != 'B' &&
                      board.grid[current.0][current.1] != 'C' {
                    let dy = hole_pos.0 as i32 - current.0 as i32;
                    let dx = hole_pos.1 as i32 - current.1 as i32;
                    let direction = if dx.abs() > dy.abs() {
                        if dx > 0 { Direction::Right } else { Direction::Left }
                    } else {
                        if dy > 0 { Direction::Down } else { Direction::Up }
                    };
                    board.carry(direction.clone());
                    match direction {
                        Direction::Right => current.1 += 1,
                        Direction::Left => current.1 -= 1,
                        Direction::Up => current.0 -= 1,
                        Direction::Down => current.0 += 1,
                    }
                }
            }

            break 'group_loop;
        }
    }
}

fn main() {
    let input = input();
    solve(input);
}

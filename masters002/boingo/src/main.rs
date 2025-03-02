use proconio::input;

fn find_first_a(vec: &Vec<char>) -> Option<usize> {
    for (index, &item) in vec.iter().enumerate() {
        if item == 'a' {
            return Some(index);
        }
    }
    None
}

fn find_last_a(vec: &Vec<char>) -> Option<usize> {
    let mut last_index: Option<usize> = None;

    for (index, &item) in vec.iter().enumerate() {
        if item == 'a' {
            last_index = Some(index);
        }
    }

    last_index
}

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [String; n],
    }
    // gridを2次元の文字ベクタに変換
    let grid: Vec<Vec<char>> = grid.into_iter().map(|s| s.chars().collect()).collect();

    // 現在位置 (pi, pj) の取得
    let mut pi = 0;
    let mut pj = 0;
    'outer: for i in 0..n {
        for j in 0..n {
            // 元のコードではここで m ではなく n を使用しています
            if grid[i][j] == 'A' {
                pi = i;
                pj = j;
                break 'outer;
            }
        }
    }

    // あなの位置を記憶
    let holei = pi;
    let holej = pj;


    // 右に移動
    for j in (pj + 1)..n {
        let c = grid[pi][j];

        let index = find_last_a(&grid[pi]);
        if index == None { continue; }
        let index = index.unwrap() ;
        if index <= pj { continue; }

        if c == '@' {
            if j > index { continue; }

            let steps = j - pj;
            for _ in 0..steps {
                println!("1 R");
            }
            println!("3 L");
            pj = j;
        }

        if c >= 'a' && c <= 'z' {
            let steps = j - pj;
            for _ in 0..steps {
                println!("1 R");
            }
            println!("3 L");
            pj = j;
        }

    }

    for count in 0..(pj - holej) {
        println!("1 L");
    }
    pj = holej;

    // 左に移動
    for j in (0..pj).rev() {
        let c = grid[pi][j];

        let index = find_first_a(&grid[pi]);
        if index == None { continue; }
        let index = index.unwrap() ;
        if index >= pj { continue; }

        if c == '@' {
            if j < index { continue; }

            let steps = (pj - j);
            for _ in 0..steps {
                println!("1 L");
            }
            println!("3 R");
            pj = j;
        }

        if c >= 'a' && c <= 'z' {
            let steps = pj - j;
            for _ in 0..steps {
                println!("1 L");
            }
            println!("3 R");
            pj = j;
        }
    }

    for count in 0..(holej - pj) {
        println!("1 R");
    }
    pj = holej;

    // 下方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に転がす
    for i in (pi + 1)..n {
        let c = grid[i][pj];
        pi = i;
        println!("1 D");

        if c == '@' || (c >= 'a' && c <= 'z') {
            println!("3 U");
        }

        // 右方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (pj + 1)..n {
            let c = grid[pi][j];

            let index = find_last_a(&grid[pi]);
            if index == None { continue; }
            let index = index.unwrap() ;
            if index <= pj { continue; }

            if c == '@' {
                if j > index { continue; }

                let steps = j - pj;
                for _ in 0..steps {
                    println!("1 R");
                }
                for _ in 0..steps {
                    println!("2 L");
                }
                println!("3 U");
            }

            if c >= 'a' && c <= 'z' {
                let steps = j - pj;
                for _ in 0..steps {
                    println!("1 R");
                }
                for _ in 0..steps {
                    println!("2 L");
                }
                println!("3 U");
            }

        }

        // 左方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (0..pj).rev() {
            let c = grid[pi][j];
            let index = find_first_a(&grid[pi]);
            if index == None { continue; }
            let index = index.unwrap() ;
            if index >= pj { continue; }

            if c == '@' {
                if j < index { continue; }
                let steps = pj - j;
                for _ in 0..steps {
                    println!("1 L");
                }
                for _ in 0..steps {
                    println!("2 R");
                }
                println!("3 U");
            }

            if c >= 'a' && c <= 'z' {
                let steps = pj - j;
                for _ in 0..steps {
                    println!("1 L");
                }
                for _ in 0..steps {
                    println!("2 R");
                }
                println!("3 U");
            }
        }
    }

    for count in 0..(pi - holei) {
        println!("1 U");
    }
    pi = holei;

    for i in (0..holei).rev() {
        let c = grid[i][pj];
        pi = i;
        println!("1 U");

        if c == '@' || (c >= 'a' && c <= 'z') {
            println!("3 D");
        }

        // 右方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (pj + 1)..n {
            let c = grid[pi][j];

            let index = find_last_a(&grid[pi]);
            if index == None { continue; }
            let index = index.unwrap() ;
            if index <= pj { continue; }

            if c == '@' {
                if j > index { continue; }

                let steps = j - pj;
                for _ in 0..steps {
                    println!("1 R");
                }
                for _ in 0..steps {
                    println!("2 L");
                }
                println!("3 D");
            }

            if c >= 'a' && c <= 'z' {
                let steps = j - pj;
                for _ in 0..steps {
                    println!("1 R");
                }
                for _ in 0..steps {
                    println!("2 L");
                }
                println!("3 D");
            }
        }

        // 左方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (0..pj).rev() {
            let c = grid[pi][j];

            let index = find_first_a(&grid[pi]);
            if index == None { continue; }
            let index = index.unwrap() ;
            if index >= pj { continue; }

            if c == '@' {
                if j < index { continue; }
                let steps = pj - j;
                for _ in 0..steps {
                    println!("1 L");
                }
                for _ in 0..steps {
                    println!("2 R");
                }
                println!("3 D");
            }

            if (c >= 'a' && c <= 'z') {
                let steps = pj - j;
                for _ in 0..steps {
                    println!("1 L");
                }
                for _ in 0..steps {
                    println!("2 R");
                }
                println!("3 D");
            }
        }
    }
}

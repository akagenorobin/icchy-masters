use proconio::input;

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
        if c == '@' || (c >= 'a' && c <= 'z') {
            let steps = j - pj;
            for _ in 0..steps {
                println!("1 R");
            }
            println!("3 L");
            pj = j;
        } else if c >= 'A' && c <= 'Z' {
            break;
        }
    }

    for count in 0..(pj - holej) {
        println!("1 L");
    }
    pj = holej;

    // 左に移動
    for j in (0..pj).rev() {
        let c = grid[pi][j];
        if c == '@' || (c >= 'a' && c <= 'z') {
            let steps = (pj - j);
            for _ in 0..steps {
                println!("1 L");
            }
            println!("3 R");
            pj = j;
        } else if c >= 'A' && c <= 'Z' {
            break;
        }
    }

    for count in 0..(holej - pj) {
        println!("1 R");
    }
    pj = holej;

    // 下方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に転がす
    for i in (pi + 1)..n {
        let c = grid[i][pj];
        if c == '@' || (c >= 'a' && c <= 'z') {
            let steps = i - pi;
            for _ in 0..steps {
                println!("1 D");
            }
            println!("3 U");
            pi = i;
        } else if c >= 'A' && c <= 'Z' {
            break;
        }
    }

    for count in 0..(pi - holei) {
        println!("1 U");
    }
    pi = holei;

    for i in (0..pi).rev() {
        let c = grid[i][pj];
        if c == '@' || (c >= 'a' && c <= 'z') {
            let steps = (pi - i);
            for _ in 0..steps {
                println!("1 U");
            }
            println!("3 D");
            pi = i;
        } else if c >= 'A' && c <= 'Z' {
            break;
        }
    }

    for count in 0..(holei - pi) {
        println!("1 D");
    }
    pi = holei;


    // 上がわの石たちを全て穴に入れる。
    for i in (0..holei).rev() {
        pi = i;
        println!("1 U");
        // 右方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (pj + 1)..n {
            let c = grid[pi][j];
            if c == '@' || (c >= 'a' && c <= 'z') {
                let steps = j - pj;
                for _ in 0..steps {
                    println!("1 R");
                }
                for _ in 0..steps {
                    println!("2 L");
                }
                println!("3 D");
            } else if c >= 'A' && c <= 'Z' {
                break;
            }
        }

        // 左方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (0..pj).rev() {
            let c = grid[pi][j];
            if c == '@' || (c >= 'a' && c <= 'z') {
                let steps = pj - j;
                for _ in 0..steps {
                    println!("1 L");
                }
                for _ in 0..steps {
                    println!("2 R");
                }
                println!("3 D");
            } else if c >= 'A' && c <= 'Z' {
                break;
            }
        }
    }

    for _ in 0..(holei - pi) {
        println!("1 D");
    }
    pi = holei;


    // 下がわの石たちを全て穴に入れる。
    for i in (holei+1)..n {
        pi = i;
        println!("1 D");
        // 右方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (pj + 1)..n {
            let c = grid[pi][j];
            if c == '@' || (c >= 'a' && c <= 'z') {
                let steps = j - pj;
                for _ in 0..steps {
                    println!("1 R");
                }
                for _ in 0..steps {
                    println!("2 L");
                }
                println!("3 U");
            } else if c >= 'A' && c <= 'Z' {
                break;
            }
        }

        // 左方向に移動して岩('@')・鉱石(小文字)を見つけたら穴に運ぶ
        for j in (0..pj).rev() {
            let c = grid[pi][j];
            if c == '@' || (c >= 'a' && c <= 'z') {
                let steps = pj - j;
                for _ in 0..steps {
                    println!("1 L");
                }
                for _ in 0..steps {
                    println!("2 R");
                }
                println!("3 U");
            } else if c >= 'A' && c <= 'Z' {
                break;
            }
        }
    }
}

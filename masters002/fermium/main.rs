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
        for j in 0..n {  // 元のコードではここで m ではなく n を使用しています
            if grid[i][j] == 'A' {
                pi = i;
                pj = j;
                break 'outer;
            }
        }
    }

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
        } else if c >= 'A' && c <= 'Z' {
            break;
        }
    }

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
}
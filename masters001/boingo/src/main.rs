use rand::Rng; // randクレートからRngトレイトを使用
use std::io;

fn main() {
    println!("Enter the size of the grid N:");
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Please type a number!");

    // 壁のデータを保持するためのベクタを初期化
    let mut walls_horizontal = vec![vec![0; n]; n + 1];
    let mut walls_vertical = vec![vec![0; n + 1]; n];

    // ランダムに壁を生成
    let mut rng = rand::thread_rng();
    for i in 0..n {
        for j in 0..n {
            if i < n - 1 {
                walls_horizontal[i + 1][j] = rng.gen_range(0..2); // 下側の壁
            }
            if j < n - 1 {
                walls_vertical[i][j + 1] = rng.gen_range(0..2); // 右側の壁
            }
        }
    }

    let digit = get_digit_count(n*n ) ;

    // マス目と壁を出力
    for i in 0..n {
        for j in 0..n {
            if  j == 0 {print!(" ") ;}
            print!("{:digit$} ", i * n + j + 1);
            if walls_vertical[i][j + 1] == 1 {
                print!("|"); // 右側の壁を表す
            } else {
                print!(" "); // 壁がない場合は空白
            }
        }
        println!();
        // 下側の壁を出力
        for j in 0..n {
            print!(" ");
            if walls_horizontal[i + 1][j] == 1 {
                // 壁がある場合
                for k in 0..digit {
                    print!("-");
                }
            } else {
                for k in 0..digit {
                    print!(" ");
                }
            }
            print!(" ");
        }
        println!();
    }
}

fn get_digit_count(num: usize) -> usize {
    // 整数を文字列に変換し、その長さを返す
    num.to_string().len()
}
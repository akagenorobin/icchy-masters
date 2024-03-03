use proconio::input;

fn solve(n: usize, v: Vec<Vec<i32>>, h: Vec<Vec<i32>>, a: Vec<Vec<i32>>) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    ans.push(format!("{} {} {} {}", 0, 0, n - 1, n - 1));
    ans
}

fn main() {
    input! {
        _t: usize,
        n: usize,
        v_input: [String; n],
        h_input: [String; n-1],
        a: [[i32; n]; n],
    }

    let mut v: Vec<Vec<i32>> = vec![];
    let mut h: Vec<Vec<i32>> = vec![];

    for v_ in v_input {
        let mut _v: Vec<i32> = vec![];
        for v_char in v_.chars() {
            _v.push(v_char as i32 - 48);
        }
        v.push(_v);
    }

    for h_ in h_input {
        let mut _h: Vec<i32> = vec![];
        for h_char in h_.chars() {
            _h.push(h_char as i32 - 48);
        }
        h.push(_h);
    }

    visualize( n, v.clone(), h.clone(), a.clone());

    let ans = crate::solve(n, v, h, a);

    for step in ans {
        println!("{}", step);
    }


}

fn visualize( n: usize, v : Vec<Vec<i32>>, h : Vec<Vec<i32>>, a : Vec<Vec<i32>> ) {

    let digit = get_digit_count(n*n ) ;

    // マス目と壁を出力
    for i in 0..n {
        for j in 0..n {
            if  j == 0 {print!(" ") ;}
            print!("{:digit$} ", a[i][j]);
            if  j == n - 1  {continue;}
            if v[i][j] == 1 {
                print!("|"); // 右側の壁を表す
            } else {
                print!(" "); // 壁がない場合は空白
            }
        }
        println!();

        if  i == n - 1  {continue;}
        // 下側の壁を出力
        for j in 0..n {
            print!(" ");
            if h[i][j] == 1 {
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
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
        v_input: [String; n-1],
        h_input: [String; n],
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

    let ans = solve(n, v, h, a);

    for step in ans {
        println!("{}", step);
    }
}

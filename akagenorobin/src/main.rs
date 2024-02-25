use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; n/m]; m],
    }

    for i in 0..m {
        for j in 0..n/m {
            println!("{}", a[i][j]);
        }
    }
}

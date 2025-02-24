use proconio::input_interactive;

#[derive(Clone, Debug)]
pub struct Input {
    pub eps: f64,
    pub delta: f64,
    pub s: (i64, i64),
    pub ps: Vec<(i64, i64)>,
    pub walls: Vec<(i64, i64, i64, i64)>,
}

fn input() -> Input {
    input_interactive! {
        n: usize,
        m: usize,
        eps: f64,
        delta: f64,
        s: (i64, i64),
        ps: [(i64, i64); n],
        walls: [(i64, i64, i64, i64); m],
    }
    Input {
        eps,
        delta,
        s,
        ps,
        walls,
    }
}
fn solve(input: &Input) {
    for _ in 0..5000 {
        println!("A 0 0");

        input_interactive! {
            c: u64,
            h: u64,
            q: [u64; h],
        }
    }
}
fn main() {
    let input = input();

    solve(&input);
}

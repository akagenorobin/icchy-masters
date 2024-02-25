use proconio::input;

fn solve(target: usize, m: usize, a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut b = a.clone();

    for (i, yama) in a.iter().enumerate() {
        for (j, hako) in yama.iter().enumerate() {
            if hako == &(target as i32) {
                if j == yama.len() - 1 {
                    println!("{} 0", target);
                }
                else {
                    let tonari: usize = (i + 1) % m;
                    println!("{} {}", yama[j + 1], tonari);
                    b[tonari].extend(&yama[j + 1..yama.len() - 1]);
                    for _ in j + 1..yama.len() - 1 {
                        b[j].pop();
                    }
                }
            }
        }
    }

    println!("{:?}", b);
    b
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; n/m]; m],
    }

/*    let mut stack = Vec::new();
    for _a in &a {
        const v = Vec::from(&_a);
        stack.push(v);
    }
*/
    let mut b = a.clone();
    for target in 1..n {
        b = solve(target, m, b);
    }
}

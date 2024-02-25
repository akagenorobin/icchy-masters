use proconio::input;

fn solve(target: usize, m: usize, a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut b = a.clone();

/*    for (k, c) in b.iter().enumerate() {
        println!("{}: {:?}", k + 1, c);
    }
    println!("-----------------------");
*/
    for (i, yama) in a.iter().enumerate() {
        for (j, hako) in yama.iter().enumerate() {
            if hako == &(target as i32) {
                if j == yama.len() - 1 {
                    println!("{} 0", target);
                    b[i].pop();
                }
                else {
                    let tonari: usize = (i + 1) % m;
                    println!("{} {}", yama[j + 1], tonari + 1);
                    println!("{} 0", target);
                    b[tonari].extend(&yama[j + 1..yama.len()]);
                    for _ in j..yama.len() {
                        b[i].pop();
                    }
                }
            }
        }
    }

    b
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; n/m]; m],
    }

    let mut b = a.clone();
    for target in 1..n+1 {
        b = solve(target, m, b);
    }
}

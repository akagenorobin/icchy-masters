use proconio::input;

fn energy(yama: Vec<i32>) -> i32{
    let mut e = 0;
    for j in 0 .. yama.len() - 1 {
        if yama[j] > yama[j + 1] {
            e -= 1;
        }
    }
    e
}

fn solve(target: usize, a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
                    let mut max_e = -300;
                    let mut target_k = 0;

                    for (k, yama2) in a.iter().enumerate() {
                        if i == k {
                            continue;
                        }
                        let mut y = yama2.clone();
                        y.extend(&yama[j + 1..yama.len()]);
                        let e = energy(y);
                        if e > max_e {
                            max_e = e;
                            target_k = k;
                        }
                    }

                    println!("{} {}", yama[j + 1], target_k + 1);
                    println!("{} 0", target);
                    b[target_k].extend(&yama[j + 1..yama.len()]);
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
        b = solve(target, b);
    }
}

use proconio::input;

/*fn energy(yama: Vec<i32>) -> i32 {
    let mut e = 0;
    for j in 0..yama.len() - 1 {
        if yama[j] > yama[j + 1] {
            e -= 1;
        }
    }
    e
}
*/
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
                } else {
                    for j2 in (j + 1..yama.len()).rev() {
                        let mut min_diff = 200;
                        let mut target_i = 100;
                        for (i2, yama2) in b.iter().enumerate() {
                            if i == i2 {
                                continue;
                            }
                            if yama2.len() == 0{
                                target_i = i2;
                                break;
                            }
                            let diff = yama[j2] - yama2[yama2.len() - 1];
                            if diff < 0 {
                                continue;
                            } else if diff < min_diff {
                                min_diff = diff;
                                target_i = i2;
                            }
                        }
                        if target_i == 100 {
                            target_i = (i + 1) % m;
                        }
                        println!("{} {}", yama[j2], target_i + 1);
                        b[target_i].push(yama[j2]);
                    }

                    println!("{} 0", target);
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
    for target in 1..n + 1 {
        b = solve(target, m, b);
        if target == 207 {
            break;
        }
    }
}

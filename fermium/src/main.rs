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
                    // let tonari: usize = (i + 1) % m;
                    // println!("{} {}", yama[j + 1], tonari + 1);
                    // b[tonari].extend(&yama[j + 1..yama.len()]);
                    // println!("{} 0", target);
                    // for _ in j..yama.len() {
                    //     b[i].pop();
                    // }
                    let mut yama_height = yama.len();

                    for k in (j..yama.len()-1).rev() {
                        let upper = yama[k+1];
                        let lower = yama[k];
                        if upper < lower && k != j {
                            continue;
                        }
                        else {
                            let mut min_height_index = 0;
                            if i == 0 {
                                min_height_index = 1;
                            }
                            for p in 1..m {
                                if p == i {
                                    continue;
                                } else if b[p].len() < b[min_height_index].len() {
                                    min_height_index = p;
                                }
                            }
                            let mut idousaki = 100;
                            if b[min_height_index].len() == 0 {
                                idousaki = min_height_index;
                            } else {
                                let mut flag = true;
                                for l in 0..m {
                                    if l == i {
                                        continue;
                                    } else if flag {
                                        if upper < b[l][b[l].len() - 1] {
                                            idousaki = l;
                                            flag = false;
                                        }
                                    } else {
                                        if upper < b[l][b[l].len() - 1] {
                                            if b[l][b[l].len() - 1] < b[idousaki][b[idousaki].len() - 1] || idousaki == 0 {
                                            // if b[l][b[l].len() - 1] > b[idousaki][b[idousaki].len() - 1] || idousaki == 0 {
                                                idousaki = l;
                                            }
                                        }
                                    }
                                }
                                if flag {
                                    idousaki = min_height_index;
                                }
                            }
                            b[idousaki].extend(&yama[k+1..yama_height]);
                            for _ in k+1..yama_height {
                                b[i].pop();
                            }
                            println!("{} {}", yama[k+1], idousaki+1);
                            yama_height = b[i].len();
                        }
                    }
                    b[i].pop();
                    println!("{} 0", target);
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
        // for i in 0..m{
        //     println!("{:?}",b[i]);
        // }

    }
}

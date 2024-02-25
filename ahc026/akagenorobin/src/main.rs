use proconio::input;
use rand::Rng;

fn move_box(target: usize, m: usize, a: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<String>, usize) {
    let mut ans: Vec<String> = vec![];
    let mut score = 0;
    let mut rng = rand::thread_rng();
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
                    ans.push(format!("{} 0", target));
                    b[i].pop();
                }
                else {
                    let mut target_i = i;
                    while target_i == i {
                        target_i = rng.gen::<usize>() % m;
                        if target_i != i {
                            break;
                        }
                    }

                    ans.push(format!("{} {}", yama[j + 1], target_i + 1));
                    ans.push(format!("{} 0", target));
                    score = yama.len() - j;

                    b[target_i].extend(&yama[j + 1..yama.len()]);
                    for _ in j..yama.len() {
                        b[i].pop();
                    }
                }
            }
        }
    }

    (b, ans, score)
}

fn solve(n: usize, m: usize, a: &Vec<Vec<i32>>) -> (Vec<String>, usize)  {
    let mut b = a.clone();
    let mut an: Vec<String>;
    let mut s;
    let mut ans: Vec<String> = vec![];
    let mut score = 0;
    for target in 1..n+1 {
        (b, an, s) = move_box(target, m, b);
        score += s;
        ans.extend(an);
    }

    (ans, score)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; n/m]; m],
    }

    let mut best_ans = vec![];
    let mut best_score = 10000;

    for _ in 0..10000 {
        let (ans, score) = solve(n, m, &a);
        if score < best_score {
            best_score = score;
            best_ans = ans;
        }
    }

    for step in best_ans {
        println!("{}", step);
    }
}

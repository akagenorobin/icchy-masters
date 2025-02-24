use proconio::input;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn energy(
    n: &usize,
    v: &Vec<Vec<i32>>,
    h: &Vec<Vec<i32>>,
    a: &Vec<Vec<i32>>,
    x: &usize,
    y: &usize,
    value: &i32,
) -> i32 {
    let mut e = 0;

    if *x != 0 && (*v)[*y][(*x) - 1] == 0 {
        e += (*value - (*a)[*y][(*x) - 1]) * (*value - (*a)[*y][(*x) - 1]);
    }
    if *x != (*n) - 1 && (*v)[*y][*x] == 0 {
        e += (*value - (*a)[*y][(*x) + 1]) * (*value - (*a)[*y][(*x) + 1]);
    }
    if *y != 0 && (*h)[(*y) - 1][*x] == 0 {
        e += (*value - (*a)[(*y) - 1][*x]) * (*value - (*a)[(*y) - 1][*x]);
    }
    if *y != (*n) - 1 && (*h)[*y][*x] == 0 {
        e += (*value - (*a)[(*y) + 1][*x]) * (*value - (*a)[(*y) + 1][*x]);
    }

    e
}

fn global_energy(n: &usize, v: &Vec<Vec<i32>>, h: &Vec<Vec<i32>>, a: &Vec<Vec<i32>>) -> i32 {
    let mut e = 0;
    for x in 0..*n {
        for y in 0..*n {
            let value = (*a)[y][x];
            e += energy(n, v, h, a, &x, &y, &value);
        }
    }
    e
}

fn get_can_walk(n: &usize, point: &Point, v: &Vec<Vec<i32>>, h: &Vec<Vec<i32>>)
                -> Vec<(char, Point)> {
    let mut p_next: Vec<(char, Point)> = vec![];

    if point.x != 0 && (*v)[point.y][point.x - 1] == 0 {
        p_next.push((
            'L',
            Point {
                x: point.x - 1,
                y: point.y,
            },
        ));
    }
    if point.x != (*n) - 1 && (*v)[point.y][point.x] == 0 {
        p_next.push((
            'R',
            Point {
                x: point.x + 1,
                y: point.y,
            },
        ));
    }
    if point.y != 0 && (*h)[point.y - 1][point.x] == 0 {
        p_next.push((
            'U',
            Point {
                x: point.x,
                y: point.y - 1,
            },
        ));
    }
    if point.y != (*n) - 1 && (*h)[point.y][point.x] == 0 {
        p_next.push((
            'D',
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ));
    }
    p_next
}

fn find_min_indexes(numbers: &[i32]) -> Option<Vec<usize>> {
    // 最小値を見つけます。
    let min_value = numbers.iter().min()?;

    // 最小値を持つ全てのインデックスを集めます。
    let min_indexes: Vec<usize> = numbers.iter()
        .enumerate()
        .filter(|&(_, &value)| value == *min_value)
        .map(|(index, _)| index)
        .collect();

    Some(min_indexes)
}

fn walk(n: &usize, point_t: &Point, point_a: &Point, v: &Vec<Vec<i32>>, h: &Vec<Vec<i32>>,
        a: &Vec<Vec<i32>>) -> ((char, Point), (char, Point)) {
    let p_next_t = get_can_walk(&n, &point_t, &v, &h);
    let p_next_a = get_can_walk(&n, &point_a, &v, &h);

    // let mut rng = rand::thread_rng();
    // let r_t: usize = rng.gen::<usize>() % p_next_t.len();
    // let r_a: usize = rng.gen::<usize>() % p_next_a.len();
    // (p_next_t[r_t], p_next_a[r_a])

    let mut moves: Vec<((char, Point), (char, Point))> = vec![];
    let mut diffs: Vec<i32> = vec![];

    let current_diff = diff(&n, &v, &h, &a, &point_t, &point_a);
    let mut swap = false;
    if current_diff < 0 {
        swap = true;
    }
    let a_ = &mut a.clone();
    if swap {
        let value_t = a_[point_t.y][point_t.x];
        let value_a = a_[point_a.y][point_a.x];
        a_[point_t.y][point_t.x] = value_a;
        a_[point_a.y][point_a.x] = value_t;
    }
    for (char_t, point_t_next_sim) in p_next_t {
        for (char_a, point_a_next_sim) in &p_next_a {

            let diff_sim = diff(&n, &v, &h, &a_, &point_t_next_sim, &point_a_next_sim);
            moves.push(((char_t, point_t_next_sim), (*char_a, *point_a_next_sim)));
            diffs.push(diff_sim);
        }
    }
    // println!("{:?},{:?}",moves, diffs);

    let min_indexes =  find_min_indexes(&diffs);
    // println!("{:?}",min_indexes);
    let mut rng = rand::thread_rng();
    if let Some(indexes) = min_indexes {
        if diffs[indexes[0]] < 0 {
            let r: usize = rng.gen::<usize>() % indexes.len();
            return moves[indexes[r]]
        } else {
            let r: usize = rng.gen::<usize>() % moves.len();
            return moves[r]
        }
    } else {
        let r: usize = rng.gen::<usize>() % moves.len();
        return moves[r]
    }

}

fn diff(n: &usize, v: &Vec<Vec<i32>>, h: &Vec<Vec<i32>>, a: &Vec<Vec<i32>>, point_t: &Point,
        point_a: &Point) -> i32 {
    let value_t = a[point_t.y][point_t.x];
    let value_a = a[point_a.y][point_a.x];

    energy(&n, &v, &h, &a, &point_a.x, &point_a.y, &value_t)
        + energy(&n, &v, &h, &a, &point_t.x, &point_t.y, &value_a)
        - energy(&n, &v, &h, &a, &point_a.x, &point_a.y, &value_a)
        - energy(&n, &v, &h, &a, &point_t.x, &point_t.y, &value_t)
}

fn update(
    n: &usize,
    v: &Vec<Vec<i32>>,
    h: &Vec<Vec<i32>>,
    a: &Vec<Vec<i32>>,
    point_t: &Point,
    point_a: &Point,
) -> (String, Point, Point, bool) {
    let diff = diff(&n, &v, &h, &a, &point_t, &point_a);

    let (point_t_next, point_a_next) = walk(&n, &point_t, &point_a, &v, &h, &a);

    // println!("diff={}",diff);
    if diff < 0 {
        (
            format!("1 {} {}", point_t_next.0, point_a_next.0),
            point_t_next.1,
            point_a_next.1,
            true
        )
    } else {
        (
            format!("0 {} {}", point_t_next.0, point_a_next.0),
            point_t_next.1,
            point_a_next.1,
            false
        )
    }
}

fn solve(n: usize, v: Vec<Vec<i32>>, h: Vec<Vec<i32>>, a: Vec<Vec<i32>>) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    let mut rng = rand::thread_rng();
    let t_x: usize = rng.gen::<usize>() % n;
    let t_y: usize = rng.gen::<usize>() % n;
    let a_x: usize = rng.gen::<usize>() % n;
    let a_y: usize = rng.gen::<usize>() % n;
    let mut point_t = Point { x: t_x, y: t_y };
    let mut point_a = Point { x: a_x, y: a_y };

    let mut a_ = a.clone();

    ans.push(format!(
        "{} {} {} {}",
        point_t.x, point_t.y, point_a.x, point_a.y
    ));

    for _ in 0..4 * n * n {
        // for _ in 0..20 {
        //     println!("{:?},{:?}",point_t,point_a);
        // println!("{}",global_energy(&n,&v,&h,&a_));
        let (ans_, point_t_next, point_a_next, swapped) = update(&n, &v, &h, &a_, &point_t, &point_a);

        if swapped {
            let value_t = a_[point_t.y][point_t.x];
            let value_a = a_[point_a.y][point_a.x];
            a_[point_t.y][point_t.x] = value_a;
            a_[point_a.y][point_a.x] = value_t;
        }

        point_t = point_t_next;
        point_a = point_a_next;
        ans.push(ans_);
    }

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

    let ans = solve(n, v, h, a);

    for step in ans {
        println!("{}", step);
    }
}

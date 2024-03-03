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

fn walk(n: &usize, point: &Point, v: &Vec<Vec<i32>>, h: &Vec<Vec<i32>>) -> (char, Point) {
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

    let mut rng = rand::thread_rng();
    let r: usize = rng.gen::<usize>() % p_next.len();
    p_next[r]
}

fn update(
    n: &usize,
    v: &Vec<Vec<i32>>,
    h: &Vec<Vec<i32>>,
    a: &Vec<Vec<i32>>,
    point_t: &Point,
    point_a: &Point,
) -> (String, Point, Point, bool) {
    let value_t = a[point_t.x][point_t.y];
    let value_a = a[point_a.x][point_a.y];

    let diff = energy(&n, &v, &h, &a, &point_a.x, &point_a.y, &value_t)
        + energy(&n, &v, &h, &a, &point_t.x, &point_t.y, &value_a)
        - energy(&n, &v, &h, &a, &point_a.x, &point_a.y, &value_a)
        - energy(&n, &v, &h, &a, &point_t.x, &point_t.y, &value_t);

    let point_t_next = walk(&n, &point_t, &v, &h);
    let point_a_next = walk(&n, &point_a, &v, &h);

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
    let mut point_t = Point { x: 0, y: 0 };
    let mut point_a = Point { x: n - 1, y: n - 1 };
    let mut a_ = a.clone();

    ans.push(format!(
        "{} {} {} {}",
        point_t.x, point_t.y, point_a.x, point_a.y
    ));

    for _ in 0..4 * n * n {
        let (ans_, point_t_next, point_a_next, swapped) = update(&n, &v, &h, &a_, &point_t, &point_a);

        if swapped {
            (a_[point_t.y][point_t.x], a_[point_a.y][point_a.x])= (a[point_a.y][point_a.x], a[point_a.y][point_a.x]);
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

use proconio::input;

#[derive(Debug)]
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

fn solve(n: usize, v: Vec<Vec<i32>>, h: Vec<Vec<i32>>, a: Vec<Vec<i32>>) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    let point_t = Point { x: 0, y: 0 };
    let point_a = Point { x: n - 1, y: n - 1 };

    ans.push(format!(
        "{} {} {} {}",
        point_t.x, point_t.y, point_a.x, point_a.y
    ));

    let value_t = a[point_t.x][point_t.y];
    let value_a = a[point_a.x][point_a.y];

    let diff = energy(&n, &v, &h, &a, &point_a.x, &point_a.y, &value_t)
        + energy(&n, &v, &h, &a, &point_t.x, &point_t.y, &value_a)
        - energy(&n, &v, &h, &a, &point_a.x, &point_a.y, &value_a)
        - energy(&n, &v, &h, &a, &point_t.x, &point_t.y, &value_t);

    if diff < 0 {
        ans.push("1 . .".to_string());
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
    /*
        println!("{:?}", v);

        println!("{:?}", h);
        println!("{:?}", a);
    */
    let ans = solve(n, v, h, a);

    for step in ans {
        println!("{}", step);
    }
}

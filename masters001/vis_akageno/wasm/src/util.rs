use proconio::{input, marker::Bytes};
use svg::node::element::{Line, Rectangle, Style, Text};
use svg::node::Text as TextContent;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
pub struct Input {
    pub t: u8,
    pub n: usize,
    pub v: Vec<Vec<u8>>,
    pub h: Vec<Vec<u8>>,
    pub a: Vec<Vec<i32>>,
}

pub struct Output {
    pub initial: (usize, usize, usize, usize),
    pub moves: Vec<(u8, String, String)>,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct State {
    pub a: Vec<Vec<i32>>,
    pub aoki: Point,
    pub takahashi: Point,
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);

    input! {
        from f,
        t: u8,
        n: usize,
        v: [Bytes; n],
        h: [Bytes; n - 1],
        a: [[i32; n]; n],
    }
    Input { t, n, v, h, a }
}

pub fn parse_output(f: &str, n: &usize) -> Output {
    let f = proconio::source::once::OnceSource::from(f);

    input! {
        from f,
        initial: (usize, usize, usize, usize),
        moves: [(u8, String, String); 4 * n * n],
    }
    Output { initial, moves }
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn text(x: usize, y: usize, txt: &str) -> Text {
    Text::new()
        .add(TextContent::new(txt))
        .set("x", x)
        .set("y", y)
        .set("fill", "black")
        .set("font-size", 10)
        .set("dominant-baseline", "central")
        .set("text-anchor", "middle")
}

pub fn line(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", "#000000")
        .set("stroke-width", 7)
}

pub fn energy(
    n: &usize,
    v: &Vec<Vec<u8>>,
    h: &Vec<Vec<u8>>,
    a: &Vec<Vec<i32>>,
    x: &usize,
    y: &usize,
    value: &i32,
) -> i32 {
    let mut e = 0;

    if *x != 0 && (*v)[*y][(*x) - 1] == b'0' {
        e += (*value - (*a)[*y][(*x) - 1]) * (*value - (*a)[*y][(*x) - 1]);
    }
    if *x != (*n) - 1 && (*v)[*y][*x] == b'0' {
        e += (*value - (*a)[*y][(*x) + 1]) * (*value - (*a)[*y][(*x) + 1]);
    }
    if *y != 0 && (*h)[(*y) - 1][*x] == b'0' {
        e += (*value - (*a)[(*y) - 1][*x]) * (*value - (*a)[(*y) - 1][*x]);
    }
    if *y != (*n) - 1 && (*h)[*y][*x] == b'0' {
        e += (*value - (*a)[(*y) + 1][*x]) * (*value - (*a)[(*y) + 1][*x]);
    }

    e
}

pub fn global_energy(n: &usize, v: &Vec<Vec<u8>>, h: &Vec<Vec<u8>>, a: &Vec<Vec<i32>>) -> i32 {
    let mut e = 0;
    for x in 0..*n {
        for y in 0..*n {
            let value = (*a)[y][x];
            e += energy(n, v, h, a, &x, &y, &value);
        }
    }
    e / 2
}

pub fn calc(input: &Input, output: &Output, turn: usize) -> (State, i64) {
    let mut state = State {
        a: input.a.clone(),
        aoki: Point {
            x: output.initial.0,
            y: output.initial.1,
        },
        takahashi: Point {
            x: output.initial.2,
            y: output.initial.3,
        },
    };

    let e_init = global_energy(&input.n, &input.v, &input.h, &state.a);

    for i in 0..turn {
        if output.moves[i].0 == 1 {
            let value_a = state.a[state.aoki.y][state.aoki.x];
            let value_t = state.a[state.takahashi.y][state.takahashi.x];
            state.a[state.aoki.y][state.aoki.x] = value_t;
            state.a[state.takahashi.y][state.takahashi.x] = value_a;
        }

        match output.moves[i].1.as_str() {
            "R" => state.aoki.x += 1,
            "L" => state.aoki.x -= 1,
            "D" => state.aoki.y += 1,
            "U" => state.aoki.y -= 1,
            _ => (),
        }

        match output.moves[i].2.as_str() {
            "R" => state.takahashi.x += 1,
            "L" => state.takahashi.x -= 1,
            "D" => state.takahashi.y += 1,
            "U" => state.takahashi.y -= 1,
            _ => (),
        }
    }

    let e_final = global_energy(&input.n, &input.v, &input.h, &state.a);

    (
        state,
        (1000000.0 * (e_init as f64 / e_final as f64).log2()) as i64,
    )
}

fn generate_color(code: usize, n: usize) -> String {
    format!(
        "#{:02X}{:02X}ff",
        (255.0 - code as f64 / n as f64 / n as f64 * 255.0) as u8,
        (255.0 - code as f64 / n as f64 / n as f64 * 128.0) as u8,
    )
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let (state, score) = calc(input, output, turn);

    let width = 800;
    let height = 800;
    let w = width / input.n;
    let h = height / input.n;

    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, width + 10, height + 10))
        .set("width", width + 10)
        .set("height", height + 10)
        .set("style", "background-color:white");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));

    for y in 0..input.n {
        for x in 0..input.n {
            doc = doc.add(rect(
                x * w,
                y * h,
                w,
                h,
                &generate_color(state.a[y][x] as usize, input.n),
            ));

            if x == state.aoki.x && y == state.aoki.y {
                doc = doc.add(rect(x * w, y * h, w, h, "#ff2000"));
            }

            if x == state.takahashi.x && y == state.takahashi.y {
                doc = doc.add(rect(x * w, y * h, w, h, "#20ff00"));
            }

            if x != input.n - 1 && input.v[y][x] == b'1' {
                doc = doc.add(line((x + 1) * w, y * h, (x + 1) * w, (y + 1) * h))
            }

            if y != input.n - 1 && input.h[y][x] == b'1' {
                doc = doc.add(line(x * w, (y + 1) * h, (x + 1) * w, (y + 1) * h))
            }

            doc = doc.add(text(
                x * w + w / 2,
                y * h + h / 2,
                &state.a[y][x].to_string(),
            ));
        }
    }

    (score, "".to_string(), doc.to_string())
}

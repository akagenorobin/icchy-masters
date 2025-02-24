use proconio::input;
use proconio::source::once::OnceSource;
use std::io::Read;

#[derive(Debug)]
struct InputData {
    num_destinations: usize,
    num_walls: usize,
    param1: f64,
    param2: f64,
    initial_position: (i32, i32),
    destinations: Vec<(i32, i32)>,
    alpha: Vec<f64>,
    wind_params: Vec<(i32, i32)>,
}

fn main() {
    // 標準入力全体を文字列として読み込む
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let source = OnceSource::new(std::io::BufReader::new(buf.as_bytes()));
    
    // 入力を読み込む
    input! {
        from source,
        num_destinations: usize,
        num_walls: usize,
        param1: f64,
        param2: f64,
        init_x: i32,
        init_y: i32,
        destinations: [(i32, i32); num_destinations],
        alpha: [f64; 5000],
        wind_params: [(i32, i32); 5000],
    }
    
    let input_data = InputData {
        num_destinations,
        num_walls,
        param1,
        param2,
        initial_position: (init_x, init_y),
        destinations,
        alpha,
        wind_params,
    };
    
    // 読み込んだ内容の確認
    println!("{:#?}", input_data);
}
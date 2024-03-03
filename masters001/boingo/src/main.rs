use std::fmt::format;
use proconio::input;
use colored::*;
use std::io::{self, stdin};

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

fn main() {
    println!("Inputを入力してください") ;
    let( t,n, v, h, a ) = read_input();

    println!("Outputを入力してください") ;
    let lines = read_output();
    let numbers: Vec<usize> = lines[0]
        .split_whitespace() // 空白で文字列を分割
        .filter_map(|s| s.parse().ok()) // 各文字列をusizeにパースし、結果がOkなら値を取得
        .collect(); // 結果をVec<usize>にまとめる

    let takahashi_first = Point { x: numbers[0], y: numbers[1] };
    let aoki_first = Point { x: numbers[2], y: numbers[3] };

    let walks: &Vec<(bool, Direction, Direction)> = &lines[1..]
        .iter().filter_map( |line| Some(translace(line)) )
        .collect();

     // TODO: energyかく。
    visualize_with_person( n, v.clone(), h.clone(), a.clone() , takahashi_first.clone(), aoki_first.clone());


}

fn visualize_with_person( n: usize, v : Vec<Vec<i32>>, h : Vec<Vec<i32>>, a : Vec<Vec<i32>>, takahashi: Point, aoki: Point ) {

    let digit = get_digit_count(n*n ) ;

    // マス目と壁を出力
    for i in 0..n {
        for j in 0..n {
            if  j == 0 {print!(" ") ;}
            let format = format!("{:digit$} ", a[i][j]) ;
            if ( takahashi.x == i, takahashi.y == j ) == ( true, true ) {
                print!("{}", format.bright_blue().bold());
            } else if (aoki.x == i, aoki.y == j ) == ( true, true ) {
                print!("{}", format.red().bold());
            } else {
                print!("{}", format);
            }

            if  j == n - 1  {continue;}
            if v[i][j] == 1 {
                print!("|"); // 右側の壁を表す
            } else {
                print!(" "); // 壁がない場合は空白
            }
        }
        println!();

        if  i == n - 1  {continue;}
        // 下側の壁を出力
        for j in 0..n {
            print!(" ");
            if h[i][j] == 1 {
                // 壁がある場合
                for k in 0..digit {
                    print!("-");
                }
            } else {
                for k in 0..digit {
                    print!(" ");
                }
            }
            print!(" ");
        }
        println!();
    }
}

fn get_digit_count(num: usize) -> usize {
    // 整数を文字列に変換し、その長さを返す
    num.to_string().len()
}

fn read_input() -> ( usize, usize, Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>){
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

    ( _t, n, v, h, a )
}

fn read_output() -> Vec<String> {
    let mut lines = Vec::new();

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");

        // Check if the line is empty (contains only a newline character)
        if line.trim().is_empty() {
            break;
        }

        // Add the line to the vector, removing the trailing newline
        lines.push(line.trim_end().to_string());
    }

    lines
}

fn translace( line: &String ) -> (bool, Direction, Direction) {
    let ss : Vec<String> = line.split_whitespace()
        .filter_map(|s| s.parse().ok()) // 各文字列をusizeにパースし、結果がOkなら値を取得
        .collect(); // 結果をVec<usize>にまとめる

    (
        if ss[0] == "0" { false} else if ss[0] == "1" { true} else { panic!()},
        seek_direction(&ss[1]),
        seek_direction(&ss[2])
    )
}

fn seek_direction( text: &String ) -> Direction {
    if text == "U" {
        return Direction::Up
    } else if text == "D" {
        return Direction::Down
    } else if text == "L" {
        return Direction::Left
    } else if text == "R" {
        return Direction::Right
    } else if text == "."{
        return Direction::Stop
    } else {
        panic!()
    }
}
use proconio::input_interactive;

#[derive(Clone, Debug)]
pub struct Input {
    pub eps: f64,
    pub delta: f64,
    pub init: (i64, i64),
    pub dests: Vec<(i64, i64)>,
    pub walls: Vec<(i64, i64, i64, i64)>,
}

#[derive(Clone, Debug)]
pub struct Drone {
    pub x: i64,
    pub y: i64,
    pub vx: i64,
    pub vy: i64,
}

impl Drone {
    pub fn new(x: i64, y: i64, vx: i64, vy: i64) -> Drone {
        Drone { x, y, vx, vy }
    }

    pub fn update(&mut self, ax: i64, ay: i64) {
        self.vx += ax;
        self.vy += ay;
        self.x += self.vx;
        self.y += self.vy;
    }

    pub fn collide(&mut self) {
        self.vx = 0;
        self.vy = 0;
    }
}

fn find_nearest(x: i64, y: i64, dests: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut min = 10000000.0;
    let mut min_dest = dests[0].clone();
    for dest in dests {
        let d = (((x - dest.0) * (x - dest.0) + (y - dest.1) * (y - dest.1)) as f64).sqrt();
        if d < min {
            min = d;
            min_dest = dest.clone();
        }
    }

    min_dest
}

fn input() -> Input {
    input_interactive! {
        n: usize,
        m: usize,
        eps: f64,
        delta: f64,
        init: (i64, i64),
        dests: [(i64, i64); n],
        walls: [(i64, i64, i64, i64); m],
    }
    Input {
        eps,
        delta,
        init,
        dests,
        walls,
    }
}

fn solve(input: &Input) {
    let mut drone = Drone::new(input.init.0, input.init.1, 0, 0);

    let mut visited = vec![false; input.dests.len()];
    let mut dest = find_nearest(drone.x, drone.y, &input.dests);

    for _ in 0..5000 {
        let d =
            (((dest.0 - drone.x) * (dest.0 - drone.x) + (dest.1 - drone.y) * (dest.1 - drone.y))
                as f64)
                .sqrt();
        let ax = (499.0 * (dest.0 - drone.x) as f64 / d) as i64;
        let ay = (499.0 * (dest.1 - drone.y) as f64 / d) as i64;

        println!("A {} {}", ax, ay);

        input_interactive! {
            c: u64,
            h: u64,
            q: [u64; h],
        }

        if h > 0 {
            for v in q {
                visited[v as usize] = true;
            }
        }

        if c == 1 {
            if h > 0 {
                let dests = input
                    .dests
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| !visited[*idx])
                    .map(|(_, d)| d.to_owned())
                    .collect::<Vec<_>>();
                dest = find_nearest(drone.x, drone.y, &dests);
            } else {
                dest = find_nearest(drone.x, drone.y, &input.dests);
            }
            drone.collide();
        } else {
            drone.update(ax, ay);
        }
    }
}

fn main() {
    let input = input();

    solve(&input);
}

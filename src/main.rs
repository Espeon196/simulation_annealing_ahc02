

use proconio;
use once_cell::sync::Lazy;
use std::sync::Mutex;

const H: i64 = 50;
const W: i64 = 50;
const TILE_NUM: i64 = H * W;

use lazy_static::lazy_static;

/*
macro_rules! interactive_input(($($tt:tt)*) => (
    let stdin = std::io::stdin();
    let mut stdin = proconio::source::line::LineSource::new(stdin.lock());
    proconio::input!(from &mut stdin, $($tt)*);
));
*/
use proconio::input;

lazy_static! {
    static ref TILES: Vec<i64> = {
        input! {tiles: [i64; TILE_NUM as usize]}
        tiles
    };
    static ref POINTS: Vec<i64> = {
        input! {points: [i64; TILE_NUM as usize]}
        points
    };
}

static NEXT_COORDS: Lazy<Mutex<Vec<Vec<i64>>>> = Lazy::new(|| {
    Mutex::new(vec![vec![]; TILE_NUM as usize])
});

struct State {
    path: Vec<i64>
}

impl State {
    fn output(&self) -> String {
        let mut res = String::new();
        for i in 0..self.path.len()-1 {
            let diff = self.path[i+1] - self.path[i];
            match diff {
                1 => res.push('R'),
                -1 => res.push('L'),
                d if d == W => res.push('D'),
                d if d == -W => res.push('U'),
                _ => panic!("Error!"),
            }
        }
        res
    }
}

fn initialize_global_info() {
    const DY: [i64; 4] = [-1, 1, 0, 0];
    const DX: [i64; 4] = [0, 0, 1, -1];
    for y in 0..H {
        for x in 0..W {
            let coord = y * W + x;
            for d in 0..4usize {
                let ty = y + DY[d];
                let tx = x + DX[d];
                if (0..H).contains(&ty) && (0..W).contains(&tx) {
                    let tcoord = ty * W + tx;
                    if TILES[coord as usize] != TILES[tcoord as usize] {
                        NEXT_COORDS.lock().unwrap()[coord as usize].push(tcoord);
                    }
                }
            }
        }
    }
}

fn main() {
    input! {
        si: usize,
        sj: usize,
    };

    lazy_static::initialize(&TILES);
    lazy_static::initialize(&POINTS);

    println!("{} {}", TILES[0], POINTS[0]);
}

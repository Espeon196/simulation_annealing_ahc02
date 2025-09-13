

use proconio;
use once_cell::sync::Lazy;
use std::sync::Mutex;

const H: usize = 50;
const W: usize = 50;
const TILE_NUM: usize = H * W;

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
    static ref NEXT_COORDS: Vec<Vec<usize>> = initialize_global_info();
}


fn initialize_global_info() -> Vec<Vec<usize>> {
    const MOVE: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)];
    let mut next_coords = vec![vec![]; TILE_NUM];
    eprint!("initialize global info\n");

    for y in 0..H {
        for x in 0..W {
            let coord = y * W + x;
            for &(dy, dx) in &MOVE {
                let ty = y.wrapping_add(dy);
                let tx = x.wrapping_add(dx);
                if (0..H).contains(&ty) && (0..W).contains(&tx) {
                    let tcoord = ty * W + tx;
                    if TILES[coord] != TILES[tcoord] {
                        next_coords[coord].push(tcoord);
                    }
                }
            }
        }
    }
    next_coords
}

struct State {
    path: Vec<usize>
}

impl State {
    fn output(&self) -> String {
        let mut res = String::new();
        for i in 0..self.path.len()-1 {
            if self.path[i+1] == self.path[i] + 1 {
                res.push('R');
            } else if self.path[i+1] + 1 == self.path[i] {
                res.push('L');
            } else if self.path[i+1] == self.path[i] + W {
                res.push('D');
            } else if self.path[i+1] + W == self.path[i] {
                res.push('U');
            } 
        }
        res
    }
}

fn main() {
    input! {
        si: usize,
        sj: usize,
    };

    lazy_static::initialize(&TILES);
    lazy_static::initialize(&POINTS);
    lazy_static::initialize(&NEXT_COORDS);

    println!("{} {}", TILES[0], POINTS[0]);
}

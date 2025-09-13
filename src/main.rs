

use proconio;

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
    static ref TILES: Vec<usize> = {
        input! {tiles: [usize; TILE_NUM as usize]}
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

#[derive(Clone)]
struct State {
    path: Vec<usize>
}

impl State {
    fn new() -> Self {
        Self {
            path: Vec::new(),
        }
    }

    fn push(&mut self, coord: usize) {
        self.path.push(coord);
    }

    fn pop(&mut self) {
        self.path.pop();
    }

    fn clear(&mut self) {
        self.path.clear();
    }

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

struct DfsSolver {
    visiteds: [bool; TILE_NUM],
    path: State,
    best_path: State,
    score: i64,
    best_score: i64,
    remaining_search_count: i64,
}

impl DfsSolver {
    fn new() -> Self {
        Self {
            visiteds: [false; TILE_NUM],
            path: State::new(),
            best_path: State::new(),
            score: 0,
            best_score: 0,
            remaining_search_count: 0,
        }
    }

    fn start(&mut self, first_coord: usize) {
        self.best_path.clear();
        self.score = 0;
        self.visiteds.fill(false);
        self.path.clear();
        self.remaining_search_count = 40000;
        self.dfs(first_coord);
    }

    fn dfs(&mut self, coord: usize) {
        self.path.push(coord);
        self.score += POINTS[coord];
        self.visiteds[TILES[coord]] = true;
        if self.best_score < self.score {
            self.best_score = self.score;
            self.best_path = self.path.clone();
        }
        self.remaining_search_count -= 1;
        if self.remaining_search_count <= 0 {return;}

        // eprintln!("now: {}", self.path.output());
        // eprintln!("now_coord: {coord}, tile: {}", TILES[coord]);

        for &next_coord in &NEXT_COORDS[coord] {
            // eprintln!("\tnext_coord: {next_coord}, tile: {}", TILES[next_coord]);
            if self.visiteds[TILES[next_coord]] {continue;}
            // eprintln!("\t go");
            self.dfs(next_coord);
            if self.remaining_search_count <= 0 {return;}
        }

        self.path.pop();
        self.score -= POINTS[coord];
        self.visiteds[TILES[coord]] = false;
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

    let first_coord = si * W + sj;
    let mut dfs_solver = DfsSolver::new();
    dfs_solver.start(first_coord);
    let ans = dfs_solver.best_path.output();
    println!("{}", ans);
}

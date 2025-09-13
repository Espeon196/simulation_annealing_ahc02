
#[allow(unused)]
mod time_keeper {
    use std::time::{Instant, Duration};

    pub struct TimeKeeper {
        start_time: Instant,
        time_threshold: Duration,
    }

    impl TimeKeeper {
        /// 制限時間を指定してTimeKeeperを作成する
        /// * `time_threshold` - 時間制限(msec)
        pub fn new(time_threshold: u64) -> Self {
            Self {
                start_time: Instant::now(),
                time_threshold: Duration::from_millis(time_threshold),
            }
        }

        /// 制限時間を超過したか判定
        pub fn is_time_over(&self) -> bool {
            let now = Instant::now();
            now - self.start_time >= self.time_threshold
        }
    }
}

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
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use once_cell::sync::Lazy;
use std::sync::Mutex;
const SEED: u64 = 0;

static RAND_GEN: Lazy<Mutex<StdRng>> = Lazy::new(|| {
    Mutex::new(StdRng::seed_from_u64(80))
});


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
        self.remaining_search_count = 4000;
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

struct DfsPartSolver<'a> {
    visiteds: &'a mut [bool; TILE_NUM],
    path: State,
    best_path: State,
    score: i64,
    best_score: i64,
    remaining_search_count: i64,
    goal: usize,
}

impl<'a> DfsPartSolver<'a> {
    fn new(visited: &'a mut [bool; TILE_NUM]) -> Self {
        Self { visiteds: visited, path: State::new(), best_path: State::new(), score: 0, best_score: 0, remaining_search_count: 0, goal: 0 }
    }

    fn start(&mut self, start: usize, goal: usize,  remaining_search_count: i64) {
        self.goal = goal;
        self.best_path.clear();
        self.best_score = 0;
        self.score = 0;
        self.path.clear();
        self.remaining_search_count = remaining_search_count;
        self.dfs(start);
    }

    fn dfs(&mut self, coord: usize) {
        if !self.visiteds[TILES[coord]] {
            self.path.push(coord);
            self.score += POINTS[coord];
            self.visiteds[TILES[coord]] = true;
        }
        self.remaining_search_count -= 1;
        if self.remaining_search_count <= 0 {
            return;
        }

        let mut legal_next_coords = Vec::<usize>::new();
        for &next_coord in &NEXT_COORDS[coord] {
            if !self.visiteds[TILES[next_coord]] {
                legal_next_coords.push(next_coord);
            } else if next_coord == self.goal {
                self.best_score = self.score;
                self.best_path = self.path.clone();
                self.remaining_search_count = 0;
                return;
            }
        }
        let mut rng = rand::thread_rng();
        legal_next_coords.shuffle(&mut rng);
        for &next_coord in &legal_next_coords {
            if self.visiteds[TILES[next_coord]] {continue;}
            self.dfs(next_coord);
            if self.remaining_search_count <= 0 {return;}
        }

        self.path.pop();
        self.score -= POINTS[coord];
        self.visiteds[TILES[coord]] = false;
    }
}

fn hill_climb_with_time_threshold(time_threshold: u64, first_coord: usize) -> State {
    let time_keeper = time_keeper::TimeKeeper::new(time_threshold);

    let mut dfs_solver = DfsSolver::new();
    dfs_solver.start(first_coord);

    let mut now_path = dfs_solver.best_path;
    let mut now_visited = [false; TILE_NUM];
    for &coord in &now_path.path {
        now_visited[TILES[coord]] = true;
    }

    loop {
        if time_keeper.is_time_over() {break;}

        let mut rng = RAND_GEN.lock().unwrap();
        let ub_len = (now_path.path.len() as f64 * 0.05) as usize;
        let delete_path_length = rng.gen_range(1..ub_len);
        let start_path_id = rng.gen_range(0..(now_path.path.len() - delete_path_length));
        let end_path_id = start_path_id + delete_path_length;

        let mut next_visited = now_visited.clone();
        let remaining_search_count= 4 * delete_path_length as i64;

        let mut now_score = 0i64;
        for &coord in &now_path.path[(start_path_id+1)..end_path_id] {
            now_score += POINTS[coord];
            next_visited[TILES[coord]] = false;
        }
        let mut dfs_part_solver = DfsPartSolver::new(&mut next_visited);
        dfs_part_solver.start(now_path.path[start_path_id], now_path.path[end_path_id], remaining_search_count);

        let next_score = dfs_part_solver.best_score;
        let diff = next_score - now_score;
        if dfs_part_solver.best_path.path.len() > 0 && diff > 0 {
            now_visited = *dfs_part_solver.visiteds;

            let mut transitioned_path = Vec::<usize>::new();
            for &coord in &now_path.path[0..=start_path_id] {
                transitioned_path.push(coord);
            }
            for &coord in &dfs_part_solver.best_path.path {
                transitioned_path.push(coord);
            }
            for &coord in &now_path.path[end_path_id..] {
                transitioned_path.push(coord);
            }
            now_path.path = transitioned_path;
        }
    }
    now_path
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
    let ans_path = hill_climb_with_time_threshold(1950, first_coord);
    println!("{}", ans_path.output());
}

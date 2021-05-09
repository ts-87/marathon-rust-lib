#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::io::Read;

macro_rules! input {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read!($iter, $t);
        input!{$iter $($r)*}
    };

    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read!($iter, $t);
        input!{$iter $($r)*}
    };
}

macro_rules! read {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

struct State {
    sy: usize,
    sx: usize,
    t: Vec<Vec<usize>>,
    p: Vec<Vec<usize>>,
    used: Vec<bool>,
    dir: usize,
    rnd: XorShift32,
    timer: Timer,
}

const H: usize = 50;
const W: usize = 50;
const DY: [usize; 4] = [0, 1, 0, !0];
const DX: [usize; 4] = [1, 0, !0, 0];
const DIR: [char; 4] = ['R', 'D', 'L', 'U'];

fn dfs(s: &mut State, ny: usize, nx: usize, depth: usize, score: usize) -> usize {
    if depth == 10 {
        return score + s.p[ny][nx];
    }
    s.used[s.t[ny][nx]] = true;
    let mut ret = score;
    for _ in 0..4 {
        let d = s.rnd.nextn(4);
        let (nny, nnx) = (ny + DY[d], nx + DX[d]);
        if nny < H && nnx < W && !s.used[s.t[nny][nnx]] {
            ret = ret.max(dfs(s, nny, nnx, depth + 1, score + s.p[ny][nx]));
        }
    }
    s.used[s.t[ny][nx]] = false;
    
    ret
}

fn dfs2(s: &mut State, ny: usize, nx: usize, depth: usize) -> usize {
    if depth == 50 {
        return 1;
    }
    s.used[s.t[ny][nx]] = true;
    let mut ret = 0;
    for d in 0..4 {
        let (nny, nnx) = (ny + DY[d], nx + DX[d]);
        if nny < H && nnx < W && !s.used[s.t[nny][nnx]] {
            ret += dfs2(s, nny, nnx, depth + 1);
        }
        if ret > 0 {
            break;
        }
    }
    s.used[s.t[ny][nx]] = false;
    
    ret
}

fn main() {
    let timer = Timer::new();

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    input! {
        input,
        sy: usize,
        sx: usize,
        t: [[usize; W]; H],
        p: [[usize; W]; H],
    }

    //let mut state = State {sy: sy, sx: sx, t: t, p: p, used: vec![false; 2500], dir: 0, rnd: XorShift32::new(0), timer: timer,};
    //let (mut py, mut px) = (sy, sx);
    let mut bestscore = 0;
    let mut bestans = String::new();

    for dd in 0..10 {
        let mut state = State {sy: sy, sx: sx, t: t.clone(), p: p.clone(), used: vec![false; 2500], dir: 0, rnd: XorShift32::new(0), timer: Timer::new(),};
        let (mut py, mut px) = (sy, sx);
        let mut score = 0;
        let mut ans = String::new();
        loop {
            score += state.p[py][px];
            state.used[state.t[py][px]] = true;
            let (mut ny, mut nx) = (H, W);
            let mut nxdir = 0;
            let mut mx = 0;
            let sel = state.rnd.nextn(100);
            for d in 0..4 {
                let td = if sel < 7 && dd > 3 {state.rnd.nextn(4)} else {(d + dd) & 3};
                if py + DY[td] < H && px + DX[td] < W && !state.used[state.t[py + DY[td]][px + DX[td]]] {
                    let ret = dfs2(&mut state, py + DY[td], px + DX[td], 0);
                    if ret > mx {    
                        ny = py + DY[td];
                        nx = px + DX[td];
                        nxdir = td;
                        mx = ret;
                    }
                    if mx > 0 {
                        break;
                    }
                }
            }
            if mx == 0 {
                for _ in 0..50 {
                    let td = state.rnd.nextn(4);
                    //let td = (d + state.dir + 3) & 3;
                    if py + DY[td] < H && px + DX[td] < W && !state.used[state.t[py + DY[td]][px + DX[td]]] {
                        let ret = dfs(&mut state, py + DY[td], px + DX[td], 0, 0);
                        if ret > mx {    
                            ny = py + DY[td];
                            nx = px + DX[td];
                            nxdir = td;
                            mx = ret;
                        }
                    }
                }
            }
            if ny >= H {
                break;
            }
            ans.push(DIR[nxdir]);
            state.dir = nxdir;
            py = ny;
            px = nx;
        }
        
        if bestscore < score {
            bestans = ans.clone();
            bestscore = score;
        }
    }
    for dd in 0..10 {
        let mut state = State {sy: sy, sx: sx, t: t.clone(), p: p.clone(), used: vec![false; 2500], dir: 0, rnd: XorShift32::new(0), timer: Timer::new(),};
        let (mut py, mut px) = (sy, sx);
        let mut score = 0;
        let mut ans = String::new();
        loop {
            score += state.p[py][px];
            state.used[state.t[py][px]] = true;
            let (mut ny, mut nx) = (H, W);
            let mut nxdir = 0;
            let mut mx = 0;
            let sel = state.rnd.nextn(100);
            for d in 0..4 {
                let td = if sel < 7 && dd > 3 {state.rnd.nextn(4)} else {(d + state.dir + dd) & 3};
                if py + DY[td] < H && px + DX[td] < W && !state.used[state.t[py + DY[td]][px + DX[td]]] {
                    let ret = dfs2(&mut state, py + DY[td], px + DX[td], 0);
                    if ret > mx {    
                        ny = py + DY[td];
                        nx = px + DX[td];
                        nxdir = td;
                        mx = ret;
                    }
                    if mx > 0 {
                        break;
                    }
                }
            }
            if mx == 0 {
                for _ in 0..50 {
                    let td = state.rnd.nextn(4);
                    //let td = (d + state.dir + 3) & 3;
                    if py + DY[td] < H && px + DX[td] < W && !state.used[state.t[py + DY[td]][px + DX[td]]] {
                        let ret = dfs(&mut state, py + DY[td], px + DX[td], 0, 0);
                        if ret > mx {    
                            ny = py + DY[td];
                            nx = px + DX[td];
                            nxdir = td;
                            mx = ret;
                        }
                    }
                }
            }
            if ny >= H {
                break;
            }
            ans.push(DIR[nxdir]);
            state.dir = nxdir;
            py = ny;
            px = nx;
        }
        
        if bestscore < score {
            bestans = ans.clone();
            bestscore = score;
        }
    }
    for dd in 0..4 {
        let mut state = State {sy: sy, sx: sx, t: t.clone(), p: p.clone(), used: vec![false; 2500], dir: 0, rnd: XorShift32::new(0), timer: Timer::new(),};
        let (mut py, mut px) = (sy, sx);
        let mut score = 0;
        let mut ans = String::new();
        loop {
            score += state.p[py][px];
            state.used[state.t[py][px]] = true;
            let (mut ny, mut nx) = (H, W);
            let mut nxdir = 0;
            let mut mx = 0;
            let sel = state.rnd.nextn(16);
            for d in 0..4 {
                let td = if sel > 0 {(d + dd) & 3} else {(d + state.dir + dd) & 3};
                if py + DY[td] < H && px + DX[td] < W && !state.used[state.t[py + DY[td]][px + DX[td]]] {
                    let ret = dfs2(&mut state, py + DY[td], px + DX[td], 0);
                    if ret > mx {    
                        ny = py + DY[td];
                        nx = px + DX[td];
                        nxdir = td;
                        mx = ret;
                    }
                    if mx > 0 {
                        break;
                    }
                }
            }
            if mx == 0 {
                for _ in 0..50 {
                    let td = state.rnd.nextn(4);
                    //let td = (d + state.dir + 3) & 3;
                    if py + DY[td] < H && px + DX[td] < W && !state.used[state.t[py + DY[td]][px + DX[td]]] {
                        let ret = dfs(&mut state, py + DY[td], px + DX[td], 0, 0);
                        if ret > mx {    
                            ny = py + DY[td];
                            nx = px + DX[td];
                            nxdir = td;
                            mx = ret;
                        }
                    }
                }
            }
            if ny >= H {
                break;
            }
            ans.push(DIR[nxdir]);
            state.dir = nxdir;
            py = ny;
            px = nx;
        }
        
        if bestscore < score {
            bestans = ans.clone();
            bestscore = score;
        }
    }
    eprintln!("{}", bestscore);
    println!("{}", bestans);
}


pub struct Timer {
    start_time: f64
}
pub fn get_time_sec() -> f64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

impl Timer {
    pub fn new() -> Timer {
        Timer {start_time: get_time_sec()}
    }

    pub fn get_time(&self) -> f64 {
        get_time_sec() - self.start_time
    }
}

pub struct XorShift32{
    pub y: u32
}

const INV32: f64 = 1.0 / std::u32::MAX as f64;
impl XorShift32 {
    pub fn new(seed:u32) -> Self {
        Self {y: seed ^ 2463534242}
    }

    #[inline]
    pub fn next_int(&mut self) -> usize {
        self.y ^= self.y << 13;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 5;
        self.y as usize
    }

    #[inline]
    pub fn nextn(&mut self, n: usize) -> usize {
        self.next_int() % n
    }

    #[inline]
    pub fn next_double(&mut self) -> f64 {
        self.next_int() as f64 * INV32
    }
}


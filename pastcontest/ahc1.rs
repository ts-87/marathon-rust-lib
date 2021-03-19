#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{io::Read, vec};

use num_integer::Roots;
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

const R: i32 = 10000;
const TIMELIMIT: f64 = 4.95;
const INF: f64 = 1e15;

#[derive(Clone, Copy)]
struct Ad {
    pos: (i32, i32),
    r: i32,
    index: usize,
    lx: i32,
    rx: i32,
    ly: i32,
    ry: i32,
}

impl Ad {
    pub fn size(&self) -> i32 {
		(self.rx - self.lx) * (self.ry - self.ly)
	}
    pub fn score(&self) -> f64 {
        let s = self.size().min(self.r) as f64 / self.size().max(self.r) as f64;
        1. - (1. - s) * (1. - s)
    }
}

fn intersect(r1: &Ad, r2: &Ad) -> bool {
	r1.rx.min(r2.rx) > r1.lx.max(r2.lx) && r1.ry.min(r2.ry) > r1.ly.max(r2.ly)
}
fn intersect2(lx: i32, rx: i32, ly: i32, ry: i32, r2: &Ad) -> bool {
	rx.min(r2.rx) > lx.max(r2.lx) && ry.min(r2.ry) > ly.max(r2.ly)
}

fn output_answer (ads: &Vec<Ad>) {
    let mut ans = vec![(0,0,0,0); ads.len()];
    for ad in ads.iter() {
        ans[ad.index] = (ad.lx, ad.ly, ad.rx, ad.ry);
    }
    for a in ans {
        print!("{} {} {} {}\n", a.0, a.1, a.2, a.3);
    }
}

fn extend_rect (ads: &mut Vec<Ad>, id: usize) {
    let (mut lx, mut rx, mut ly, mut ry) = (ads[id].lx, ads[id].rx, ads[id].ly, ads[id].ry);
    let (mut l, mut r) = (-1, lx);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut ok = true;
        for (i, ad) in ads.iter().enumerate() {
            if i == id {continue;}
            if ads[id].r < (rx - m) * (ry - ly) {ok = false;break;}
            if intersect2(m, rx, ly, ry, ad) {ok = false;break;}
        }
        if ok {r = m;} else {l = m;}
    }
    lx = r;
    let (mut l, mut r) = (rx, R+1);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut ok = true;
        for (i, ad) in ads.iter().enumerate() {
            if i == id {continue;}
            if ads[id].r < (m - lx) * (ry - ly) {ok = false;break;}
            if intersect2(lx, m, ly, ry, ad) {ok = false;break;}
        }
        if ok {l = m;} else {r = m;}
    }
    rx = l;
    let (mut l, mut r) = (-1, ly);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut ok = true;
        for (i, ad) in ads.iter().enumerate() {
            if i == id {continue;}
            if ads[id].r < (rx - lx) * (ry - m) {ok = false;break;}
            if intersect2(lx, rx, m, ry, ad) {ok = false;break;}
        }
        if ok {r = m;} else {l = m;}
    }
    ly = r;
    let (mut l, mut r) = (ry, R+1);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut ok = true;
        for (i, ad) in ads.iter().enumerate() {
            if i == id {continue;}
            if ads[id].r < (rx - lx) * (m - ly) {ok = false;break;}
            if intersect2(lx, rx, ly, m, ad) {ok = false;break;}
        }
        if ok {l = m;} else {r = m;}
    }
    ry = l;
    ads[id].lx = lx;
    ads[id].rx = rx;
    ads[id].ly = ly;
    ads[id].ry = ry;
}

fn find_rect(ads: &mut Vec<Ad>, i: usize, rgx: i32, rgy: i32, dx: i32, dy: i32, neighbor: &Vec<usize>, pre_sz: i32, rnd: &mut XorShift32) -> (i32, i32, i32, i32) {
    let ps = ads[i].pos;
    let (mut lx, mut rx, mut ly, mut ry) = (0.max(ps.0 - rgx + dx), R.min(ps.0 + 1.max(rgx + dx)), 0.max(ps.1 - rgy + dy), R.min(ps.1 + 1.max(rgy + dy)));
    for &j in neighbor.iter().filter(|j| i!=**j) {
        if pre_sz > (rx - lx) * (ry - ly) {
            lx = ps.0;rx = ps.0 + 1;
            ly = ps.1;ry = ps.1 + 1;
            break;
        }
        let adi = &ads[j];
        if !intersect2(lx, rx, ly, ry, adi) {continue;}
        if adi.pos.0 <= ps.0 && adi.pos.1 <= ps.1 {
            if ps.1 < adi.ry {lx = lx.max(adi.rx);continue;}
            if ps.0 < adi.rx {ly = ly.max(adi.ry);continue;}
            if (rx - lx.max(adi.rx)) * (ry - ly) > (rx - lx) * (ry - ly.max(adi.ry)) {
                lx = lx.max(adi.rx);
            }
            else {
                ly = ly.max(adi.ry);
            }
            
        }
        else if adi.pos.0 <= ps.0 {
            if ps.1 >= adi.ly {lx = lx.max(adi.rx);continue;}
            if ps.0 < adi.rx {ry = ry.min(adi.ly);continue;}
            if (rx - lx.max(adi.rx)) * (ry - ly) > (rx - lx) * (ry.min(adi.ly - ly)) {
                lx = lx.max(adi.rx);
            }
            else {
                ry = ry.min(adi.ly);
            }
            
        }
        else if adi.pos.1 <= ps.1 {
            if ps.1 < adi.ry {rx = rx.min(adi.lx);continue;}
            if ps.0 >= adi.lx {ly = ly.max(adi.ry);continue;}
            if (rx.min(adi.lx) - lx) * (ry - ly) > (rx - lx) * (ry - ly.max(adi.ry)) {
                rx = rx.min(adi.lx);
            }
            else {
                ly = ly.max(adi.ry);
            }
            
        }
        else {
            if ps.1 >= adi.ly {rx = rx.min(adi.lx);continue;}
            if ps.0 >= adi.lx {ry = ry.min(adi.ly);continue;}
            if (rx.min(adi.lx) - lx) * (ry - ly) > (rx - lx) * (ry.min(adi.ly) - ly) {
                rx = rx.min(adi.lx);
            }
            else {
                ry = ry.min(adi.ly);
            }
        }
    }

    if ads[i].r < (rx - lx) * (ry - ly) {
        let r = ads[i].r;
         
        for _ in 0..6 {
            if (rx - lx) * (ry - ly) <= r {break;}
            //if rx - lx > ry - ly {
            if (rnd.next_int() & 1) == 0 {
                let diff = 0.max(rx - lx - r / (ry - ly));
                if rx - ps.0 - 1 <= ps.0 - lx && lx != 0 || rx == R {
                    lx += (ps.0 - lx).min(diff);
                }  
                else {
                    rx -= (rx - ps.0 - 1).min(diff);
                }
                
            }
            else {
                let diff = 0.max(ry - ly - r / (rx - lx));
                if ry - ps.1 - 1 <= ps.1 - ly && ly != 0 || ry == R {
                    ly += (ps.1 - ly).min(diff);
                }  
                else {
                    ry -= (ry - ps.1 - 1).min(diff);
                }
                
            }
        }
        if r < (rx - lx) * (ry - ly) {
            lx += (ps.0 - lx).min(0.max(rx - lx - r / (ry - ly)));
            rx -= (rx - ps.0 - 1).min(0.max(rx - lx - r / (ry - ly)));
            ly += (ps.1 - ly).min(0.max(ry - ly - r / (rx - lx)));
            ry -= (ry - ps.1 - 1).min(0.max(ry - ly - r / (rx - lx)));
        }
        
    }
    (lx, rx, ly, ry)
}

fn find_rect2(ads: &mut Vec<Ad>, i: usize, lx: i32, rx: i32, ly: i32, ry: i32, neighbor: &Vec<usize>, rnd: &mut XorShift32) -> (i32, i32, i32, i32) {
    let ps = ads[i].pos;
    let (mut lx, mut rx, mut ly, mut ry) = (lx, rx, ly, ry);
    for &j in neighbor.iter().filter(|j| i!=**j) {
        let adi = &ads[j];
        if !intersect2(lx, rx, ly, ry, adi) {continue;}
        if adi.pos.0 <= ps.0 && adi.pos.1 <= ps.1 {
            if ps.1 < adi.ry {lx = lx.max(adi.rx);continue;}
            if ps.0 < adi.rx {ly = ly.max(adi.ry);continue;}
            if (rx - lx.max(adi.rx)) * (ry - ly) > (rx - lx) * (ry - ly.max(adi.ry)) {
                lx = lx.max(adi.rx);
            }
            else {
                ly = ly.max(adi.ry);
            }
            
        }
        else if adi.pos.0 <= ps.0 {
            if ps.1 >= adi.ly {lx = lx.max(adi.rx);continue;}
            if ps.0 < adi.rx {ry = ry.min(adi.ly);continue;}
            if (rx - lx.max(adi.rx)) * (ry - ly) > (rx - lx) * (ry.min(adi.ly - ly)) {
                lx = lx.max(adi.rx);
            }
            else {
                ry = ry.min(adi.ly);
            }
            
        }
        else if adi.pos.1 <= ps.1 {
            if ps.1 < adi.ry {rx = rx.min(adi.lx);continue;}
            if ps.0 >= adi.lx {ly = ly.max(adi.ry);continue;}
            if (rx.min(adi.lx) - lx) * (ry - ly) > (rx - lx) * (ry - ly.max(adi.ry)) {
                rx = rx.min(adi.lx);
            }
            else {
                ly = ly.max(adi.ry);
            }
            
        }
        else {
            if ps.1 >= adi.ly {rx = rx.min(adi.lx);continue;}
            if ps.0 >= adi.lx {ry = ry.min(adi.ly);continue;}
            if (rx.min(adi.lx) - lx) * (ry - ly) > (rx - lx) * (ry.min(adi.ly) - ly) {
                rx = rx.min(adi.lx);
            }
            else {
                ry = ry.min(adi.ly);
            }
        }
    }

    if ads[i].r < (rx - lx) * (ry - ly) {
        let r = ads[i].r;
         
        for _ in 0..6 {
            if (rx - lx) * (ry - ly) <= r {break;}
            
            //if rx - lx > ry - ly {
            if (rnd.next_int() & 1) == 0 {
                let diff = 0.max(rx - lx - r / (ry - ly));
                if rx - ps.0 - 1 <= ps.0 - lx && lx != 0 || rx == R {
                    lx += (ps.0 - lx).min(diff);
                }  
                else {
                    rx -= (rx - ps.0 - 1).min(diff);
                }
            }
            else {
                let diff = 0.max(ry - ly - r / (rx - lx));
                if ry - ps.1 - 1 <= ps.1 - ly && ly != 0 || ry == R {
                    ly += (ps.1 - ly).min(diff);
                }  
                else {
                    ry -= (ry - ps.1 - 1).min(diff);
                }
            }
        }
        if r < (rx - lx) * (ry - ly) {
            lx += (ps.0 - lx).min(0.max(rx - lx - r / (ry - ly)));
            rx -= (rx - ps.0 - 1).min(0.max(rx - lx - r / (ry - ly)));
            ly += (ps.1 - ly).min(0.max(ry - ly - r / (rx - lx)));
            ry -= (ry - ps.1 - 1).min(0.max(ry - ly - r / (rx - lx)));
        }
        
    }
    (lx, rx, ly, ry)
}

fn find_rect3(ads: &mut Vec<Ad>, i: usize, rgx: i32, rgy: i32, dx: i32, dy: i32, neighbor: &Vec<usize>, rnd: &mut XorShift32) -> (i32, i32, i32, i32) {
    let ps = ads[i].pos;
    let mut mx = 0;
    let (mut mlx, mut mrx, mut mly, mut mry) = (0, 0, 0, 0);
    for _ in 0..3 { 
        let (mut lx, mut rx, mut ly, mut ry) = (0.max(ps.0 - rgx + dx), R.min(ps.0 + rgx + dx), 0.max(ps.1 - rgy + dy), R.min(ps.1 + rgy +dy));
        for &j in neighbor.iter().filter(|j| i!=**j) {
            if mx >= (rx - lx) * (ry - ly) {break;}
            let adi = &ads[j];
            if !intersect2(lx, rx, ly, ry, adi) {continue;}
            if adi.pos.0 <= ps.0 && adi.pos.1 <= ps.1 {
                if ps.1 < adi.ry {lx = lx.max(adi.rx);continue;}
                if ps.0 < adi.rx {ly = ly.max(adi.ry);continue;}
                if (rnd.next_int() & 1) == 0 {
                    lx = lx.max(adi.rx);
                }
                else {
                    ly = ly.max(adi.ry);
                }
            }
            else if adi.pos.0 <= ps.0 {
                if ps.1 >= adi.ly {lx = lx.max(adi.rx);continue;}
                if ps.0 < adi.rx {ry = ry.min(adi.ly);continue;}
                if (rnd.next_int() & 1) == 0 {
                    lx = lx.max(adi.rx);
                }
                else {
                    ry = ry.min(adi.ly);
                }
                
            }
            else if adi.pos.1 <= ps.1 {
                if ps.1 < adi.ry {rx = rx.min(adi.lx);continue;}
                if ps.0 >= adi.lx {ly = ly.max(adi.ry);continue;}
                if (rnd.next_int() & 1) == 0 {
                    rx = rx.min(adi.lx);
                }
                else {
                    ly = ly.max(adi.ry);
                }
                
            }
            else {
                if ps.1 >= adi.ly {rx = rx.min(adi.lx);continue;}
                if ps.0 >= adi.lx {ry = ry.min(adi.ly);continue;}
                if (rnd.next_int() & 1) == 0 {
                    rx = rx.min(adi.lx);
                }
                else {
                    ry = ry.min(adi.ly);
                }
            }
            
        }
        if mx < (rx - lx) * (ry - ly) {
            mx = (rx - lx) * (ry - ly);
            mlx = lx;
            mrx = rx;
            mly = ly;
            mry = ry;
            if mx >= ads[i].r {break;}
        }
    }
    let (mut lx, mut rx, mut ly, mut ry) = (mlx, mrx, mly, mry);
    if ads[i].r < (rx - lx) * (ry - ly) {
        let r = ads[i].r;
         
        for _ in 0..6 {
            if (rx - lx) * (ry - ly) <= r {break;}
            //if rx - lx > ry - ly {
            if (rnd.next_int() & 1) == 0 {
                let diff = 0.max(rx - lx - r / (ry - ly));
                if rx - ps.0 - 1 <= ps.0 - lx && lx != 0 || rx == R {
                    lx += (ps.0 - lx).min(diff);
                }  
                else {
                    rx -= (rx - ps.0 - 1).min(diff);
                }
                
            }
            else {
                let diff = 0.max(ry - ly - r / (rx - lx));
                if ry - ps.1 - 1 <= ps.1 - ly && ly != 0 || ry == R {
                    ly += (ps.1 - ly).min(diff);
                }  
                else {
                    ry -= (ry - ps.1 - 1).min(diff);
                }
                
            }
        }
        if r < (rx - lx) * (ry - ly) {
            lx += (ps.0 - lx).min(0.max(rx - lx - r / (ry - ly)));
            rx -= (rx - ps.0 - 1).min(0.max(rx - lx - r / (ry - ly)));
            ly += (ps.1 - ly).min(0.max(ry - ly - r / (rx - lx)));
            ry -= (ry - ps.1 - 1).min(0.max(ry - ly - r / (rx - lx)));
        }
        
    }
    (lx, rx, ly, ry)
}

fn init_solve(ads: &mut Vec<Ad>, n: usize, neighbor: &Vec<Vec<usize>>, rnd: &mut XorShift32) -> f64 {
    let mut score = 0.;
    for i in 0..n {
        let rng = find_rect(ads, i, ads[i].r.sqrt(), ads[i].r.sqrt(), 0, 0, &neighbor[i], 0, rnd);
        ads[i].lx = rng.0;
        ads[i].rx = rng.1;
        ads[i].ly = rng.2;
        ads[i].ry = rng.3;
        score += ads[i].score() / (n as f64);
    }
    score
}
fn improve (ads: &mut Vec<Ad>, timer: &mut Timer, rnd: &mut XorShift32, score: f64, trycnt: usize, neighbor: &Vec<Vec<usize>>) -> f64 {
    let mut cutratio = 0.7;
    let mut cand =  ads.iter().enumerate().filter(|(_, adi)| adi.score() < cutratio).map(|(i, _)| i).collect::<Vec<usize>>();
    let mut bestscore = score;
    let mut best_ads = ads.clone();
    let mut loopcnt = 0;
    let n = ads.len();
    let inv_n = 1.0 / (n as f64);
    let sqrt_r = ads.iter().map(|ad| ad.r.sqrt() as f64).collect::<Vec<f64>>();
    let mut b_scores = ads.iter().map(|ad| ad.score()).collect::<Vec<f64>>();
    let mut watch_id = vec![];
    let mut watch_pos = vec![];
    let start = timer.get_time();
    let mut mn_score = b_scores.iter().fold(INF, |acc, &x| acc.min(x));
    loop {
        loopcnt += 1;
        if bestscore >= 1.0 {break;}
        if cand.len() == 0 {
            if cutratio == 0.7 {
                cutratio = 0.99;
                cand =  ads.iter().enumerate().filter(|(_, adi)| adi.score() < cutratio).map(|(i, _)| i).collect::<Vec<usize>>();
            }
        }
        //if (loopcnt & 127) == 0 && timer.get_time() > TIMELIMIT {break;}
        
        if (loopcnt & 127) == 0 {
            let t = timer.get_time();
            if t - start > TIMELIMIT / trycnt as f64 {break;}
            if t - start > 1. / trycnt as f64 && cutratio == 0.7 {
                cutratio = 0.99;
                cand =  ads.iter().enumerate().filter(|(_, adi)| adi.score() < cutratio).map(|(i, _)| i).collect::<Vec<usize>>();
            }
        }
        
        let id = if (loopcnt & 1) == 0 || cand.len() == 0 {rnd.nextn(n)} else {cand[rnd.nextn(cand.len())]};
        let ad_backup = ads[id];
        if cand.len() == 0 || loopcnt % 9 == 0 {
            let fac = (1.0 - b_scores[id]).max(0.01).sqrt() * 7.;
            let add = (sqrt_r[id] * (rnd.next_double() * fac + 0.002)).round() as i32;
            match rnd.next_int() & 3 {
                0 => {ads[id].lx = 0.max(ads[id].lx - add);},
                1 => {ads[id].rx = R.min(ads[id].rx + add);},
                2 => {ads[id].ly = 0.max(ads[id].ly - add);},
                3 => {ads[id].ry = R.min(ads[id].ry + add);},
                _ => {}
            }
            let add2 = (sqrt_r[id] * (rnd.next_double() * fac * 0.25 + 0.002)).round() as i32;
            match rnd.next_int() & 7 {
                0 => {ads[id].lx = 0.max(ads[id].lx - add2);},
                1 => {ads[id].rx = R.min(ads[id].rx + add2);},
                2 => {ads[id].ly = 0.max(ads[id].ly - add2);},
                3 => {ads[id].ry = R.min(ads[id].ry + add2);},
                _ => {}
            }

            watch_id.clear();
            watch_pos.clear();
            for &j in &neighbor[id] {
                if id == j {continue;}
                if intersect(&ads[id], &ads[j]) {
                    let adi = &mut ads[j];
                    watch_pos.push((adi.lx, adi.rx, adi.ly, adi.ry));
                    adi.lx = adi.pos.0;adi.rx = adi.pos.0 + 1;
                    adi.ly = adi.pos.1;adi.ry = adi.pos.1 + 1;
                    watch_id.push(j);
                }
            }
            
            let rng = find_rect2(ads, id, ads[id].lx, ads[id].rx, ads[id].ly, ads[id].ry, &neighbor[id], rnd);
            ads[id].lx = rng.0;ads[id].rx = rng.1;
            ads[id].ly = rng.2;ads[id].ry = rng.3;

            let mut diff = 0.;
            diff += ads[id].score() - b_scores[id];

            let mut cnt = watch_id.len() as f64;
            for (&i, &ps) in watch_id.iter().zip(watch_pos.iter()) {
                let rng = find_rect2(ads, i, 0.max(ps.0 - add), R.min(ps.1 + add), 0.max(ps.2 - add), R.min(ps.3 + add), &neighbor[i], rnd);
                ads[i].lx = rng.0;ads[i].rx = rng.1;
                ads[i].ly = rng.2;ads[i].ry = rng.3;
                diff += ads[i].score() - b_scores[i];
                cnt -= 1.;
                if diff + cnt * (1.0 - mn_score) < 0. {break;} 
            }

            if diff > 0. {
                bestscore += diff * inv_n;
                best_ads = ads.clone();
                b_scores = ads.iter().map(|ad| ad.score()).collect::<Vec<f64>>();
                mn_score = b_scores.iter().fold(INF, |acc, &x| acc.min(x));
                cand = ads.iter().enumerate().filter(|(_, ad)| ad.score() < cutratio).map(|(i, _)| i).collect::<Vec<_>>();
                //eprintln!("{} {}", timer.get_time(), bestscore);
            }
            else {
                for (&i, &ps) in watch_id.iter().zip(watch_pos.iter()) {
                    ads[i].lx = ps.0;
                    ads[i].rx = ps.1;
                    ads[i].ly = ps.2;
                    ads[i].ry = ps.3;
                }
                ads[id] = ad_backup;
            }
            continue;
        }

        let rg = (sqrt_r[id] * (rnd.next_double() * 0.4 + 0.3)) as i32;
        let rg2 = (sqrt_r[id] * (rnd.next_double() * 0.4 + 0.3)) as i32;
        let fac = rnd.next_double() * 0.9 + 0.1;
        let mut rgx = (rg as f64 * fac).round() as i32;
        let mut rgy = (rg2 as f64 / fac).round() as i32;
        if (rnd.next_int() & 1) == 0 {std::mem::swap(&mut rgx, &mut rgy);}
        let dx = (rgx as f64 * (rnd.next_double() * 2. - 1.)) as i32;
        let dy = (rgy as f64 * (rnd.next_double() * 2. - 1.)) as i32;
        
        let ps = ads[id].pos;
        ads[id].lx = 0.max(ps.0 - rgx + dx);ads[id].rx = R.min(ps.0 + 1.max(rgx + dx));
        ads[id].ly = 0.max(ps.1 - rgy + dy);ads[id].ry = R.min(ps.1 + 1.max(rgy + dy));

        watch_id.clear();
        watch_pos.clear();

        for &j in &neighbor[id] {
            if id == j {continue;}
            if intersect(&ads[id], &ads[j]) {
                let adi = &mut ads[j];
                watch_pos.push((adi.lx, adi.rx, adi.ly, adi.ry));
                adi.lx = adi.pos.0;adi.rx = adi.pos.0 + 1;
                adi.ly = adi.pos.1;adi.ry = adi.pos.1 + 1;
                watch_id.push(j);
            }
        }
        let pre_sz = (ad_backup.size() as f64 * 0.9).floor() as i32;
        let rng = find_rect(ads, id, rgx, rgy, dx, dy, &neighbor[id], pre_sz, rnd);
        //let rng = find_rect3(ads, id, rgx, rgy, dx, dy, &neighbor[id], rnd);
        ads[id].lx = rng.0;ads[id].rx = rng.1;
        ads[id].ly = rng.2;ads[id].ry = rng.3;
        
        if ads[id].score() + 0.01 < b_scores[id] {
            for (&i, &ps) in watch_id.iter().zip(watch_pos.iter()) {
                ads[i].lx = ps.0;
                ads[i].rx = ps.1;
                ads[i].ly = ps.2;
                ads[i].ry = ps.3;
            }
            ads[id] = ad_backup;
            continue;
        }

        let mut diff = 0.;
        diff += ads[id].score() - b_scores[id];
        let mut cnt = watch_id.len() as f64;
        for &i in &watch_id {
            let rgx2 = (sqrt_r[i].max(rgx as f64) * (rnd.next_double() * 8. + 4.)).round() as i32;
            let rgy2 = (sqrt_r[i].max(rgy as f64) * (rnd.next_double() * 8. + 4.)).round() as i32;
            //let rng = find_rect(ads, i, rgx2, rgy2, 0, 0, &neighbor[i]);
            let rng = find_rect3(ads, i, rgx2, rgy2, 0, 0, &neighbor[i], rnd);
            ads[i].lx = rng.0;ads[i].rx = rng.1;
            ads[i].ly = rng.2;ads[i].ry = rng.3;
            diff += ads[i].score() - b_scores[i];
            cnt -= 1.;
            if diff + cnt * (1.0 - mn_score) < 0. {break;}
        }

        if diff > 0. {
            bestscore += diff * inv_n;
            best_ads = ads.clone();
            b_scores = ads.iter().map(|ad| ad.score()).collect::<Vec<f64>>();
            mn_score = b_scores.iter().fold(INF, |acc, &x| acc.min(x));
            cand = ads.iter().enumerate().filter(|(_, ad)| ad.score() < cutratio).map(|(i, _)| i).collect::<Vec<_>>();
        }
        else {
            for (&i, &ps) in watch_id.iter().zip(watch_pos.iter()) {
                ads[i].lx = ps.0;
                ads[i].rx = ps.1;
                ads[i].ly = ps.2;
                ads[i].ry = ps.3;
            }
            ads[id] = ad_backup;
        }
    }
    eprintln!("loop:{} best:{}", loopcnt, bestscore);
    *ads = best_ads.clone();
    bestscore
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();
    
    let mut timer = Timer::new();
    let mut rnd = XorShift32::new(0);
    input!{
        input,
        n: usize,
        data: [(i32, i32, i32); n],
    }
    let mut ads = Vec::new();
    for (i, ad) in data.into_iter().enumerate() {
        ads.push(Ad{pos: (ad.0, ad.1), r: ad.2, index: i, lx: ad.0, rx: ad.0 + 1, ly: ad.1, ry: ad.1 + 1});
    }
    //ads.sort_by(|x,y| y.r.cmp(&x.r));
    ads.sort_by(|x,y| x.r.cmp(&y.r));
    let iota = (0..n).collect::<Vec<usize>>();

    let mut neighbor = vec![iota.clone(); n];
    for (i,ni) in neighbor.iter_mut().enumerate() {
        ni.sort_by(|a, b| 
            ((ads[i].pos.0 - ads[*a].pos.0).abs() + (ads[i].pos.1 - ads[*a].pos.1).abs()).cmp(
                &((ads[i].pos.0 - ads[*b].pos.0).abs() + (ads[i].pos.1 - ads[*b].pos.1).abs())
            )
        );
    }
    let neighbor = neighbor;

    let score = init_solve(&mut ads, n, &neighbor, &mut rnd);
    eprintln!("{}", score);

    let base_ads = ads.clone();
    let mut best_ads = ads.clone();
    let trycnt = if n < 100 {20} else {10};
    let mut bestscore = score;
    for _ in 0..trycnt {
        let ret_score = improve(&mut ads, &mut timer, &mut rnd, score, trycnt*3/2, &neighbor);
        if ret_score > bestscore {
            bestscore = ret_score;
            best_ads = ads.clone();
        }
        ads = base_ads.clone();
    }
    ads = best_ads.clone();
    let ret_score = improve(&mut ads, &mut timer, &mut rnd, bestscore, 3, &neighbor);
    if ret_score > bestscore {
        //bestscore = ret_score;
        best_ads = ads.clone();
    }
    //let _score = improve(&mut ads, &mut timer, &mut rnd, score);

    for i in 0..n {
        if best_ads[i].score() < 0.998 {extend_rect(&mut best_ads, i);}
    }
    /*
    for i in 0..n {
        eprintln!("{} {}", ads[i].index, ads[i].score());
    }
    */
    eprintln!("{}", timer.get_time());
    output_answer(&best_ads);
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
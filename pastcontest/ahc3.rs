#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::io::Read;
pub fn readln() -> String {
	let mut line = String::new();
	::std::io::stdin().read_line(&mut line).unwrap_or_else(|e| panic!("{}", e));
	line
}
 
macro_rules! read {
	($($t:tt),*; $n:expr) => {{
		let stdin = ::std::io::stdin();
		let ret = ::std::io::BufRead::lines(stdin.lock()).take($n).map(|line| {
			let line = line.unwrap();
			let mut it = line.split_whitespace();
			_read!(it; $($t),*)
		}).collect::<Vec<_>>();
		ret
	}};
	($($t:tt),*) => {{
		let line = readln();
		let mut it = line.split_whitespace();
		_read!(it; $($t),*)
	}};
}
 
macro_rules! _read {
	($it:ident; [char]) => {
		_read!($it; String).chars().collect::<Vec<_>>()
	};
	($it:ident; [u8]) => {
		Vec::from(_read!($it; String).into_bytes())
	};
	($it:ident; usize1) => {
        $it.next().unwrap_or_else(|| panic!("input mismatch")).parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1
	};
	($it:ident; [usize1]) => {
		$it.map(|s| s.parse::<usize>().unwrap_or_else(|e| panic!("{}", e)) - 1).collect::<Vec<_>>()
	};
	($it:ident; [$t:ty]) => {
		$it.map(|s| s.parse::<$t>().unwrap_or_else(|e| panic!("{}", e))).collect::<Vec<_>>()
	};
	($it:ident; $t:ty) => {
		$it.next().unwrap_or_else(|| panic!("input mismatch")).parse::<$t>().unwrap_or_else(|e| panic!("{}", e))
	};
	($it:ident; $($t:tt),+) => {
		($(_read!($it; $t)),*)
	};
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
const INF: usize = 1000100010;
const TIMELIMIT: f64 = 1.9;
const Q: usize = 1000;
const H: usize = 30;
const W: usize = 30;
const MC: usize = 1000;
const DY: [usize; 4] = [!0, 0, 1, 0];
const DX: [usize; 4] = [0, !0, 0, 1];//ULDR
const DYX: [usize; 26] = [!12, !11, !10, !9, !8, !7, !6, !5, !4, !3, !2, !1, !0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
const DY2: [usize; 8] = [!0, !0, !0, 0, 0, 1, 1, 1];
const DX2: [usize; 8] = [!0, 0, 1, !0, 1, !0, 0, 1];

fn output_string(s: &String){
	println!("{}", s)
}

fn shortest_path(
	si: usize, sj: usize, ti: usize, tj: usize,
	g: &Vec<Vec<f64>>,
	parent: &mut Vec<(usize, usize)>,
	_s_count: &Vec<Vec<f64>>,
	_tm: usize,
	_rnd: &mut XorShift32) {
	let mut cost = vec![vec![INF; H]; W];
	let mut pq = BinaryHeap::new();
	cost[si][sj] = 0;
	pq.push(Reverse((0, (si, sj))));
	while let Some(Reverse((co, pos))) = pq.pop() {
		if pos.0 == ti && pos.1 == tj {break;}
		if cost[pos.0][pos.1] < co {continue;}
		for k in 0..4 {
			let ny = pos.0 + DY[k];
			let nx = pos.1 + DX[k];
			if ny < H && nx < W && cost[ny][nx] > co + g[ny*H+nx][k] as usize {
				cost[ny][nx] = co + g[ny*H+nx][k] as usize;
				pq.push(Reverse((cost[ny][nx], (ny, nx))));
				parent[ny*H+nx] = pos;
			}
		}
	}
}

fn straight_path(
	si: usize, sj: usize, ti: usize, tj: usize,
	g: &Vec<Vec<f64>>,
	parent: &mut Vec<(usize, usize)>,
	s_count: &Vec<Vec<f64>>,
	total_count: f64,
	_rnd: &mut XorShift32,
	_tm: usize) -> bool {
	let y_rng = if ti > si {ti - si} else {si - ti};
	let x_rng = if tj > sj {tj - sj} else {sj - tj};
	let mut mx = -1000100010.;
	let mut s_pos = 0;
	let mut dir = 0;
	let pp = 0.01;
	for td in 0..2 {
		let rng = if td == 0 {(x_rng, y_rng)} else {(y_rng, x_rng)};
		for sp in 0..=rng.0 {
			//if sp > 0 && sp < rng.0 && tm < 100 {continue;}
			let mut cnt = 0;
			let mut vsum = 0.0;
			let (mut py, mut px) = (si, sj);
			while cnt < x_rng + y_rng { 
				if td == 0 && cnt >= sp && cnt < sp + rng.1 || td == 1 && (cnt < sp || cnt >= sp + rng.1) {
					if ti > si {
						//vsum += (10000.0 - g[py*H+px][2])*pp + (0.5*(alpha+1.0)*(e*total_count/(alpha+1.0).powf(s_count[py*H+px][2])).ln()/(alpha+1.0).powf(s_count[py*H+px][2])).sqrt();
						vsum += (10000.0 - g[py*H+px][2])*pp + (2.0*total_count.ln()/s_count[py*H+px][2]).sqrt();
						py += 1;
					}
					else {
						//vsum += (10000.0 - g[py*H+px][0])*pp + (0.5*(alpha+1.0)*(e*total_count/(alpha+1.0).powf(s_count[py*H+px][0])).ln()/(alpha+1.0).powf(s_count[py*H+px][0])).sqrt();
						vsum += (10000.0 - g[py*H+px][0])*pp + (2.0*total_count.ln()/s_count[py*H+px][0]).sqrt();
						py -= 1;
					}
				}
				else {
					if tj > sj {
						//vsum += (10000.0 - g[py*H+px][3])*pp + (0.5*(alpha+1.0)*(e*total_count/(alpha+1.0).powf(s_count[py*H+px][3])).ln()/(alpha+1.0).powf(s_count[py*H+px][3])).sqrt();
						vsum += (10000.0 - g[py*H+px][3])*pp + (2.0*total_count.ln()/s_count[py*H+px][3]).sqrt();
						px += 1;
					}
					else {
						//vsum += (10000.0 - g[py*H+px][1])*pp + (0.5*(alpha+1.0)*(e*total_count/(alpha+1.0).powf(s_count[py*H+px][1])).ln()/(alpha+1.0).powf(s_count[py*H+px][1])).sqrt();
						vsum += (10000.0 - g[py*H+px][1])*pp + (2.0*total_count.ln()/s_count[py*H+px][1]).sqrt();
						px -= 1;
					}
				}
				cnt += 1
			}
			if vsum > mx {
				mx = vsum;
				s_pos = sp;
				dir = td;
			}
		}
	}
	//eprintln!("{}",mx / (x_rng+y_rng) as f64);
	if (mx / (x_rng+y_rng) as f64) < 50. {
		return true;
	}
	let mut cnt = 0;
	let (mut py, mut px) = (si, sj);
	let rng = if dir == 0 {(x_rng, y_rng)} else {(y_rng, x_rng)};
	while cnt < x_rng + y_rng { 
		if dir == 0 && cnt >= s_pos && cnt < s_pos + rng.1 || dir == 1 && (cnt < s_pos || cnt >= s_pos + rng.1) {
			if ti > si {
				parent[(py+1)*H+px] = (py, px);
				py += 1;
			}
			else {
				parent[(py-1)*H+px] = (py, px);
				py -= 1;
			}
		}
		else {
			if tj > sj {
				parent[py*H+px+1] = (py, px);
				px += 1;
			}
			else {
				parent[py*H+px-1] = (py, px);
				px -= 1;
			}
		}
		cnt += 1
	}
	false
}

fn update_val(g: &mut Vec<Vec<f64>>, s_count: &mut Vec<Vec<f64>>, y:usize, x: usize, k: usize, cur_val: f64, p: f64) {
	g[y*H+x][k] = (g[y*H+x][k] * s_count[y*H+x][k] + cur_val * p) / (s_count[y*H+x][k] + p);
	s_count[y*H+x][k] += p;
}

fn fill_cell2(g: &mut Vec<Vec<f64>>, s_count: &mut Vec<Vec<f64>>, tm: usize) {
	let base = 15000.;
	let bo = base * (((tm+1 - 100)/400 + 1) as f64);
	let bo2 = (base - 1000.) * (((tm+1 - 100)/400 + 1) as f64);
	let pp = base - 1000.;
	for h in 0..H {
		for &k in [1,3].iter() {
			let mut s2 = 0.;
			let mut s = 0.;
			let mut cnt = 0.;
			for w in 0..W {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] > bo {
					s += g[h*H + w][k];
					s2 += g[h*H + w][k] * g[h*H + w][k];
					cnt += 1.;
				}
			}
			if cnt == 0. {continue;}
			let mut l = 0;
			let mut r = 0;
			let mut mn = s2/cnt - s*s/(cnt*cnt);
			let (mut al, mut ar) = (s/cnt, s/cnt);
			let mut ss = 0.;
			let mut ss2 = 0.;
			let mut cnt2 = 0.;
			for w in 0..W {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] > bo {
					ss += g[h*H + w][k];
					ss2 += g[h*H + w][k] * g[h*H + w][k];
					cnt2 += 1.;
				}
				if cnt2 == 0.{continue;}
				if (cnt - cnt2).abs() < 1e-7 {break;}
				let di =  ss2/cnt2 - ss*ss/(cnt2*cnt2);
				let di2 = (s2-ss2)/(cnt-cnt2) - (s-ss)*(s-ss)/((cnt-cnt2)*(cnt-cnt2));
				if mn > di/cnt2 + di2/(cnt-cnt2) {
					mn = di/cnt2 + di2/(cnt-cnt2);
					al = ss/cnt2;
					ar = (s-ss)/(cnt-cnt2);
					l = w+1;
				}
				if (mn - di/cnt2 - di2/(cnt-cnt2)).abs() < 1e-7 {
					r = w+1;
				}
			}
			let mid = (r+l)/2;
			//eprint!("{} ",mid);
			for w in 0..W {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] < bo2 {
					if s_count[h*W+w][k] == 0.1 {
						g[h*H+w][k] = if w < mid {al} else {ar};
					}
					g[h*H+w][k] = if w < mid {(al+g[h*H+w][k])/2.} else {(ar+g[h*H+w][k])/2.};
					s_count[h*W+w][k] += pp;
				}
			}
		}
	}
	for w in 0..W {
		for &k in [0,2].iter() {
			let mut s2 = 0.;
			let mut s = 0.;
			let mut cnt = 0.;
			for h in 0..H {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] > bo {
					s += g[h*H + w][k];
					s2 += g[h*H + w][k] * g[h*H + w][k];
					cnt += 1.;
				}
			}
			if cnt == 0. {continue;}
			let mut l = 0;
			let mut r = 0;
			let mut mn = s2/cnt - s*s/(cnt*cnt);
			let (mut al, mut ar) = (s/cnt, s/cnt);
			let mut ss = 0.;
			let mut ss2 = 0.;
			let mut cnt2 = 0.;
			for h in 0..H {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] > bo {
					ss += g[h*H + w][k];
					ss2 += g[h*H + w][k] * g[h*H + w][k];
					cnt2 += 1.;
				}
				if cnt2 == 0.{continue;}
				if (cnt - cnt2).abs() < 1e-7 {break;}
				let di =  ss2/cnt2 - ss*ss/(cnt2*cnt2);
				let di2 = (s2-ss2)/(cnt-cnt2) - (s-ss)*(s-ss)/((cnt-cnt2)*(cnt-cnt2));
				if mn > di/cnt2 + di2/(cnt-cnt2) {
					mn = di/cnt2 + di2/(cnt-cnt2);
					al = ss/cnt2;
					ar = (s-ss)/(cnt-cnt2);
					l = h+1;
				}
				if (mn - di/cnt2 - di2/(cnt-cnt2)).abs() < 1e-7 {
					r = h+1;
				}
			}
			let mid = (r+l)/2;
			//eprintln!("{}", mid);
			for h in 0..H {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] < bo2 {
					if s_count[h*W+w][k] == 0.1 {
						g[h*H+w][k] = if h < mid {al} else {ar};
					}
					g[h*H+w][k] = if h < mid {(al+g[h*H+w][k])/2.} else {(ar+g[h*H+w][k])/2.};
					s_count[h*W+w][k] += pp;
				}
			}
		}
	}
}

fn fill_cell(g: &mut Vec<Vec<f64>>, s_count: &mut Vec<Vec<f64>>, tm: usize) {
	let _t = tm as f64;
	let pp = 5.;
	for h in 0..H {//[14, 15, 13, 16, 12, 17, 11, 18, 10, 19, 9, 20, 8, 21, 7, 22, 6, 23, 5, 24, 4, 25, 3, 26, 2, 27, 1, 28, 0, 29].iter() {
		for w in 0..W {//[14, 15, 13, 16, 12, 17, 11, 18, 10, 19, 9, 20, 8, 21, 7, 22, 6, 23, 5, 24, 4, 25, 3, 26, 2, 27, 1, 28, 0, 29].iter() {
			for k in 0..4 {
				if k==0 && h==0 || k==1 && w==0 || k==2 && h==H-1 || k==3 && w==W-1 {continue;}
				if s_count[h*H+w][k] <= 250. {
					let mut v_sum = g[h*H+w][k] * 0.1;
					let mut tcnt = 0.1;
					for &k2 in DYX.iter() {
						let (ny, nx) = (h+DY[k]*k2, w+DX[k]*k2);
						if ny < H && nx < W && s_count[ny*H+nx][k] > 300. {
							tcnt += pp;
							v_sum += g[ny*H+nx][k] * pp;
						}
					}
					if tcnt > 0.1 {
						g[h*H+w][k] = v_sum / tcnt;
						s_count[h*W+w][k] += tcnt - 0.1;
					}
				}
			}
		}
	}
}

fn improve(
	g: &mut Vec<Vec<f64>>,
	_s_count: &mut Vec<Vec<f64>>,
	score_list: &Vec<f64>,
	record: &Vec<Vec<Vec<usize>>>,
	timer: &Timer,
	rnd: &mut XorShift32){

	let mut curscore = 0.;
	let mut calc_scores = vec![0.; score_list.len()];
	for h in 0..H {
		for w in 0..W {
			for k in 0..2 {
				for &tid in &record[h*H+w][k] {
					calc_scores[tid] += g[h*H+w][k];
				}
			}
		}
	}
	for (&c1, &c2) in score_list.iter().zip(calc_scores.iter()) {
		curscore += (c1-c2).powf(2.0);
	}
	//eprintln!("{}", curscore);

    for _ in 0..500 {
        let t = timer.get_time();
        if t > TIMELIMIT {
            break;
        }
        for h in 0..H {
			let mut nxscore = 0.0;
			let c = if (rnd.next_int()&1) == 0 {7.5} else {-7.5};
			let rng = rnd.nextn(15) + 10;
			let l = rnd.nextn(H-rng-1)+1;
			for w in l..=l+rng {
				if g[h*H+w][1] <= 1000. {continue;}
				for &tid in &record[h*H+w][1] {
					calc_scores[tid] += c;
				}
			}
			for tid in 0..calc_scores.len() {
				nxscore += (calc_scores[tid] - score_list[tid]).powf(2.0);
			}
			if curscore - nxscore >= 0. {
				for w in l..=l+rng {
					if g[h*H+w][1] <= 1000. {continue;}
					g[h*H+w][1] += c;
					if w-1 < W {
						g[h*H+w-1][3] += c;
					}
				}
				curscore = nxscore;
			}
			else {
				for w in l..=l+rng {
					if g[h*H+w][1] <= 1000. {continue;}
					for &tid in &record[h*H+w][1] {
						calc_scores[tid] -= c;
					}
				}
			}
		}
		for w in 0..W {
			let mut nxscore = 0.0;
			let c = if (rnd.next_int()&1) == 0 {7.5} else {-7.5};
			let rng = rnd.nextn(15) + 10;
			let l = rnd.nextn(H-rng-1)+1;
			for h in l..=l+rng {
				if g[h*H+w][0] <= 1000. {continue;}
				for &tid in &record[h*H+w][0] {
					calc_scores[tid] += c;
				}
			}
			for tid in 0..calc_scores.len() {
				nxscore += (calc_scores[tid] - score_list[tid]).powf(2.0);
			}
			if curscore - nxscore >= 0. {
				for h in l..=l+rng {
					if g[h*H+w][0] <= 1000. {continue;}
					g[h*H+w][0] += c;
					if h-1 < H {
						g[(h-1)*H+w][2] += c;
					}
				}
				curscore = nxscore;
			}
			else {
				for h in l..=l+rng {
					if g[h*H+w][0] <= 1000. {continue;}
					for &tid in &record[h*H+w][0] {
						calc_scores[tid] -= c;
					}
				}
			}
		}
    }
	/*
	let mut check_score = 0.0;
	for (&c1, &c2) in score_list.iter().zip(calc_scores.iter()) {
		check_score += (c1-c2).abs();
	}
	eprintln!("{} : {}", curscore, check_score);
	*/
}

fn main() {
	let timer = Timer::new();
	let mut rnd = XorShift32::new(0);

	let mut g = vec![vec![5000.; 4]; H*W];
	let mut parent = vec![(H, W); H*W];
	let mut s_count = vec![vec![0.1; 4]; H*W];

	let mut score_list: Vec<f64> = Vec::new();
	let mut record = vec![vec![Vec::new(); 2]; H*W];
	//mincost 1000
	let mut route = vec![];
	let mut total_count = 0.1 * 4.0 * (H*W) as f64;
	for tm in 0..Q {
		let (si, sj, ti, tj) = read!(usize, usize, usize, usize);
		if tm < 100 {
			let ret = straight_path(si, sj, ti, tj, &g, &mut parent, &s_count, total_count, &mut rnd, tm);
			if ret {
				shortest_path(si, sj, ti, tj, &g, &mut parent, &s_count, tm, &mut rnd);
			}
		}
		else {
			shortest_path(si, sj, ti, tj, &g, &mut parent, &s_count, tm, &mut rnd);
		}
		let (mut py, mut px) = (ti, tj);

		//ULDR
		let mut s = String::new();
		route.clear();
		while py != si || px != sj {
			let (ny, nx) = parent[py*H+px];
			let dir;
			if ny < py {
				s.push('D');
				dir = 2;
			}
			else if nx < px {
				s.push('R');
				dir = 3;
			}
			else if ny > py {
				s.push('U');
				dir = 0;
			}
			else{
				s.push('L');
				dir = 1;
			}
			route.push((py, px, ny, nx, dir));
			py = ny;
			px = nx;
		}

		s = s.chars().rev().collect::<String>();
		output_string(&s);


		let score = read!(f64);//shortest_cost*rand(0.9,1.1)
		score_list.push(score);

		if tm >= 100 {
			let pp = tm as f64* 30.;//if tm < 200 {3000.} else {30000.};
			let mut vals = vec![0.0; route.len()];
			let mut v_sum = 0.;
			for &r in &route {
				v_sum += g[r.2*H+r.3][r.4];
			}
			for (i, &r) in route.iter().enumerate() {
				let wt = (score / v_sum).powf(1.5);
				vals[i] = g[r.2*H+r.3][r.4] * wt;
			}
			for (i, &r) in route.iter().enumerate() {
				update_val(&mut g, &mut s_count, r.0, r.1, (r.4+2)&3, vals[i], pp);
				update_val(&mut g, &mut s_count, r.2, r.3, r.4, vals[i], pp);
				if r.4 <= 1 {record[r.2*H+r.3][r.4&1].push(tm);}
				else {record[r.0*H+r.1][r.4&1].push(tm);}
				total_count += pp * 2.;
			}
		}
		else {
			let (mut h_sum, mut v_sum) = (0., 0.);
			let (mut h_num, mut v_num) = (0., 0.);
			for &r in &route {
				if (r.4 & 1) == 1 {
					h_num += 1.;
					h_sum += g[r.2*H+r.3][r.4];
				}
				else {
					v_num += 1.;
					v_sum += g[r.2*H+r.3][r.4];
				}
			}
			let (h_ave, v_ave) = ((score / (h_sum + v_sum)).powf(1.1) * h_sum / h_num, (score / (h_sum + v_sum)).powf(1.1) * v_sum / v_num);
			//let average_score = score as f64 / route.len() as f64;

			let tot = 61. - (h_num + v_num).min(60.);
			let (h_p, v_p) = (h_num / (h_num + v_num)*tot, v_num / (h_num + v_num)*tot);
			
			for &r in &route {
				if r.4 <= 1 {record[r.2*H+r.3][r.4&1].push(tm);}
				else {record[r.0*H+r.1][r.4&1].push(tm);}
				if (r.4 & 1) == 1 {
					update_val(&mut g, &mut s_count, r.0, r.1, (r.4+2)&3, h_ave, h_p*h_p);
					update_val(&mut g, &mut s_count, r.2, r.3, r.4, h_ave, h_p*h_p);
					total_count += h_p*h_p*2.0;
				}
				else {
					update_val(&mut g, &mut s_count, r.0, r.1, (r.4+2)&3, v_ave, v_p*v_p);
					update_val(&mut g, &mut s_count, r.2, r.3, r.4, v_ave, v_p*v_p);
					total_count += v_p*v_p*2.0;
				}
			}
		}
		if tm + 1 <= 200 && (tm + 1) % 100 == 0 {
			fill_cell(&mut g, &mut s_count, tm);
			//fill_cell2(&mut g, &mut s_count, tm);
		}
		if tm+1 >= 100 && (tm+1-100) % 50 == 0 {
			fill_cell2(&mut g, &mut s_count, tm);
		}
		if tm+1 >= 100 && (tm+1-100) % 50 == 0 {//if tm+1 >= 10 && ((tm+1-10) % 20 == 0 && tm+1 <= 200 || (tm+1-10) % 50 == 0 && tm+1 > 200) {
			improve(&mut g, &mut s_count, &score_list, &record, &timer, &mut rnd);
		}
	}
	eprintln!("{}", timer.get_time());
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

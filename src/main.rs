use std::collections::HashSet;
use std::i64::MAX;

type Vec3D = (i32, i32, i32);

const MOVES: [Vec3D; 24] = [(2,1,0), (2,-1,0), (-2,1,0), (-2,-1,0), (2,0,1), (2,0,-1), (-2,0,1), (-2,0,-1), (0,2,1), (0,2,-1), (0,-2,1), (0,-2,-1),(0,1,2), (0,1,-2), (0,-1,2), (0,-1,-2),(1,0,2), (1,0,-2), (-1,0,2), (-1,0,-2),(1,2,0), (1,-2,0), (-1,2,0), (-1,-2,0)];

fn add(a: &Vec3D, b: &Vec3D) -> Vec3D {
    return (a.0 + b.0, a.1 + b.1, a.2 + b.2);
}

fn abs(a: &Vec3D) -> i32 {
    return (a.0 * a.0 + a.1 * a.1 + a.2 * a.2)
}

fn jump(a: &Vec3D, tour: &HashSet<Vec3D>) -> Option<Vec3D> {
    let mut next_sq: Vec<Vec3D> = vec![];

    for m in MOVES {
        let j = add(a, &m);

        if !tour.contains(&j) {
            next_sq.push(j);
        }
    }

    if next_sq.is_empty() {
        return None;
    }

    let mut min_sq = next_sq[0];
    for sq in next_sq.iter() {
        if abs(&min_sq) > abs(&sq) {
            min_sq = *sq;
        }
    }

    return Some(min_sq);
}

fn tour(max_iter: i64) -> Option<i64> {
    let mut tour: HashSet<Vec3D> = HashSet::new();
    let mut current: Vec3D = (0,0,0);
    let mut m = 0;
    
    loop {

        tour.insert(current);
	
        current = match jump(&current, &tour) {
            Some(x) => x,
            None => {
                println!("Knight-Bro im stuck!");
                return Some(m);
            }
        };

        if m > max_iter {
            break;
        }

        m += 1;

	
        if m % 10000 == 0 {
	   // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	   // println!("Iteration: {} \t\t At: {} {} {} \t\t ABS: {}", m, current.0, current.1, current.2, (abs(&current) as f32).sqrt());
           // Garbage collection
           let mut exclude:Vec<Vec3D> = Vec::new();
           for sq in tour.iter() {
               if is_surrounded(sq, &tour) {
                    exclude.push(*sq);
               }
           }
           for r in exclude.iter() {	        
               tour.remove(r);
	   }
	}
    }

    return None;
}


fn is_surrounded(sq: &Vec3D, tour: &HashSet<Vec3D>) -> bool {
    for m in MOVES {
        if !tour.contains(&add(&m, &sq)) {
            return false;
        }
    }
    return true;
}

fn main() {
    let max_iter = 99999;
    println!("Starting knight tour test with max_iter={}", max_iter);
    match tour(max_iter) {
        Some(m) => println!("Knight stopped at iteration step {}", m),
        None => println!("Knight did not stop lol."),
    }
}


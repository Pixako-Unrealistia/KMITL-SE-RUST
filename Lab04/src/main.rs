use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use libm::*;


#[derive(Debug, Copy, Clone)]
struct Point {
   x: f64,
   y: f64,
  }

impl Point {
    fn new() -> Point {
        Point{x: 0.0, y: 0.0}
    }
}

#[derive(Debug, Copy, Clone)]
struct Triangle {
   name:[char;2],
   p : [Point;3],
  }

//main thing to look at
fn check_triangle(t:Triangle) {
	println!("\nMytriangle\n{:?}\n",t);
    let d = t.p;
	let mut dx = d[0].x - d[1].x;
    let mut dy = d[0].y - d[1].y;
    let mut dd = dx * dx + dy * dy;
    let ab = dd.sqrt();
	println!("L1 Is {}", dd.sqrt());
    
    dx = d[1].x - d[2].x;
    dy = d[1].y - d[2].y;
    dd = dx * dx + dy * dy;
	let bc = dd.sqrt();
    println!("L2 Is {}", dd.sqrt());
    
	dx = d[2].x - d[0].x;
    dy = d[2].y - d[0].y;
    dd = dx * dx + dy * dy;
    let ac = dd.sqrt();
	
	println!("L3 Is {}", dd.sqrt());
   
	println!("{}" , acos( ( ac*ac + ab*ab - bc*bc ) / (2.0 * ac * ab) ));
	println!("{}", acos( ( ac*ac + bc*bc - ab*ab ) / (2.0 * ac * bc) ));
	println!("{}", acos( ( ab*ab + bc*bc - ac*ac ) / (2.0 * ab  * bc) ));
	let s = (ab + bc + ac ) / 2.0 ;
	println!("Area equals to {}", (s * (s - ab)*(s - bc)*(s-ac)).sqrt())
    //no reason to bother implementing redundant, nested functions for such simple task.
}

//Does its work, no need to refactor.
fn read_triangles( s:String ) -> u64 {
	print!("File contains:\n{}",  s);
        let lines:Vec<&str> = s.split('\n').collect();
	let n_lines = lines.len();
	println!("s has {} lines", n_lines );
	let mut n_tri:u64 = 0;
	for j in 0..n_lines {
		let t_set:Vec<&str> = lines[j].split(' ').collect();
		let n_tokens = t_set.len();
		if n_tokens < 6 { break; }
		println!("t_set[{}] has {} tokens", j, n_tokens );
		for k in 0..n_tokens { print!(" {}:{}", k, t_set[k]); }
		println!();
		let label = t_set[0];
		print!("T {}: ", label);
		let mut k1:usize = 1;
		let mut ps = [Point::new();3];
		for m in 0..3 { 
			print!("m = {} k1 {} {},{}, ", m, k1, t_set[k1], t_set[k1+1] );
			let x: f64 = t_set[k1].parse().unwrap(); 
			k1 = k1 + 1;
			let y: f64 = t_set[k1].parse().unwrap(); 
			print!("Point ({},{}) ", x, y );
			k1 = k1 + 1;
			// let p = Point{x:x,y:y};
			ps[m] = Point{x:x,y:y};
			}
		let namej = ['A','A'];
		let tj = Triangle{name:namej,p:ps };
		check_triangle(tj);
		n_tri += 1;
		println!();
	}
	n_tri
}

fn main() {
    // Access the file
    let path = Path::new("triangles.txt");
    let display = path.display();

       let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let mut n_t:u64 = 0;
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => { n_t = read_triangles(s); }
    }
    println!("{} triangles read", n_t);
    // `file` goes out of scope, and the "hello.txt" file gets closed
}

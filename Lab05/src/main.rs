use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
struct Point {
   x: f64,
   y: f64,
  }



#[derive(Debug, Copy, Clone)]
struct Tpose{
    origin: Point,
    base:   f64,
    height: f64,
//Not so much of a Tpose, but like a T.
}


fn time_diff_nsecs( t0:Instant ) ->  f64 {
	let duration = t0.elapsed();
	let d_nsecs = duration.as_nanos();
	d_nsecs as f64
}


  const N:usize = 20000; //Why.


fn time_access(insert: [Tpose; N]) -> f64 {
    let t1 = Instant::now();  
    let mut tempo: f64 = 0.0;
    for j in insert
    {
        tempo = tempo + j.base; //change this
    }
    let duration = time_diff_nsecs(t1);
    println!("{}",tempo);
    duration as f64;
    N as f64;
    duration/N as f64
}

fn time_access_vector(insert: Vec<Tpose>) -> f64 {
    let t1 = Instant::now(); 
    let mut tempo: f64 = 0.0;
    for j in insert
    {
        tempo = tempo + j.base; //change this 
    }
    let duration = time_diff_nsecs(t1);
    println!("{}", tempo);
    duration as f64;
    N as f64;
    duration/N as f64
}

fn time_array(n:usize) -> f64 {
  // Initialize array
  let p0 = Point{x:0.0,y:0.0};
  let tk = Instant::now();
  let mut c0 = Tpose{origin:p0, base:1.0, height:1.0};
  let mut v_array: [Tpose; N] = [c0; N];
  let durationk = time_diff_nsecs(tk);
  println!("Solution so far from sight {}", durationk);
  let t0 = Instant::now();
  for j in 0..N {
    v_array[j] = c0;
    c0.base *= 1.001;
    c0.height *= 1.001;
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to initialize array {:?}", t0, duration);
  println!("{}", time_access(v_array));
  duration as f64
}

fn time_vector(n:usize) -> f64 {
  // Initialize vector
  let p0 = Point{x:0.0,y:0.0};
  let t1 = Instant::now();
  let mut c0 = Tpose{origin:p0,base: 1.0, height:1.0};
  let mut v_objects:Vec<Tpose> = Vec::with_capacity(20000);
  println!("Solution so far from sight {}", time_diff_nsecs(t1));
  let t0 = Instant::now();
  println!("Start {:?}", t0 );
  for j in 0..N {
    v_objects.push(c0);
    c0.base *= 1.001;
    c0.height *= 1.001;
  }
  let duration = time_diff_nsecs(t0);
  println!("Start {:?} Time to initialize array {:?}", t0, duration);
//  println!("{}", time_access_vector(v_objects));
  duration as f64
}



//fn time_access_vector() -> f64 {
 //   let t0 = Instant::now();
//
    
//}

fn init_timing() {

  let p0 = Point{x:0.0,y:0.0};
  // let mut c0 = ....
  let mut v_objects:Vec<Tpose> = Vec::new();

  let t_per_array = time_array(N);
  println!("Array {} elements total {} ns mean {} ns/elem", N, t_per_array, (t_per_array)/(N as f64));
  let t_per_vector = time_vector(N);
  println!("Vector {} elements total {} ns mean {} ns/elem", N, t_per_vector, t_per_vector/(N as f64));


}

fn main() {
    println!("First attempt using 20000");
    init_timing();
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nAttempt using 20000");
    init_timing();
}

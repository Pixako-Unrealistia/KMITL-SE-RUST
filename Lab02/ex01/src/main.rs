use rand::prelude::*;


fn main() {
    let mut _a:char = 'a';
    let mut in_pt:f64 = 0.0;
    let mut out_pt:f64 = 0.0;
    let _z:char = '\u{7FFF}';
    
    const EPS:f64 = 1.0e-10; //change this to whatever you wish
    
    print!("Calculate Pi!\n");
    for _n in 1..1000000 {
       // Generate two random numbers that will be used in our equation
       let x:f64 = rand::random();
       let y:f64 = rand::random();
       // Calculate distance of (x,y) from origin using MATH
       let d2:f64 = (x*x + y*y).sqrt();
       // Is this point inside the unit circle?
      if d2 <= 1.0
       { 
            in_pt += 1.0;
       }
            out_pt += 1.0; 
       // Calculate estimate of pi every 10000 trials, change this to something else if you wish.
       if (_n % 10000) == 0 {
        let pi_est = (in_pt/out_pt) * 4.0;
        let diff = core::f64::consts::PI - pi_est;
        print!("{} {} {} ", _n, x, y);
        println!(" pi = {} {}", pi_est, diff);
	// Is the difference too large?
        let out_of_range = diff > EPS;
        let ok = diff > 0.0;
        let mut abs_diff = f64::abs(diff);
        print!("{} {} {}\n", out_of_range, ok, abs_diff);
        abs_diff = diff.abs();
        print!("{} {} {}\n", out_of_range, ok, abs_diff);
       }
	    
    }
}


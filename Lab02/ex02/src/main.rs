fn main() {
    //loop{
    let mut _a:char = 'a';
    let mut coun:f64 = 0.0;
    let mut keep:f64 = 0.0;
    const EPS:f64 = 1.0e-10; //this one is the Tolerance, change it to whatever you wished.
    
    print!("OWO!\n");
    loop {
        //generate a random number
        let x:f64 = rand::random();
        keep += x;
        coun += 1.0;
       
        let avg = keep/coun; //calculating the thing every line is ineffecient. Thank god we are using Rust and not something like python
        let diff = (avg - 0.5).abs();
        if diff <= ESP
        {
        println!("{} {} {} Tolerance = {}", coun, avg, diff, ESP);
        break;
        }
        }
        
   // }
    }


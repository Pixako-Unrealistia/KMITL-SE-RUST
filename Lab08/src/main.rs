
use libm::*;
use rand::Rng;
use std::io;


const NOTMAGICNUMBER: usize = 361; 
//const NOTMAGICTEN: usize = 11;
const TESTSIZE: usize = 20000 ; // > around 50000 = overflow :I
const N:usize = 4;

fn lut_sin(lut: [f64; NOTMAGICNUMBER], x:f64) -> f64
{
    let mut screen:f64 = x;
//    println!("{}", screen); 
    while screen < 0.0 //wasted so many cycles on this garbage
    {
        screen = 360.0 + screen;
    }
    while screen > 360.0
    {
        screen = screen - 360.0;
    }
    let down:f64 = lut[screen.floor() as usize];
    let up:f64 = lut[screen.ceil() as usize];
    return down + ((up - down) * (screen - screen.floor()) / (screen.floor() - screen.ceil()))
}



fn print_result(trigo_types: String, lut: [f64; NOTMAGICNUMBER], testcases: [f64; TESTSIZE])
{
	println!("Amount of test cases: {}\nTesting for {}", TESTSIZE, trigo_types);
	let mut accumu = 0.0;
	println!("{}", trigo_types);
	if trigo_types == "Sine"
	{
		for x in testcases
		{
		let asnnd:f64 = sin((x).to_radians()) - lut_sin(lut, x);
		println!("src {:.6} function {:.6} difference: {:.6} at {:.6} degree", sin((x).to_radians()) , lut_sin(lut, x), asnnd ,x); 
		accumu = accumu + asnnd.abs(); //maintaing result's intregety by absoluting the val.
		
		if asnnd.abs() > 0.01
        {
            eprintln!("{} {}", asnnd, x);
        }
		}	
        
	}
	
	else if trigo_types == "Cosine"
	{
		for x in testcases
		{
		let asnnd:f64 = cos((x).to_radians()) - lut_sin(lut, 90.0 - x);
		println!("src {:.6} function {:.6} difference: {:.6} at {:.6} degree", cos((x).to_radians()) , lut_sin(lut, 90.0 - x), asnnd ,x); 
		accumu = accumu + asnnd.abs(); //maintaing result's intregety by absoluting the val.
		}			
	}

    else if trigo_types == "Tangent"
    {
		for x in testcases
		{
		let asnnd:f64 = tan((x).to_radians()) - ((lut_sin(lut, x))/ (lut_sin(lut, 90.0 - x)));
		println!("src {:.6} function {:.6} difference: {:.6} at {:.6} degree",tan((x).to_radians()) ,((lut_sin(lut, x))/ (lut_sin(lut, 90.0 - x))),asnnd ,x); 
		accumu = accumu + asnnd.abs(); //maintaing result's intregety by absoluting the val.
		}		    
    }

    else if trigo_types == "Cosecant"
    {

		for x in testcases
		{
		let asnnd:f64 = (1.0 / sin((x).to_radians())) - ( 1.0 / (lut_sin(lut, x)));
		println!("src {:.6} function {:.6} difference: {:.6} at {:.6} degree", 1.0/ sin((x).to_radians()), 1.0 / lut_sin(lut, x), asnnd ,x); 
            accumu = accumu + asnnd.abs(); //maintaing result's intregety by absoluting the val.
        /*if asnnd.abs() > 10.0
        {
            eprintln!("{} {}", asnnd, x);
        }*/

        }		    
    }
	    else if trigo_types == "Secant"
    {

		for x in testcases
		{
		let asnnd:f64 = tan((x).to_radians()) - ((lut_sin(lut, x))/ (lut_sin(lut, 90.0 - x)));
		println!("src {:.6} function {:.6} difference: {:.6} at {:.6} degree", 1.0/ sin((x).to_radians()), 1.0 / lut_sin(lut, x), asnnd ,x); 
            accumu = accumu + asnnd.abs(); //maintaing result's intregety by absoluting the val.
        /*if asnnd.abs() > 10.0
        {
            eprintln!("{} {}", asnnd, x);
        }*/

        }		    
    }
	
	    else if trigo_types == "Cotangent"
    {
		for x in testcases
		{
		let asnnd:f64 = (1.0 / sin((x).to_radians())) - ( 1.0 / (lut_sin(lut, x)));
		println!("src {:.6} function {:.6} difference: {:.6} at {:.6} degree", 1.0/ sin((x).to_radians()), 1.0 / lut_sin(lut, x), asnnd ,x); 
            accumu = accumu + asnnd.abs(); //maintaing result's intregety by absoluting the val.
        /*if asnnd.abs() > 10.0
        {
            eprintln!("{} {}", asnnd, x);
        }*/

        }
    }
	
	println!("Accumulated -> {} Mean diff -> {}", accumu, accumu / TESTSIZE as f64);	
	
}

fn hard_input() -> String
{
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("error: unable to read user input");
	
	return input
}


fn main() {
    let mut lut: [f64; NOTMAGICNUMBER] = [0.0; NOTMAGICNUMBER];
    for c in 0..NOTMAGICNUMBER
    {
        lut[c] = sin((c as f64).to_radians());
    }

    let mut testcases: [f64; TESTSIZE] = [0.0; TESTSIZE];
    
    for c in 0..TESTSIZE
    {
        testcases[c] = rand::thread_rng().gen_range(-360.0..=360.0);
    }



//    println!("Please input 'Sine' 'Cosine' 'Tangent' 'Cosecant' 'Secant' or 'Cotangent'");
//	print_result(hard_input(), lut, testcases);
// These two lines doesn't work for some unholy reason.


//	print_result("Cosecant".to_string(), lut, testcases);
//	print_result("Tangent".to_string(),lut, testcases);
//	print_result("Sine".to_string(), lut, testcases);
//	print_result("Cosine".to_string(),lut, testcases);
	print_result("Tangent".to_string(),lut, testcases);
//	print_result("Secant".to_string(),lut, testcases);
//	print_result("Cotangent".to_string(),lut, testcases);

//    println!("{} {}", lut_sin(lut, 260.0 as f64), cos((-190.0 as f64).to_radians()))    ;
}



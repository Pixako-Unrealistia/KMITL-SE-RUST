

//How to run!
//>cargo run > something.txt
//Yes, you have to manually pipe the result :U


use std::fs;
use std::error::Error;
use std::time::{Duration, Instant};

//Making a struct because the doc told me to.

#[derive(Debug, Copy, Clone)]
struct Coerced{
    achar : char,
    count : f64,
    frequency : f64,
}

//I know it's kinda a great sin to do this, but can't think of eaiser way since you can't return
//multiple types from function 

static mut largest: f64 = 0.0;
static mut largestX: char = 'n';
static mut postcleaned: usize  = 0;




//Goes through Thai unicodes. But nevermind.
/*
fn parser(text:String) {
    let mut storage: Vec<Coerced> =  Vec::new(); //unused for now
    let mut tempstr = String::new();
    let mut tempint: f64 = 0.0;
    let mylen : f64 = text.chars().count() as f64;
    //    println!("{}", text);
    for x in 3585..=3675{
        tempstr = from_u32(x).unwrap().to_string();
        tempint = text.matches(&tempstr).count() as f64;
        if tempint != 0.0
        {
            println!("{} {} {}",tempstr,tempint, tempint/mylen);

        }
        //storage.push(something); 
    }
}
*/

//100% totally definitely very optimised..... not...:(
//Basically, it works by finding all instances of x in input
//then shove them into vector before buring all the instances of x
//process repeat til there aren't anything left in the string.
//should be a lot slower considering the usage of count matches and counts.
fn verysadfunction(text: String) -> Vec<Coerced>
{
    let mut tempstr = text;
    //clean the input a bit.
    tempstr = tempstr.replace(" ", "");
    tempstr = tempstr.replace("\n", "");

    let notlen = tempstr.chars().count();
    unsafe {postcleaned = notlen};
    
    //vector with minimum capacity without any genius algorithm
    let mut verysadvec: Vec<Coerced> = Vec::with_capacity(notlen);
    let mut tempchr = Coerced{achar: 'K', count: 0.0, frequency: 0.0};
    


    loop //loop through every chars.
    {
    for x in tempstr.chars() //just want to access the value without intention to loop.
    {
        tempchr.achar = x.clone();
        tempchr.count = tempstr.matches(x).count() as f64;
        tempchr.frequency = (tempchr.count / notlen as f64) * 100.0;
        verysadvec.push(tempchr);
        really(tempchr.count, tempchr.achar); //check if the value is the most frequent
        break;
    }
    tempstr = tempstr.replace(tempchr.achar, "");
    if tempstr.chars().count() == 0
    {
        break;
    }
    }
    return verysadvec;
}

//named after how rust try it damnest to not let me use global variables.
fn really(n : f64 ,c : char)
{
    unsafe
    {
    if n > largest
    {
        largestX = c;
        largest = n;
    }
    }
}

//As the name suggests 
fn ft_read() -> Result<String, Box<dyn std::error::Error>>
{
    print!("Text to be processed ");
    let contents:String = fs::read_to_string("./readme.txt")?; //change filename here
    eprintln!("Initial size ({})", contents.len());
    println!("\n{}\n", contents);
    Ok(contents)
}


fn summary(sum : Vec<Coerced>, t2: Duration, tf: Duration, feq : f64)
{
    unsafe{
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("Summary\nTime taken to execute looping function is {:?}", t2);
    println!("Time taken to print out the data {:?}", tf);
    println!("Excluding spacebar and newlies, total {} chars processed.", postcleaned);
    println!("Most frequent character is '{}' with ({}) recursions. Being {}%", largestX, largest ,((largest/postcleaned as f64) * 100.0));
    
    println!("Accuracy of combined frequency: {}%", feq)
    }
}

fn main() {
    let mut checker: f64 = 0.0;
    let t0 =  Instant::now();
    let absolution:Vec<Coerced> = verysadfunction(ft_read().unwrap());
    let t1 = Instant::now();
    let t2 = t1-t0;
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("Key        Count       Freq");
    let t3 = Instant::now();
    for x in &absolution
    {
        println!("{}       {}        {}%", x.achar, x.count, x.frequency);
        checker += x.frequency;
    }
    let t4 = Instant::now();
    summary(absolution,t2, t4-t3,checker);
}
    //    parser(read());

//    parser(_text);


    //println!("{}", _text);
//  loop {
//    let mut text = read_text_line();
//    eprintln!("Text [{}]", text );
//    if text.len() == 0 { break; }
//    else {
//      let nc = unpack_text_line( text );
//      ix += 1;
//    }
//  }
//  eprintln!("\n{} lines processed", ix );




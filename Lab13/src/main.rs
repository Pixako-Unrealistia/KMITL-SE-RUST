use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

//Changable consts
const DATA_SIZE :usize = 48;
const FILE_PATH : &str = "./GPSB.csv";
const AMOUNT_OF_ELEMENTS : usize = 5;
const RONNDED: usize = 4;

//Wouldn't recommend changing consts
const SEARCH_RANGE : usize = 10; 
const EMPTY_STRING : String = String::new();

//struct
#[derive(Debug,Copy,Clone)]
struct gps
{
    coordx : f64,
    coordy : f64,
}

//functions
fn read_and_store() -> [gps; DATA_SIZE]
{
	let file = File::open(FILE_PATH).unwrap();
    let raw_data = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join(",")
        //.replace("\n", ",")
        .replace(" ", "");

    println!("{raw_data}");
    let mut data : Vec<&str> = raw_data.split(",").collect();

    let mut processed_array : [gps;DATA_SIZE] = [gps{coordx : 0.0, coordy : 0.0}; DATA_SIZE];
    
    for i in 0..DATA_SIZE
    {
        
        processed_array[i] = gps
        {
        coordx : data[0].parse::<f64>().unwrap(), //[0] being the first index from data
        coordy : data[1].parse::<f64>().unwrap(), //[1] being the second index from data
        };
        
        println!("{} {}", data[0], data[1]);
        for k in 0..=AMOUNT_OF_ELEMENTS
        {
            data.remove(0);
        }
    }


    processed_array    
}

fn find_mean(processed_array : [gps;DATA_SIZE]) -> (f64,f64)
{
    let mut x_summation : f64 = 0.0;
    let mut y_summation : f64 = 0.0;
    for i in 0..DATA_SIZE
    {
        x_summation += processed_array[i].coordx;
        y_summation += processed_array[i].coordy;
    }
    (x_summation / (DATA_SIZE as f64) , y_summation / (DATA_SIZE as f64))
}

fn find_max(processed_array : [gps;DATA_SIZE]) -> (f64, f64)
{
    let mut x_max : f64 = 0.0;
    let mut y_max : f64 = 0.0;
    for i in 0..DATA_SIZE
    {
        if processed_array[i].coordx > x_max
        {
            x_max = processed_array[i].coordx;
        }
        if processed_array[i].coordy > y_max
        {
            y_max = processed_array[i].coordy;
        }
    }
    (x_max, y_max)
}


fn find_min(processed_array : [gps;DATA_SIZE]) -> (f64, f64)
{
    let mut x_min : f64 = processed_array[0].coordx;
    let mut y_min : f64 = processed_array[0].coordy;
    for i in 0..DATA_SIZE
    {
        if processed_array[i].coordx < x_min
        {
            x_min = processed_array[i].coordx;
        }
        if processed_array[i].coordy < y_min
        {
            y_min = processed_array[i].coordy;
        }
    }
    (x_min, y_min)
}


fn standard_d(processed_array : [gps;DATA_SIZE], x_mean : f64, y_mean : f64) -> (f64, f64)
{
    let mut x_summation : f64 = 0.0;
    let mut y_summation : f64 = 0.0;
    for i in 0..DATA_SIZE - 1
    {
        x_summation += (processed_array[i].coordx - x_mean).powf(2.0); //inaccordance to the equation
        y_summation += (processed_array[i].coordy - y_mean).powf(2.0);
    }
    ((x_summation / (DATA_SIZE as f64)).sqrt(), (y_summation / (DATA_SIZE as f64)).sqrt())
}

fn convert_standard_d(standard_d : (f64, f64)) -> (f64, f64)
{
    let mut x_standard_d : f64 = standard_d.0;
    let mut y_standard_d : f64 = standard_d.1;
    x_standard_d *= 111139.0;
    y_standard_d *= 107963.0;
    (x_standard_d, y_standard_d)
}


//I mean if someone copy this they really need to reevaluate themselves xD - Sorawis
fn make_histrogram(processed_array : [gps;DATA_SIZE])
{
    let mut x_range : [String; DATA_SIZE] = [EMPTY_STRING; DATA_SIZE];
    let mut y_range : [String; DATA_SIZE] = [EMPTY_STRING; DATA_SIZE];

    for x in 0..DATA_SIZE
    {
        x_range[x] = processed_array[x].coordx.to_string();
    }

    for y in 0..DATA_SIZE
    {
        y_range[y] = processed_array[y].coordy.to_string();
    }

    let key_x = x_range.iter().unique().into_iter().collect::<Vec<_>>();
    let key_y = y_range.iter().unique().into_iter().collect::<Vec<_>>();
    
    println!("X values");
    for c in key_x 
    {
        let mut countette = 0;
        for d in x_range.iter()
        {
            if c == d
            {
                countette += 1;
            }
        }
        print!("{} ", c);
        printast(countette);
    }

    println!("Y values");
    for c in key_y
    {
        let mut countette = 0;
        for d in y_range.iter()
        {
            if c == d
            {
                countette += 1;
            }
        }
        print!("{} ", c);
        printast(countette);
    }
    //Due to the earlier notice, while I believe checking min max might be the best solution, I'm too afraid of doing so in case the stars aligned and it matched with someone's. 
}

fn printast(counter : usize)
{
    for i in 0..counter
    {
        print!("*");
    }
    print!("\n")
}

fn main() {
    let processed_array = read_and_store();
    println!("Mean {:?}", find_mean(processed_array));
    println!("Max {:?}", find_max(processed_array));
    println!("Min {:?}", find_min(processed_array));
    println!("SD {:?}", standard_d(processed_array, find_mean(processed_array).0, find_mean(processed_array).1));
    println!("Error {:?}", convert_standard_d(standard_d(processed_array, find_mean(processed_array).0, find_mean(processed_array).1)));
    println!("Histrogram of the entire thing");
    make_histrogram(processed_array);
    //println!("Histrogram of SD");
    //make_histrogram(standard_d(processed_array, find_mean(processed_array).0, find_mean(processed_array).1));
}

use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

//Changable consts
const DATA_SIZE :usize = 48;
const FILE_PATH : &str = "./GPSA.csv";
const AMOUNT_OF_ELEMENTS : usize = 5;
const RONNDED: usize = 4;

//Wouldn't recommend changing consts
const SEARCH_RANGE : usize = 10; 


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
        coordx : data[0].parse::<f64>().unwrap(),
        coordy : data[1].parse::<f64>().unwrap(),
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
        x_summation += (processed_array[i].coordx - x_mean).powf(2.0);
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


//please fix me ;-;
fn make_histrogram(processed_array : [gps;DATA_SIZE])
{
    let mut x_histrogram : [usize; 10] = [0; SEARCH_RANGE];
    let mut y_histrogram : [usize; 10] = [0; SEARCH_RANGE];
    for i in 0..DATA_SIZE
    {
        let x_index : usize = format!("{:.5}",processed_array[i].coordx).chars().last().unwrap().to_string().parse::<usize>().unwrap();
        let y_index : usize = format!("{:.5}",processed_array[i].coordy).chars().last().unwrap().to_string().parse::<usize>().unwrap();
        x_histrogram[x_index] += 1;
        y_histrogram[y_index] += 1;
    }
    println!("X histrogram");
    for i in 0..10
    {
        println!("{}: {}", i, x_histrogram[i]);
    }
    println!("Y histrogram");
    for i in 0..10
    {
        println!("{}: {}", i, y_histrogram[i]);
    }
}

fn main() {
    let processed_array = read_and_store();
    println!("Mean {:?}", find_mean(processed_array));
    println!("Max {:?}", find_max(processed_array));
    println!("Min {:?}", find_min(processed_array));
    println!("SD {:?}", standard_d(processed_array, find_mean(processed_array).0, find_mean(processed_array).1));
    println!("Error {:?}", convert_standard_d(standard_d(processed_array, find_mean(processed_array).0, find_mean(processed_array).1)));
    make_histrogram(processed_array);
    //println!("Hello, world!");
}

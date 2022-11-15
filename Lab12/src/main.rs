use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use json;
use std::fs::File;
use serde::{Deserialize};
//to derive serde, it would appear that editing features in depedency is required
//therefore: I've submitted the cargo.toml too.

//Constants

const DEG : char = '\u{00B0}'; //The DEGree thing
const DATA_NAME : &str = "roles";
const JOB_NAME : &str = "job";
const FILE_PATH : &str =  "data.json";

//Edit data_size here!
const DATA_SIZE : usize = 3; 
//File's seperator
const SEPERATOR : &str = ",";

//Json's default seperator DO NOT CHANGE.
const DEFAULT_SEPERATOR : &str = ",";

//Enum Definition
#[derive(Debug,Clone)]
enum Types{
	EnumFortuneteller(Fortune_Teller),
	EnumPolitician(Politician),
	EnumFreelancer(Freelancer),
	
	
}


//Trait Definitions

trait Employee
{
	fn get_name(&self) -> String;
	fn get_age(&self) -> usize;
	fn get_location(&self) -> String;
	fn get_special(&self) -> String;
}

//Struct Definitions

#[derive(Deserialize,Debug, Copy, Clone)]
struct Coordinate
{
	coordx:f64,
	coordy:f64,
}

#[derive(Deserialize,Debug, Clone)]
struct Fortune_Teller
{
	name: String,
	age: usize,
	location: Coordinate,
	moneyswindled: f64,
}

#[derive(Deserialize,Debug, Clone)]
struct	Politician
{
	name: String,
	age: usize,
	location: Coordinate,
	party: String,
}

#[derive(Deserialize,Debug, Clone)]
struct	Freelancer
{
	name: String,
	age: usize,
	location: Coordinate,
	jobsdone: Vec<String>,
}

//Implementations
impl Employee for Fortune_Teller {
	fn get_name(&self) -> String
	{
		format!("{}", self.name)
	}
	fn get_age(&self) -> usize
	{
		self.age
	}
	fn get_location(&self) -> String
	{
		getcoordinate(self.location)
	}
	fn get_special(&self) -> String
	{
		format!("{}", self.moneyswindled)
	}
}


impl Employee for Politician {
	fn get_name(&self) -> String
	{
		format!("{}", self.name)
	}
	fn get_age(&self) -> usize
	{
		self.age
	}
	fn get_location(&self) -> String
	{
		getcoordinate(self.location)
	}
	fn get_special(&self) -> String
	{
		format!("{}", self.party)
	}
}


impl Employee for Freelancer {
	fn get_name(&self) -> String
	{
		format!("{}", self.name)
	}
	fn get_age(&self) -> usize
	{
		self.age
	}
	fn get_location(&self) -> String
	{
		getcoordinate(self.location)
	}
	fn get_special(&self) -> String
	{
		let mut output: String = "".to_owned();
		for parser in &self.jobsdone
		{
			output.push_str(&parser);
			output.push(',');
		}
		output
	}
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.coordx, self.coordy)
    }
}

//Functions



//This part is unused. But implemented nontheless.
fn getcoordinate(mut k:Coordinate) -> String
{
	let mut first:char = ' ';
	let mut second:char = ' ';

	if k.coordx >= 0.0 {first = 'N'}
	else {first = 'S'; k.coordx = k.coordx*-1.0;}

	if k.coordy >= 0.0 {second = 'E'}
	else {second = 'W'; k.coordy = k.coordy*-1.0;}

	format!("{:.3}{DEG} {first}, {:.3}{DEG} {second}", k.coordx, k.coordy)
}

//This function reads json file then store values in a vector
fn read_and_store() -> Vec<Types>
{
	let file = File::open(FILE_PATH).unwrap();
    let raw_data = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("");
	
	//change seperator to default seperator so it can be parsed as json.
	let cleaned_data = str::replace(raw_data.as_str(), DEFAULT_SEPERATOR, SEPERATOR);
	
	
    let data = json::parse(cleaned_data.as_str()).unwrap();
	let mut new_data: Vec<Types> = Vec::new();
	for x in 0..DATA_SIZE{
		if data[DATA_NAME][x][JOB_NAME] == "Fortune_Teller"
		{
			let extract : String = data[DATA_NAME][x].to_string();
			let ft : Fortune_Teller = serde_json::from_str(&extract).unwrap();
			new_data.push(Types::EnumFortuneteller(ft));
		}
		else if data[DATA_NAME][x][JOB_NAME] == "Politician" 
		{			
			let extract : String = data[DATA_NAME][x].to_string();
			let ft : Politician = serde_json::from_str(&extract).unwrap();
			new_data.push(Types::EnumPolitician(ft));
		}
		else if data[DATA_NAME][x][JOB_NAME] == "Freelancer"
		{
			let extract : String = data[DATA_NAME][x].to_string();
			let ft : Freelancer = serde_json::from_str(&extract).unwrap();
			new_data.push(Types::EnumFreelancer(ft));
		}
	}
	new_data
}

fn get_average_age(some_vec:Vec<Types>) -> f64
{
	let mut average:usize = 0;
		for k in some_vec
		{
				match &k
				{
						Types::EnumFortuneteller(Fortune_Teller) => average = average + Fortune_Teller.age,
						Types::EnumPolitician(Politician) => average = average + Politician.age,
						Types::EnumFreelancer(Freelancer) => average = average + Freelancer.age,
					
				}
		}
		average as f64 /DATA_SIZE as f64
}


fn main() {
	let new_data = read_and_store();
	//println!("{:?}", new_data[0]);
	println!("{:.4}",get_average_age(new_data));
	//println!("Cordme {}", cordme);

}

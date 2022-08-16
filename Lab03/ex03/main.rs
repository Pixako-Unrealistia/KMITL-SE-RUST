
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;

fn write_my_name(my_name:String, s_id:i64) -> std::io::Result<()>{
	let mut f = File::create("name1.txt")?;
	let utc: DateTime<Utc> = Utc::now();
	let year = utc.year();
	// Make a string to add to your file
	// build the string with format
	let contents = format!("{} {}\n{} {}", my_name, s_id, utc, year);
	//Print the contents!
    println!("Contents <<{}>>", contents );
	// Write the content to a file
    write!(f,"{}", contents);
    Ok(())
}

fn main() {
    println!("\nVersion 1");
	let my_name = String::from("\u{0E2A}\u{0E23}\u{0E27}\u{0E34}\u{0E28}");
	println!("My name is {}", my_name);
	let s_id:i64 = 65011544;
    write_my_name(my_name, s_id).unwrap();
}

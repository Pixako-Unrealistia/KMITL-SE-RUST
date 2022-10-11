use std::fs;
use std::error::Error;
use std::time::{Duration, Instant};
use std::io::*;


#[derive(Debug, Copy, Clone)]
enum Languages
{
	Kata_hira,
	Thai,
	Laos,
	Ascii,
	Latineqsue,
	Chinese,
	Burmese,
	Korean,
	Accentedlatin,
	Others,
}

#[derive(Debug, Copy, Clone)]
struct	Hardstruct
{
	langtype : Languages,
	unistart : u32,
	uniend : u32,
	count : u32,
}



const LANGUAGESCOUNT : usize =  9;

fn hard_input() -> String
{
	let mut input = String::new();
	println!("Enter file name please?");
	stdin()
		.read_line(&mut input)
		.expect("error: unable to read user input");
	
	return input
}

fn ft_read() -> String
{
    print!("Text to be processed ");
  
    let contents = fs::read_to_string("./WhatLanguage.txt")
        .expect("Should have been able to read the file");
	eprintln!("Initial size ({})", contents.len());
    println!("\n{}\n", contents);
	return contents
}

impl Hardstruct
{
	fn up(&mut self)
	{
			self.count += 1
	}
	
	fn zero(&mut self)
	{
			self.count = 0
	}
}

fn seek(c: char) ->  Languages
{
    unsafe{
	    for mut x in 0..LANGUAGESCOUNT
	    {
		    if c as u32 >= tablechan[x].unistart && c as u32 <= tablechan[x].uniend
		    {
		    		tablechan[x].up();
		    		return tablechan[x].langtype
		    }
	    }
	    return Languages::Others

    }
}


fn cleanse() 
{
    unsafe{
	    for x in 0..LANGUAGESCOUNT
	    {
		    tablechan[x].zero();
	    }
    }
}


fn printme()
{
	unsafe
	{
	print!("Line:{} ", globalnum);
	for x in tablechan
	{
			if x.count != 0
			{
					print!("{:?}({:?}) " , x.langtype ,x.count);
			}
	}
	println!("");
	}
}
static mut globalnum: u32 = 1;
static mut tablechan : [Hardstruct; LANGUAGESCOUNT] = [
		Hardstruct{langtype: Languages::Kata_hira, unistart: 0x3040, uniend: 0x309F, count : 0},
        Hardstruct{langtype: Languages::Latineqsue, unistart: 0x0041, uniend: 0x007A, count : 0},
        Hardstruct{langtype: Languages::Chinese, unistart: 0x4E00, uniend: 0x9FFF, count : 0},
        Hardstruct{langtype: Languages::Accentedlatin, unistart: 0x00C0, uniend: 0x00FF, count : 0},
        Hardstruct{langtype: Languages::Burmese, unistart: 0x1000, uniend: 0x109F, count : 0},
        Hardstruct{langtype: Languages::Korean, unistart: 0xAC00, uniend: 0xD7AF, count : 0},
        Hardstruct{langtype: Languages::Thai, unistart: 0x0E00, uniend: 0x0E7F, count : 0},
		Hardstruct{langtype: Languages::Laos, unistart: 0x0E80, uniend: 0x0EFF, count : 0},
		Hardstruct{langtype: Languages::Ascii, unistart: 0x0020, uniend: 0x007E, count : 0},
	];

fn main() {
	unsafe
    {
		let unfortunette: String = ft_read();
		for x in unfortunette.chars()
		{
			if x == '\n'
			{
				printme();
				cleanse();
				globalnum += 1;
			}	
			else
			{
				seek(x);
			}
		}
	}
}

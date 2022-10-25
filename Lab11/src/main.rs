use std::io;

#[derive(Debug)]
enum phone_types{

local,
international,
likely_scammer,
special,

}

#[derive(Debug)]
struct TelNo { 
    Number: String, 
    types: phone_types,
}

impl TelNo
{
		fn specialme(&mut self)
		{
			self.types = phone_types::special;
		}
}


#[derive(Debug)]
struct Credit { 
    bank_code: String, 
    typed_num: String,
}

//Constants of numbers
const nine:u32 = 9;
const ten:u32 = 10;
const eleven:u32 = 11;
const twelvth:u32 = 12;

//Ascii comparator
const first_comparer = '0';
const second_comparer = '9';

//default country code currently being Thailand.
const first_order = '6';
const second_order = '6';


fn print_telNo( tn: TelNo ) {
    if  tn.Number != "0".to_string()
    {
        print!("Tel number {} Type: {:?}", tn.Number, tn.types);
        println!(" is valid");
    }
    else
    {
        println!("Input value is NOT valid");

    }
}

fn read_text_line() -> String {
  eprint!("Enter a telephone number ");
  let mut buffer = String::new();
  let result = io::stdin().read_line(&mut buffer);
  eprintln!("Buffer read ({}) [{}]", buffer.len(), buffer );
  buffer
}



fn	proc_card_line (s : String) -> Credit {
	let next_string: String = String::new();
	let truecounter: u64 = 0;
	let fourfirst: String = "".to_owned();
	let eightlast: String = "".to_owned();
	
	for ch in s.chars()
	{
		if (ch  >= first_comparer) && (ch <= second_comparer)	
		{
			if truecounter < 4
			{
				fourfirst.push(ch);
			}
			else
			{
				eightlast.push(ch);
			}
			truecounter += 1
		}
	}
	
	if (truecounter != twelvth)
	{
		return (Credit{bank_code: "0".to_string(),typed_num: "0".to_string(),});
	}
	return (Credit{bank_code: fourfirst ,typed_num:eightlast ,});

fn print_Card( tn: Credit ) {
    if  tn.Number != "0".to_string()
    {
        print!("Tel number {} Type: {:?}", tn.bank_code, tn.typed_num);
        println!(" is valid");
    }
    else
    {
        println!("Input value is NOT valid");

    }
}

fn proc_text_line( s: String ) -> TelNo {
  let next_string: String = String::new();
  let mut truecounter: u64 = 0;
  let mut truenumber: String = "".to_owned(); 
  let mut supercoolmarker: u8 = 0;
  let mut isinter: u8 = 0;
  //let tn = TelNo{ dummy : 99 };

  for ch in s.chars() {
    if truecounter == 0 && ch == '+' 
    {
        isinter = 1;
    }
    if (ch  >= '0') && (ch <= '9')
        {
            truenumber.push(ch);
            truecounter += 1;
        }
    else if (truecounter > twelvth)
        {
            supercoolmarker = 1;
            break
        }
  }

	eprintln!("Char processed {} {}",  truecounter, truenumber);
	

	    if ((truecounter == nine || truecounter == ten || truecounter == eleven) && supercoolmarker == 1)
    {
        return TelNo {    
            Number: truenumber, 
            types: phone_types::special,
        };
    }


	+
	if  (truenumber.chars().nth(0).unwrap() == first_order && truenumber.chars().nth(0).unwrap() == second_order)
	{
		isinter = 0;
	}
	
	let tn = TelNo{Number: "0".to_string(), types: phone_types::local};
    if (isinter == 1 && truecounter == 12 && truenumber.chars().nth(0).unwrap() == '6' && truenumber.chars().nth(1).unwrap() == '5' && truenumber.chars().nth(2).unwrap() == '7')
    {
        return TelNo {    
            Number: truenumber, 
            types: phone_types::likely_scammer,
        };
    }
	
    if ((truecounter == eleven || truecounter == ten || truecounter == twelvth) && isinter != 1)
    {
        return TelNo {    
            Number: truenumber, 
            types: phone_types::local,
        };
    }
    if ((truecounter == eleven || truecounter == twelvth)&& isinter == 1)
    { 
        return TelNo {    
            Number: truenumber, 
            types: phone_types::international,
        };
    }
    tn  
}

fn main() {
  println!("Telephone numbers");
  let mut cnt: u64 = 0;
 
  // Check each phone number
  loop {
    let mut text = read_text_line();
    eprintln!("Text [{}]", text );
    if text.len() < 3 { break; }
    else {
      let nc = proc_text_line( text );
      cnt += 1;
      print_telNo(nc);
    }
  }
  eprintln!("\n{} tel nos processed", cnt );
  


	//credit part
	loop {
    let mut text = read_text_line();
    eprintln!("Text [{}]", text );
    if text.len() < 3 { break; }
    else {
      let nc = proc_card_line( text );
      cnt += 1;
      print_Card(nc);
    }
  }

  
  
}

use std::io;

pub fn sign64 () -> i64
{
    println!("Enter integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i64 = string.trim().parse().unwrap();

    string

}

pub fn sign32 () -> i32
{
    println!("Enter integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i32 = string.trim().parse().unwrap();

    string

}

pub fn sign16 () -> i16
{
    println!("Enter integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i16 = string.trim().parse().unwrap();

    string

}

pub fn sign8 () -> i8
{
    println!("Enter integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i8 = string.trim().parse().unwrap();

    string

}
pub fn usizeint () -> usize
{
    println!("Enter integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : usize = string.trim().parse().unwrap();

    string

}
 pub fn float64 () -> f64
{
    println!("Enter float");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : f64 = string.trim().parse().unwrap();

    string
}
pub fn float32 () -> f32
{
    println!("Enter float");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : f32 = string.trim().parse().unwrap();

    string
}

pub fn chara() -> char
{
    println!("Enter char");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : char = string.trim().parse().unwrap();

    string
}

pub fn str()->String
{
    println!("Enter string");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    
    let string = String::from(string.trim());

    string
}

pub fn unsign32 () -> u32
{
    println!("Enter unsigned integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u32 = string.trim().parse().unwrap();

    string

}
pub fn unsign64 () -> u64
{
    println!("Enter unsigned integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u64 = string.trim().parse().unwrap();

    string

}

pub fn unsign16 () -> u16
{
    println!("Enter unsigned integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u16 = string.trim().parse().unwrap();

    string
}

pub fn unsign8 () -> u8
{
    println!("Enter unsigned integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u8 = string.trim().parse().unwrap();

    string

}

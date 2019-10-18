use std::io;

pub fn int () -> i64
{
    println!("Enter integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i64 = string.trim().parse().unwrap();

    string

}
 pub fn float () -> f64
{
    println!("Enter float");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : f64 = string.trim().parse().unwrap();

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

pub fn unint () -> u32
{
    println!("Enter unsigned integer");
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u32 = string.trim().parse().unwrap();

    string

}
use std::io;

pub fn sign64 () -> i64
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i64 = string.trim().parse().unwrap();

    string

}

pub fn sign32 () -> i32
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i32 = string.trim().parse().unwrap();

    string

}

pub fn sign16 () -> i16
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i16 = string.trim().parse().unwrap();

    string

}

pub fn sign8 () -> i8
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : i8 = string.trim().parse().unwrap();

    string

}
pub fn usizeint () -> usize
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : usize = string.trim().parse().unwrap();

    string

}
 pub fn float64 () -> f64
{
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : f64 = string.trim().parse().unwrap();

    string
}
pub fn float32 () -> f32
{
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : f32 = string.trim().parse().unwrap();

    string
}

pub fn chara() -> char
{
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : char = string.trim().parse().unwrap();

    string
}

pub fn str()->String
{
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    
    let string = String::from(string.trim());

    string
}

pub fn unsign32 () -> u32
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u32 = string.trim().parse().unwrap();

    string

}
pub fn unsign64 () -> u64
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u64 = string.trim().parse().unwrap();

    string

}

pub fn unsign16 () -> u16
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u16 = string.trim().parse().unwrap();

    string
}

pub fn unsign8 () -> u8
{
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failed to read line");

    let string : u8 = string.trim().parse().unwrap();

    string

}

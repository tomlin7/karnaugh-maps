use std::io::{self, Write};

fn main() {
    //binary conversion, 1s complement, 2s complement
    print!("Enter decimal number: ");
    io::stdout().flush().unwrap();
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read input");
    let mut y = y.trim().parse::<i32>().unwrap();
    let mut signed = false;
    if y < 0 {
        y = -y;
        signed = true;
    }

    let mut binary = String::new();
    loop {
        binary = format!("{}{}", y % 2, binary);
        y /= 2;
        if y == 0 {
            break;
        }
    }
    println!("Binary: {}", binary);

    let mut ones_complement = String::new();
    if signed {
        for i in binary.chars() {
            if i == '1' {
                ones_complement = format!("{}{}", ones_complement, 0);
            } else {
                ones_complement = format!("{}{}", ones_complement, 1);
            }
        }
    } else {
        ones_complement = binary.clone();
    }
    println!("1s Complement: {}", ones_complement);

    let mut twos_complement = String::new();
    let mut carry = true;
    if signed {
        for i in ones_complement.chars().rev() {
            if carry {
                if i == '1' {
                    twos_complement = format!("{}{}", 0, twos_complement);
                } else {
                    twos_complement = format!("{}{}", 1, twos_complement);
                    carry = false;
                }
            } else {
                twos_complement = format!("{}{}", i, twos_complement);
            }
        }
    } else {
        twos_complement = binary.clone();
    }
    println!("2s Complement: {}", twos_complement);
}

use std::io::{self, Write};

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut input = input.trim().split('.');
    let mut integer = input.next().unwrap().parse::<i32>().unwrap();
    let sign = if integer > 0 { 0 } else { 1 };
    let mut decimal = format!(".{}", input.next().unwrap())
        .parse::<f32>()
        .unwrap();

    let mut binary_int = String::new();
    loop {
        binary_int = format!("{}{}", integer % 2, binary_int);
        integer /= 2;
        if integer == 0 {
            break;
        }
    }
    let mut exp = binary_int.len() as i32 - 1;
    let mut exp_32 = exp + 127; // excess 127 bias

    let mut binary_dec = String::new();
    for _ in 1..6 {
        if decimal == 0.0 {
            break;
        }

        decimal *= 2.0;
        if decimal >= 1.0 {
            binary_dec = format!("{}1", binary_dec);
            decimal -= 1.0;
        } else {
            binary_dec = format!("{}0", binary_dec);
        }
    }

    //  1 8 23

    // 8 bits for exponent
    let mut binary_exp_32 = String::new();
    for _ in 0..8 {
        binary_exp_32 = format!("{}{}", exp_32 % 2, binary_exp_32);
        exp_32 /= 2;
    }

    println!("Binary: {}.{}", binary_int, binary_dec);
    println!("Exponent: {}", binary_exp_32);

    // hidden format because H.xxxxx H is always 1
    // 23 bits for mantissa
    let mut mantissa = format!("{}{}", binary_int.split_off(1), binary_dec);
    let count = 23 - mantissa.len();
    for _ in 0..count {
        mantissa = format!("{}0", mantissa)
    }

    let binary_32 = format!("{}{}{}", sign, binary_exp_32, mantissa);
    println!("32bit precision: {} {} {}", sign, binary_exp_32, mantissa);

    let hex_32 = format!("{:X}", i32::from_str_radix(&binary_32, 2).unwrap());
    println!("Hex: {}H", hex_32);

    //  1 11 52

    let mut exp_64 = exp + 1023;

    // 11 bits for exponent
    let mut binary_exp_64 = String::new();
    for _ in 0..11 {
        binary_exp_64 = format!("{}{}", exp_64 % 2, binary_exp_64);
        exp_64 /= 2;
    }

    println!("Exponent: {}", binary_exp_64);

    // hidden format because H.xxxxx H is always 1
    // 23 bits for mantissa
    let mut mantissa = format!("{}{}", binary_int.split_off(1), binary_dec);
    let count = 52 - mantissa.len();
    for _ in 0..count {
        mantissa = format!("{}0", mantissa)
    }

    let binary_64 = format!("{}{}{}", sign, binary_exp_64, mantissa);
    println!("64bit precision: {} {} {}", sign, binary_exp_64, mantissa);
}

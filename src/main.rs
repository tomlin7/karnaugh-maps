use std::io::{self, Write};

fn main() {
    // 8 bits for multiplicand and multiplier

    print!("Enter multiplicand: ");
    io::stdout().flush().unwrap();
    let mut BR = String::new();
    io::stdin()
        .read_line(&mut BR)
        .expect("Failed to read input");
    let mut BR = BR.trim().parse::<i32>().expect("Failed to parse input");

    print!("Enter multiplier: ");
    io::stdout().flush().unwrap();
    let mut QR = String::new();
    io::stdin()
        .read_line(&mut QR)
        .expect("Failed to read input");
    let mut QR = QR.trim().parse::<i32>().expect("Failed to parse input");

    let mut AC = 0;
    let mut QE = 0;

    println!("AC: {:b} QR: {:b} QE: {}", AC, QR, QE);
    for _ in 0..8 {
        let lsb = QR & 1;

        if lsb == 1 && QE == 0 {
            // Qn Qn+1 = 10
            AC -= BR;
        } else if lsb == 0 && QE == 1 {
            // Qn Qn+1 = 01
            AC += BR;
        } // else 00 or 11

        println!("AC: {:b} QR: {:b} QE: {}", AC, QR, QE);

        QE = lsb;
        let lsb = AC & 1; // ac's lsb
        AC >>= 1;
        QR >>= 1;
        QR |= lsb << 7; // ac's lsb to qr's msb
    }

    println!("AC: {:b} QR: {:b} QE: {}", AC, QR, QE);

    let mut result = AC;
    result <<= 8;
    result |= QR;

    println!("Result: {:016b}", result);
}

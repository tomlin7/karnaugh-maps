use std::io;

fn main() {
    let mut km = vec![vec![0; 4]; 4];
    println!("Enter the K-map rows");

    for i in 0..km.len() {
        println!("Enter row {}:", i + 1);

        let mut row = String::new();
        io::stdin()
            .read_line(&mut row)
            .expect("Failed to read input");
        let values: Vec<i32> = row
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        for (j, &value) in values.iter().enumerate() {
            if j < km[i].len() {
                km[i][j] = value;
            }
        }
    }

    println!("Your K-map is:");
    for row in km.iter() {
        for &val in row.iter() {
            print!("{} ", val);
        }
        println!();
    }
}

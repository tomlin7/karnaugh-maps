use std::io;
use std::io::Write;

fn main() {
    let mut input = [(0, 0, 0, 0); 16];
    let mut output = Vec::new();

    println!(" A  B  C  D  |  Y ");
    println!("------------------");

    let mut i = 0;
    for a in 0..2 {
        for b in 0..2 {
            for c in 0..2 {
                for d in 0..2 {
                    input[i] = (a, b, c, d);
                    i += 1;

                    let mut y = String::new();
                    print!(" {}  {}  {}  {}  => ", a, b, c, d);
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut y).expect("Failed to read input");
                    y = y.trim().to_string();

                    if y == "1" {
                        output.push(1);
                    } else if y == "0" {
                        output.push(0);
                    } else {
                        output.push(-1); // dont care
                    }
                }
            }
        }
    }

    println!(concat!(
        "Truth Table\n",
        " A  B  C  D  |  Y\n",
        "-------------+----"
    ));
    for i in 0..16 {
        let inp = input[i];
        println!(
            " {}  {}  {}  {}  |  {}",
            inp.0,
            inp.1,
            inp.2,
            inp.3,
            if output[i] != -1 {
                output[i].to_string()
            } else {
                "x".to_string()
            }
        );
    }

    // to kmap
    let mut kmap = vec![vec![0; 4]; 4];
    // for i in 0..16 {
    //     if y[i] != -1 {
    //         kmap[a[i] as usize * 2 + b[i] as usize][c[i] as usize * 2 + d[i] as usize] = y[i];
    //     }
    // }

    let vals = vec![(0, 0), (0, 1), (1, 1), (1, 0)];
    for i in 0..4 {
        let ab = vals[i];
        for j in 0..4 {
            let cd = vals[j];

            for k in 0..16 {
                if input[k].0 == ab.0
                    && input[k].1 == ab.1
                    && input[k].2 == cd.0
                    && input[k].3 == cd.1
                {
                    kmap[i][j] = output[k];
                    break;
                }
            }
        }
    }

    // print kmap
    println!("Kmap");
    for i in 0..4 {
        for j in 0..4 {
            print!(
                " {} |",
                match kmap[i][j] {
                    -1 => "x",
                    0 => "0",
                    1 => "1",
                    _ => unreachable!(),
                }
            );
        }
        println!();
    }
}

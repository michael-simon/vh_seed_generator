pub mod random;
pub mod map;

use std::thread;
use std::time::Instant;

fn main() {
    //let map = map::Map::from_code("QBDBLDCHBB").unwrap();
    //map.print_map();
    //map.save_map("QBDBLDCHBB");

    let mut line = String::new();
    println!("Virtual Hydlide Map Generation Toolkit v1.0");
    println!("1 for timing generating a million seeds");
    println!("2 for generating and printing a specific seed");
    let _bytecount = std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    let choice = line.parse::<u8>().unwrap();
    if choice == 2 {
        println!("Enter a seed string (10 characters)");
        println!("♂ is Alt-11, ♀ is Alt-12");
        let mut line2 = String::new();
        let seedcount = std::io::stdin().read_line(&mut line2).unwrap();
        line2.pop();        
        if seedcount == 11 {
            let map = map::Map::from_code(line2.as_str()).unwrap();
            map.print_map();
            println!("Legend");
            println!("\x1b[93;100m{}\t{}\t{}\x1b[0m","G - Graveyard","M - Mansion", "T - Trial");
            println!("\x1b[93;100m{}\t{}\t{}\x1b[0m","R - Ruins","V - Volcano", "F - Fairy");
            println!("\x1b[93;100m{}\t{}\t{}\x1b[0m", "C - Castle Tablet", "@ - Start", "$ - Shop");
            println!("\x1b[31;100m{}\t\x1b[35;100m{}\x1b[0m", "┏ ┛┗ ┓ (isolated) - Sealed", "T - Transport Crystals");
        }
        else {
            println!("Please enter exactly 10 characters next time. Spaces count!");
        }
    }
    else {
        let now = Instant::now();

        let t1 = thread::spawn(|| {
            for i in 0..1000000/4 {
                let _ = map::Map::from_code(&random::VHRandom::from_seed(i).get_code());
            }
        });
    
        let t2 = thread::spawn(|| {
            for i in 1000000/4..1000000/2 {
                let _ = map::Map::from_code(&random::VHRandom::from_seed(i).get_code());
            }
        });
    
        let t3 = thread::spawn(|| {
            for i in 1000000/2..3000000/4 {
                let _ = map::Map::from_code(&random::VHRandom::from_seed(i).get_code());
            }
        });
    
        for i in 3000000/4..1000000 {
            let _ = map::Map::from_code(&random::VHRandom::from_seed(i).get_code());
        }
    
        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
    
        let elapsed = now.elapsed();
        println!("1,000,000 seeds generated in {} seconds", elapsed.as_secs_f64());
    }
}

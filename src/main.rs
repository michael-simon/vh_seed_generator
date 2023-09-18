pub mod random;
pub mod map;

use std::error::Error;
use std::thread;
use std::time::Instant;
use map::_FCargs;

#[macro_use(fcargs)]

fn map_iteration(count: u32, start: u32, winnow: bool, save: bool ) ->  Result<bool, Box<dyn Error>> {
    
    let now = Instant::now();    

    thread::scope(|s| {                    
        let t1 = s.spawn(|| {
            for i in start..(start + count/4) {
                let code = &random::VHRandom::from_seed(i).get_code();
                let m = match map::Map::from_code(&fcargs!(&code, winnow)) {
                    Ok(m) => Some(m),
                    Err(_) => None,
                };
                if save && m.is_some() {
                    m.unwrap().save_map(&code).unwrap(); 
                }
            }
        });
    
        let t2 = s.spawn(|| {
            for i in (start + count/4)..(start + count/2) {
                let code = &random::VHRandom::from_seed(i).get_code();
                let m = match map::Map::from_code(&fcargs!(&code, winnow)) {
                    Ok(m) => Some(m),
                    Err(_) => None,
                };
                if save && m.is_some() {
                    m.unwrap().save_map(&code).unwrap(); 
                }
            }
        });
    
        let t3 = s.spawn(|| {
            for i in (start + count/2)..(start + count/4*3) {
                let code = &random::VHRandom::from_seed(i).get_code();
                let m = match map::Map::from_code(&fcargs!(&code, winnow)) {
                    Ok(m) => Some(m),
                    Err(_) => None,
                };
                if save && m.is_some() {
                    m.unwrap().save_map(&code).unwrap(); 
                }
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
    });
        for i in (start + count/4*3)..(start + count) {
                let code = &random::VHRandom::from_seed(i).get_code();
                let m = match map::Map::from_code(&fcargs!(&code, winnow)) {
                    Ok(m) => Some(m),
                    Err(_) => None,
                };
                if save && m.is_some() {
                    m.unwrap().save_map(&code).unwrap(); 
                }
        }
    
        let elapsed = now.elapsed();
        let mut savestring = "";
        if save {
            savestring = ", saved ";
        }
        let mut winnowedstring = "";
        if winnow {
            winnowedstring = ", winnowed "
        }
        println!("{} maps generated {}{}in {} seconds", count, winnowedstring, savestring, elapsed.as_secs_f64());
        return Ok(true);
}

fn main() {    
    let mut line = String::new();
    println!("Virtual Hydlide Map Generation Toolkit v1.1");
    println!("1 for timing generating a million seeds");
    println!("2 for generating and printing a specific seed");    
    println!("3 to generate ascii for all 5 base maps");
    println!("4 to generate a given number of seeds, possibly winnowing the results out and saving only the remainder.");
    let _bytecount = std::io::stdin().read_line(&mut line).unwrap();        
    let choice = line.trim_end().parse::<u8>().unwrap();
    if choice == 2 {
        println!("Enter a seed string (10 characters)");
        println!("♂ is Alt-11, ♀ is Alt-12");
        let mut line2 = String::new();
        let _seedcount = std::io::stdin().read_line(&mut line2).unwrap();
        while line2.len() > 10 {
            line2.pop();
        }        
        if line2.len() == 10 {
            let map = map::Map::from_code(&fcargs!(line2.as_str())).unwrap();
            map.print_map();
            println!("Legend");
            println!("\x1b[93;100m{}\t{}\t{}\x1b[0m","G - Graveyard","M - Mansion", "T - Trial");
            println!("\x1b[93;100m{}\t{}\t{}\x1b[0m","R - Ruins","V - Volcano", "F - Fairy");
            println!("\x1b[93;100m{}\t{}\t{}\x1b[0m", "C - Castle Tablet", "@ - Start", "$ - Shop");
            println!("\x1b[93;100m{}\t\x1b[35;100m{}\x1b[0m", "S - Sealed", "T - Transport Crystals");
        }
        else {
            println!("Please enter exactly 10 characters next time. Spaces count!");
        }
    }
    else if choice == 3 {
        for i in 1..6 {
          let m = match map::load_base_map(i) {
              Ok(m) => m,
              Err(error) => panic!("Couldn't open base map {}: {:?}",i, error),
          };
          m.print_map();
          println!("--------------------------------------------------");
        }
    }
    else if choice == 1 {
        let _ = map_iteration(1000000, 0, false, false);
    }
    else if choice == 4 {
        let mut save = false;
        let mut winnow = false;
        println!("Enter what number you want to start on");
        let mut line_start = String::new();
        let mut _count = std::io::stdin().read_line(&mut line_start).unwrap();        
        let start = line_start.trim_end().parse::<u32>().unwrap();
        println!("Enter the number of iterations you want");
        let mut line2 = String::new();
        _count = std::io::stdin().read_line(&mut line2).unwrap();        
        let iterations = line2.trim_end().parse::<u32>().unwrap();
        println!("Enter Y to winnow (winnowing criteria: perfect V->S->C walk)");
        let mut line3 = String::new();
        _count = std::io::stdin().read_line(&mut line3).unwrap();        
        if line3.trim_end() == "Y" {
            winnow = true;
        }
        println!("Enter Y to save (9K per file, do the math)");
        let mut line4 = String::new();
        _count = std::io::stdin().read_line(&mut line4).unwrap();        
        if line4.trim_end() == "Y" {
            save = true;
        }
        map_iteration(iterations, start, winnow, save).unwrap();
    }
    else {
        println!("You didn't pick one of the options, so we're done! Congratulations.")
    }
}

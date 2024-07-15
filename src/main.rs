pub mod random;
pub mod map;

use std::error::Error;
use std::thread;
use std::time::Instant;
use map::_FCargs;
use std::collections::hash_map::HashMap;
use std::fs;
use std::io;
use std::ffi::OsString;
use std::ffi::OsStr;

#[macro_use(fcargs)]

fn map_iteration(count: u32, start: u32, difficulty: map::Difficulty, winnow: &Vec<bool>, save: bool) ->  Result<bool, Box<dyn Error>> {
    
    let now = Instant::now();    

    thread::scope(|s| {                    
        let t1 = s.spawn(|| {
            for i in start..(start + count/4) {
                let code = &random::VHRandom::from_seed(i).get_code();
                let m = match map::OverworldMap::from_code(&fcargs!(&code, difficulty, winnow.to_vec())) {
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
                let m = match map::OverworldMap::from_code(&fcargs!(&code, difficulty, winnow.to_vec())) {
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
                let m = match map::OverworldMap::from_code(&fcargs!(&code, difficulty, winnow.to_vec())) {
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
                let m = match map::OverworldMap::from_code(&fcargs!(&code, difficulty, winnow.to_vec())) {
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
        let mut winnowedstring = String::from("");
        for i in 0..winnow.len() {
            if winnow[i] {
              winnowedstring = winnowedstring + &i.to_string() + "w ";
            }
        }
        println!("{} maps generated {}{}in {} seconds", count, winnowedstring, savestring, elapsed.as_secs_f64());
        return Ok(true);
}

fn main() {
    let mut difficulty:map::Difficulty = map::Difficulty::Easy;    
    loop {
        let mut line = String::new();
        println!("Virtual Hydlide Map Generation Toolkit v2.4.0");
        println!("1 to set the difficulty, currently {}", map::difficulty_text(&difficulty));
        println!("2 for generating and printing a specific seed");    
        println!("3 to generate ascii for all 5 base maps");
        println!("4 to generate a given number of seeds, possibly winnowing the results out and saving only the remainder.");
        println!("5 to return the maps in /genmaps in shortest path order");
        println!("Anything else to quit or crash.");
        let _bytecount = std::io::stdin().read_line(&mut line).unwrap();
        println!("{}",line);
        let choice = line.strip_suffix("\n").unwrap().parse::<u8>().unwrap();
        if choice == 2 {
            println!("Enter a seed string (10 characters)");
            println!("♂ is Alt-11, ♀ is Alt-12");
            let mut line2 = String::new();
            let _seedcount = std::io::stdin().read_line(&mut line2).unwrap();            
            let mut str_line = line2.as_str().strip_suffix("\n").unwrap();            
            let result_map= map::OverworldMap::from_code(&fcargs!(str_line, difficulty));
            match result_map {
                Ok(map) => { map.print_map();
                println!("Legend");
                println!("\x1b[93;100m{}\t{}\t{}\x1b[0m","G - Graveyard","M - Mansion", "T - Trial");
                println!("\x1b[93;100m{}\t{}\t{}\x1b[0m","R - Ruins","V - Volcano", "F - Fairy");
                println!("\x1b[93;100m{}\t{}\t{}\x1b[0m", "C - Castle Tablet", "@ - Start", "$ - Shop");
                println!("\x1b[93;100m{}\t\x1b[35;100m{}\x1b[0m", "S - Sealed", "T - Transport Crystals");
                println!("\x1b[32m{}\t{}\t{}\t{}\x1b[0m", "h - herbs", "a - antidotes", "p - poison herbs","e - elevator");
                },
                Err(e) => { println!("{}", e); println!("Please enter exactly 10 characters next time. Spaces count!"); }
            }
        }
        else if choice == 3 {
            for i in 1..6 {
              let mut m = match map::load_base_map(i) {
                  Ok(m) => m,
                  Err(error) => panic!("Couldn't open base map {}: {:?}",i, error),
              };
              for _i in 0..4 {
                m.rotate(1);
                m.print_map();
                println!("--------------------------------------------------");
              }
              println!("--------------------------------------------------");
            }
        }
        else if choice == 1 {
            let mut difficulty_line = String::new();
            println!("Enter the difficulty: Easy, Medium, Hard, or PRO");
            let mut _count = std::io::stdin().read_line(&mut difficulty_line).unwrap();
            let difficulty_string = difficulty_line.trim_end();
            difficulty = map::text_difficulty(difficulty_string);
        }
        else if choice == 4 {
            let mut save = false;
            let mut winnow = Vec::from([false, false, false]);
            println!("Enter what number you want to start on");
            let mut line_start = String::new();
            let mut _count = std::io::stdin().read_line(&mut line_start).unwrap();        
            let start = line_start.trim_end().parse::<u32>().unwrap();
            println!("Enter the number of iterations you want");
            let mut line2 = String::new();
            _count = std::io::stdin().read_line(&mut line2).unwrap();        
            let iterations = line2.trim_end().parse::<u32>().unwrap();
            println!("Do you want to winnow based on...");
            println!("... not being Map 4? (Y/N)");
            let mut line3 = String::new();
            _count = std::io::stdin().read_line(&mut line3).unwrap();                    
            if line3.trim_end() == "Y" {
                winnow[0] = true;                
            }
            println!("... not having a shortest possible V-S-C? (Y/N)");
            line3 = String::new();
            _count = std::io::stdin().read_line(&mut line3).unwrap();                    
            if line3.trim_end() == "Y" {                
                winnow[1] = true;               
            }
            println!("... not having a short overall path? (Y/N)");
            line3 = String::new();
            _count = std::io::stdin().read_line(&mut line3).unwrap();                    
            if line3.trim_end() == "Y" {                
                winnow[2] = true;               
            }
            println!("Enter Y to save (9K per file, do the math)");
            let mut line4 = String::new();
            _count = std::io::stdin().read_line(&mut line4).unwrap();        
            if line4.trim_end() == "Y" {
                save = true;
            }
            map_iteration(iterations, start, difficulty, &winnow, save).unwrap();
        }
        else if choice == 5 {
            // shortest_path dictionary empty
            let mut path_lengths = HashMap::new();
            for mapfile in fs::read_dir("./genmaps").expect("read dir call failed") {
                if let Ok(mapfile) = mapfile {
                    let mappath: std::path::PathBuf = mapfile.path();
                    if mappath.is_dir() {
                        continue;
                    }
                    //println!("{:?}", mappath.display());
                    let e = mappath.extension().unwrap();                                  
                    if e == "BIN" {
                        let stem = mappath.file_stem().unwrap();
                        let n: OsString = stem.to_os_string();
                        //println!("{:?}", n);
                        let newmap = map::load_map(&n).unwrap();
                        let (first_half, last_half) = newmap.calculate_shortest_distance();                        
                        let shortest_distance = first_half + last_half;
                        path_lengths.entry(shortest_distance)
                          .and_modify(|e: &mut Vec<OsString>| { e.push(n.clone()) })
                          .or_insert(vec![n.clone()]);
                    }                
                }
            }
            let mut lengths: Vec<_> = path_lengths.keys().collect();
            lengths.sort();
            lengths.reverse();
            for l in lengths {
                println!("{}", l);
                println!("{:?}", path_lengths[l]);
            }        
        }
        else {
            println!("You didn't pick one of the options, so we're done! Congratulations.");
            break;
        }
    }
}

pub mod random;
pub mod map;

use std::thread;
use std::time::Instant;

fn main() {
    //let map = map::Map::from_code("QBDBLDCHBB").unwrap();
    //map.print_map();
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

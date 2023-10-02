use std::error::Error;
use crate::random::VHRandom;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tile {
    pub id : u8,
    pub rotation : i8,
    pub height : i8
}

impl Tile {
    fn get_ascii_art(&self) -> &'static str {
        match self.id {
            0x0 => " ",
            0x1..=0x4 | 0x7 | 0x19 | 0x27 | 0x2a | 0x30 => "\x1b[90m█\x1b[0m",
            0x5 | 0x6 | 0x8 => "█",
            // An experiment to see about drawing the edge tiles more cleanly than the base
            // game does
            /*0x6 => {
                match self.rotation {
                    0 => "\x1b[33;100m▄\x1b[0m",
                    1 => "\x1b[33;100m▌\x1b[0m",
                    2 => "\x1b[33;100m▀\x1b[0m",
                    _ => "\x1b[33;100m▐\x1b[0m",
                }
            }
            0x7 => {
                match self.rotation {
                    0 => "\x1b[33;100m▗\x1b[0m",
                    1 => "\x1b[33;100m▖\x1b[0m",
                    2 => "\x1b[33;100m▘\x1b[0m",
                    _ => "\x1b[33;100m▝\x1b[0m",
                }
                
            }
            0x8 => {
                match self.rotation {
                    0 => "\x1b[33;100m▟\x1b[0m",
                    1 => "\x1b[33;100m▙\x1b[0m",
                    2 => "\x1b[33;100m▛\x1b[0m",
                    _ => "\x1b[33;100m▜\x1b[0m",
                }
            }*/
            0x9 | 0xa | 0xb | 0xc | 0x38 => "\x1b[32m█\x1b[0m",
            0xd..=0x11 | 0x13..=0x15 | 0x34 | 0x3a => "\x1b[34m█\x1b[0m",
            0x16 | 0x31 => {
                match self.rotation {
                    0 | 2 => "\x1b[30;44m┃\x1b[0m",
                    _ => "\x1b[30;44m━\x1b[0m"
                }
            }
            0x17 | 0x18 | 0x1a => "\x1b[31m█\x1b[0m",
            0x1b..=0x20 => "\x1b[100m▲\x1b[0m",
            0x21 | 0x24 => {
                match self.rotation {
                    0 | 2 => "\x1b[31;100m━\x1b[0m",
                    _ => "\x1b[31;100m┃\x1b[0m"
                }
            }
            0x22 => {
                match self.rotation {
                    0 => "\x1b[31;100m┏\x1b[0m",
                    1 => "\x1b[31;100m┓\x1b[0m",
                    2 => "\x1b[31;100m┛\x1b[0m",
                    _ => "\x1b[31;100m┗\x1b[0m",
                }
            }
            0x23 => {
                match self.rotation {
                    0 => "\x1b[31;100m┛\x1b[0m",
                    1 => "\x1b[31;100m┗\x1b[0m",
                    2 => "\x1b[31;100m┏\x1b[0m",
                    _ => "\x1b[31;100m┓\x1b[0m",
                }
            }
            0x25..=0x29 | 0x2b | 0x2f => "\x1b[32;100m♣\x1b[0m",
            0x2c => "\x1b[93;100mG\x1b[0m",
            0x2d => "\x1b[93;100mS\x1b[0m",
            0x2e => "\x1b[93;100mM\x1b[0m",
            0x32 => "\x1b[100mR\x1b[0m",
            0x39 => "\x1b[93;100mR\x1b[0m",
            0x33 => "\x1b[93;100mT\x1b[0m",
            0x35 => "\x1b[93;100mV\x1b[0m",
            0x36 => "\x1b[93;100mF\x1b[0m",
            0x37 => "\x1b[35;100mT\x1b[0m",
            0x3b => "\x1b[93;100m$\x1b[0m",
            0xff => "\x1b[93;100m@\x1b[0m",            
            0x12 => "\x1b[93;44mC\x1b[0m",
            _ => "?"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Difficulty {
        Easy,
        Medium,
        Hard,
        PRO,
}

pub fn difficulty_text(d:&Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => return "Easy",
        Difficulty::Medium => return "Medium",
        Difficulty::Hard => return "Hard",
        Difficulty::PRO => return "PRO"
    }    
}

pub fn text_difficulty(s:&str) -> Difficulty {
    match s {
        "Easy" => return Difficulty::Easy,
        "Medium" => return Difficulty::Medium,
        "Hard" => return Difficulty::Hard,
        "PRO" => return Difficulty::PRO,
        &_ => return Difficulty::Easy,
    }
}

#[macro_use]
mod args {       
    pub use crate::map;
    pub struct _FCargs {
        pub code: String,
        pub difficulty: map::Difficulty,
        pub winnow: bool,
    }
    
    #[macro_export]
    macro_rules! fcargs {
        ($mand_1:expr, $mand_2:expr) => {
            _FCargs {code: $mand_1.to_string(), difficulty: $mand_2, winnow: false}
        };
        ($mand_1:expr, $mand_2:expr, $opt:expr) => {
            _FCargs {code: $mand_1.to_string(), difficulty: $mand_2, winnow: $opt}
        };
    }
}

pub use args::_FCargs;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map {
    width : usize,
    height : usize,
    tiles : Vec<Tile>,
    dungeons : Vec<(Tile, (usize, usize))> // dungeons array is in order of MapIds below. First is Ruins, etc.
}

#[repr(u8)]
enum MapIds {
    Ruins = 0x39,
    Mansion = 0x2e,
    Fairy = 0x36,
    Trial = 0x33,
    Graveyard = 0x2c,
    Volcano = 0x35,
    Sealed = 0x2d,
    Shop = 0x3b
}

impl Map {
    /// Generate the overworld map from a given code.
    /// The winnow paramaeter allows you to stop generation based on the Map::winnow function for the instance
    pub fn from_code(fc: &args::_FCargs) -> Result<Map, Box<dyn Error>>{
        let code = fc.code.as_str();
        let winnow = fc.winnow;
        let difficulty = fc.difficulty;
        let Some(mut rng) = VHRandom::from_code(code)
            else {return Err("Could not create RNG from Code!".into())};
        
        let map_id = rng.rand(5) + 1;
        if winnow && (map_id != 4) {
            return Err("Map was not base map 4, cannot be good.".into());
        }
        let mut base_map = load_base_map(map_id)?;

        let base_rotation = rng.rand(4) as u8;
        base_map.rotate(base_rotation as i8);

        let mut timeout = 0;
        let mut rng_seed = rng.get_seed();

        loop {
            // Weird little bit of code to get out of bad rng? but the rng used has a cycle length of the full
            // 32 bits so this seems really unnecessary. I wonder if it ever comes up ever.
            timeout += 1;
            if timeout > 2 {
                //println!("Found a seed that triggered the rng oddness: {}", rng.get_code());
                timeout = 0;
                rng_seed += 1;
                rng.set_seed(rng_seed);
            }

            let mut map = base_map.clone();
            //Place forest
            map.fill_tiles(&[0x25, 0x29, 0x2a, 0x2b], 600, &mut rng);
            //Make inner forest edges
            map.fix_edges(&[0x25, 0x26, 0x27, 0x28]);

            //Place meadows (green) tiles
            map.fill_tiles(&[9], 400, &mut rng);
            //Place rocky (dark gray) tiles
            map.fill_tiles(&[5, 6, 7, 8], 200, &mut rng);
            //Place sparse trees (brown) tiles
            map.fill_tiles(&[0x17, 0x18, 0x19, 0x1a], 300, &mut rng);

            for t in map.tiles.iter_mut() {
                if t.id == 0 {t.id = 1}
            }

            let mut feature_locations: Vec<(usize, usize)> = Vec::new();

            // Place ruins
            if !map.place_feature(&[(MapIds::Ruins as u8, 2), (0x32, 3), (0x32, 1), (0x32, 0)], 2, 2, 1, 5, &mut feature_locations, &mut rng) {
                //println!("Failed to place ruins in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Ruins as u8, rotation: 2, height: h }, (x, y)));
            // Place mansion
            let rand_rotation = (rng.rand_byte() & 3) as i8;
            if !map.place_feature(&[(MapIds::Mansion as u8, rand_rotation)], 1, 1, 1, 0x25, &mut feature_locations, &mut rng) {
                //println!("Failed to place mainsion in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Mansion as u8, rotation: rand_rotation, height: h }, (x, y)));
            // Place meadow variants
            if !map.place_feature(&[(0xa, 0)], 1, 1, 2, 9, &mut feature_locations, &mut rng) {
                //println!("Failed to place meadows1 in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            if !map.place_feature(&[(0xb, 0)], 1, 1, 2, 9, &mut feature_locations, &mut rng) {
                //println!("Failed to place meadows2 in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            if !map.place_feature(&[(0xc, 0)], 1, 1, 2, 9, &mut feature_locations, &mut rng) {
                //println!("Failed to place meadows3 in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            if !map.place_feature(&[(0x38, 0)], 1, 1, 2, 9, &mut feature_locations, &mut rng) {
                //println!("Failed to place meadows4 in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            // Fairy Forest
            if !map.place_feature(&[(MapIds::Fairy as u8, 0)], 1, 1, 1, 1, &mut feature_locations, &mut rng) {
                //println!("Failed to place fairy forest in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Fairy as u8, rotation: 0, height: h }, (x, y)));
            // Place Trial Dungeon
            if !map.place_feature(&[(MapIds::Trial as u8, 0)], 1, 1, 1, 9, &mut feature_locations, &mut rng) {
                //println!("Failed to place trial dungeon in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Trial as u8, rotation: 0, height: h }, (x, y)));
            // Place Graveyard
            let rand_rotation = (rng.rand_byte() & 3) as i8;
            if !map.place_feature(&[(MapIds::Graveyard as u8, rand_rotation)], 1, 1, 1, 0x25, &mut feature_locations, &mut rng) {
                //println!("Failed to place graveyard in seed {}", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Graveyard as u8, rotation: rand_rotation, height: h }, (x, y)));
            // Place Volcano
            let rand_rotation = (rng.rand_byte() & 3) as i8;
            if !map.place_feature(&[(MapIds::Volcano as u8, rand_rotation)], 1, 1, 1, 5, &mut feature_locations, &mut rng) {
                //println!("Failed to place volcano in seed {}", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Volcano as u8, rotation: rand_rotation, height: h }, (x, y)));
            // Place Sealed Dungeon
            if !map.place_feature(&[(MapIds::Sealed as u8, -1)], 1, 1, 1, 0x1b, &mut feature_locations, &mut rng) {
                //println!("Failed to place sealed dungeon in seed {}", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            let r = map.tiles[x + y * map.width].rotation;
            map.dungeons.push((Tile { id: MapIds::Sealed as u8, rotation: r, height: h }, (x, y)));

            // Winnowing calculation can be completed
            if winnow {
                if map.volcano_to_sealed_to_castle(base_rotation) > 9 {
                  return Err("Ending not close enough to perfect".into());
                }
            }
            
            // Place Shop            
            let rand_rotation = (rng.rand_byte() & 3) as i8;

            let mut shop_feature = MapIds::Shop as u8;
            if difficulty == Difficulty::Hard || difficulty == Difficulty::PRO {
                shop_feature = 1 as u8;
            }
            
            if !map.place_feature(&[(shop_feature, rand_rotation)], 1, 1, 1, 1, &mut feature_locations, &mut rng) {
                //println!("Failed to place shop in seed {}!", rng.get_code());
                //map.print_map();
                continue
            };
            let (x,y) = feature_locations[feature_locations.len() - 1];
            let h = map.tiles[x + y * map.width].height;
            map.dungeons.push((Tile { id: MapIds::Shop as u8, rotation: rand_rotation, height: h }, (x, y)));
            
            let num_default_tiles = map.tiles.iter().filter(|&t| t.id == 1).count();
            let start_pos_idx1 = rng.rand(num_default_tiles as u32);

            let start_pos_idx2 = map.tiles.iter().enumerate().filter(|&(_, t)| t.id == 1).nth(start_pos_idx1 as usize).unwrap().0;

            // Using 0xff to mark the starting tile, as opposed to adding more to the map struct. No information is lost because the
            // player always starts on an id 1 tile
            map.tiles[start_pos_idx2].id = 0xff;

            // I think this is for selecting the fairy forest tree with the fairy, but I'm not sure.
            // Ultimately this is slightly meaningless to include but it is the next thing called by rng.
            // After this is some 0-99 rolls that I haven't deciphered, but might be placing items on the
            // ground?
            let _fairy_forest_tree_maybe = rng.rand(0x10);

            //map.save_map(&code)?;

            return Ok(map)
        }
    }

    fn rotate(&mut self, rotation : i8) {
        match rotation {
            1 => {
                let mut new_tiles: Vec<Tile> = Vec::with_capacity(50*50);
                for x in (0..self.width).rev() {
                    for y in 0..self.height {
                        let mut tile = self.tiles[x + y * self.width].clone();
                        // yes, 4-rotation. For whatever reason the base-map rotations and
                        // the tile rotations go in opposite directions
                        tile.rotation = (tile.rotation + (4 - rotation)) % 4;
                        new_tiles.push(tile)
                    }
                }
                self.tiles = new_tiles;
            },
            2 => {
                let mut new_tiles: Vec<Tile> = Vec::with_capacity(50*50);
                for y in (0..self.height).rev() {
                    for x in (0..self.width).rev() {
                        let mut tile = self.tiles[x + y * self.width].clone();
                        tile.rotation = (tile.rotation + (4 - rotation)) % 4;
                        new_tiles.push(tile)
                    }
                }
                self.tiles = new_tiles;
            }
            3 => {
                let mut new_tiles: Vec<Tile> = Vec::with_capacity(50*50);
                for x in 0..self.width {
                    for y in (0..self.height).rev() {
                        let mut tile = self.tiles[x + y * self.width].clone();
                        tile.rotation = (tile.rotation + (4 - rotation)) % 4;
                        new_tiles.push(tile)
                    }
                }
                self.tiles = new_tiles;
            }
            _ => return
        }
       
    }

    fn fill_tiles(&mut self, tile_ids: &[u8], mut count: u32, rng: &mut VHRandom) {
        loop {
            if count < 1 {break}

            let (mut x, mut y) = (0, 0);

            // Find a candidate empty tile to start placing from
            for timeout in 0..2000 {
                x = rng.rand(self.width as u32) as usize;
                y = rng.rand(self.height as u32) as usize;

                if self.tiles[x + y * self.width].id == 0 {break;}
                if timeout == 1999 {return}
            }

            self.tiles[x + y * self.width].id = tile_ids[0]; 
            count -= 1;

            let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);

            while count > 0 {
                neighbors.truncate(0);
                // Go through the current tile's neighbors, setting all empty tiles to the fill tile
                for (x_dir, y_dir) in [(0, -1), (-1, 0), (1, 0), (0, 1)].iter() {
                    let mut new_x = x as i64 + x_dir;
                    // wrapping around the borders
                    if new_x < 0 {new_x = new_x + self.width as i64}
                    else if new_x >= self.width as i64 {new_x = new_x - self.width as i64}

                    let mut new_y = y as i64 + y_dir;
                    if new_y < 0 {new_y = new_y + self.height as i64}
                    else if new_y >= self.height as i64 {new_y = new_y - self.height as i64}

                    if self.tiles[new_x as usize + new_y as usize * self.width].id == 0 {
                        count -= 1;
                        self.tiles[new_x as usize + new_y as usize * self.width].id = tile_ids[0];
                        neighbors.push((new_x as usize, new_y as usize));
                    }
                    // Note that we're only breaking out of the loop checking each neighboring direction.
                    // We'll still end up picking one of the neighboring tiles with an rng call even though
                    // we have no more placing to do!
                    if count < 1 {break}
                }
                //If we didn't have any neighbors, break and go back to picking a random empty tile to start again
                if neighbors.len() == 0 {break}

                //Pick a random neighboring tile we just filled to be the new tile to fill from.
                let next_tile = rng.rand(neighbors.len() as u32);
                (x, y) = neighbors[next_tile as usize]
            }
        }
        if tile_ids.len() == 4 {
            self.fix_edges(tile_ids);
        }
    }

    // Uses the values in tile_ids to create the edges of the terrain regions. tile_ids[0] is the base tile,
    // then tile_ids[1] is for edges, tile_ids[2] is for outside corners, and tile_ids[3] is for inside corners
    fn fix_edges(&mut self, tile_ids: &[u8]) {
        let mut new_tiles = self.tiles.clone();

        for y in 0..self.height {
            for x in 0..self.width{
                if self.tiles[x + y * self.width].id == tile_ids[0] {
                    let north_y = if y == 0 {self.height - 1} else {y - 1};
                    let north = self.tiles[x + north_y * self.width].id == tile_ids[0];

                    let west_x = if x == 0 {self.width - 1} else {x - 1};
                    let west = self.tiles[west_x + y * self.width].id == tile_ids[0];

                    let east_x = if x == self.width - 1 {0} else {x + 1};
                    let east = self.tiles[east_x + y * self.width].id == tile_ids[0];

                    let south_y = if y == self.height - 1 {0} else {y + 1};
                    let south = self.tiles[x + south_y * self.width].id == tile_ids[0];

                    // Would a lookup table be less code? yes probably but shh...
                    match (north, west, east, south) {
                        //isolated
                        (false, false, false, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[0];
                            new_tiles[x + y * self.width].rotation = 0;
                        }
                        //southern protrusion
                        (true, false, false, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 2;
                        }
                        //eastern protrusion
                        (false, true, false, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 1;
                        }
                        //south-eastern outer corner
                        (true, true, false, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[2];
                            new_tiles[x + y * self.width].rotation = 2;
                        }
                        //western protrusion
                        (false, false, true, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 3;
                        }
                        //south-western outer corner
                        (true, false, true, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[2];
                            new_tiles[x + y * self.width].rotation = 3;
                        }
                        //east-west bridge
                        (false, true, true, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[0];
                            new_tiles[x + y * self.width].rotation = 0;
                        }
                        //southern edge
                        (true, true, true, false) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 2;
                        }
                        //northern protrusion
                        (false, false, false, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 0;
                        }
                        //north-south bridge
                        (true, false, false, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[0];
                            new_tiles[x + y * self.width].rotation = 0;
                        }
                        //north-eastern outer corner
                        (false, true, false, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[2];
                            new_tiles[x + y * self.width].rotation = 1;
                        }
                        //eastern edge
                        (true, true, false, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 1;
                        }
                        //north-western outer corner
                        (false, false, true, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[2];
                            new_tiles[x + y * self.width].rotation = 0;
                        }
                        //western edge
                        (true, false, true, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 3;
                        }
                        //southern edge
                        (false, true, true, true) => {
                            new_tiles[x + y * self.width].id = tile_ids[1];
                            new_tiles[x + y * self.width].rotation = 0;
                        }
                        // completely surrounded, check diagonals
                        (true, true, true, true) => {
                            let north_west = self.tiles[west_x + north_y * self.width].id == tile_ids[0];
                            let north_east = self.tiles[east_x + north_y * self.width].id == tile_ids[0];
                            let south_east = self.tiles[east_x + south_y * self.width].id == tile_ids[0];
                            let south_west = self.tiles[west_x + south_y * self.width].id == tile_ids[0];

                            match (north_west, north_east, south_east, south_west) {
                                // Open only to the north-west
                                (false, true, true, true) => {
                                    new_tiles[x + y * self.width].id = tile_ids[3];
                                    new_tiles[x + y * self.width].rotation = 0;
                                }
                                // Open only to the north-east
                                (true, false, true, true) => {
                                    new_tiles[x + y * self.width].id = tile_ids[3];
                                    new_tiles[x + y * self.width].rotation = 1;
                                }
                                // Open only to the south-east
                                (true, true, false, true) => {
                                    new_tiles[x + y * self.width].id = tile_ids[3];
                                    new_tiles[x + y * self.width].rotation = 2;
                                }
                                // Open only to the south-west
                                (true, true, true, false) => {
                                    new_tiles[x + y * self.width].id = tile_ids[3];
                                    new_tiles[x + y * self.width].rotation = 3;
                                }
                                _ => {
                                    new_tiles[x + y * self.width].id = tile_ids[0];
                                    new_tiles[x + y * self.width].rotation = 0;
                                }
                            }
                        }
                    } 
                }
            }
        }

        self.tiles = new_tiles;
    }

    // Place a feature like a dungeon. Features are disallowed from being within +- 5 tiles of any other spawned feature. (well technically,
    // only the first feature placed if the count is >1 counts for this, for some reason)
    fn place_feature(
        &mut self, tiles: &[(u8, i8)], 
        feature_width: usize, 
        feature_height: usize, 
        count: usize, 
        spawn_tile: u8, 
        feature_tiles: &mut Vec<(usize, usize)>, 
        rng: &mut VHRandom) -> bool 
    {
        // Find all tiles with the id of `spawn_tile` which are not within +-5 tiles of another feature
        let mut possible_locations: Vec<(usize, usize)> = Vec::with_capacity(400);

        for y in 0..self.height {
            'tile_loop: for x in 0..self.width {
                if self.tiles[x + y*self.width].id == spawn_tile {
                    // make sure none of the features in our pre-existing feature list are too close to the tile
                    for &(feature_x, feature_y) in feature_tiles.iter() {
                        let in_x_bound = 
                            // need to handle the wrap around near the borders
                            if feature_x < 5 {(x <= feature_x + 5) || self.width - (5 - feature_x) <= x}
                            else if feature_x > (self.width - 1) - 5 {(feature_x - 5 <= x && x <= self.width - 1) || x <= feature_x + 5 - self.width}
                            else {feature_x - 5 <= x && x <= feature_x + 5};
                        let in_y_bound = 
                            if feature_y < 5 {(y <= feature_y + 5) || self.height - (5 - feature_y) <= y}
                            else if feature_y > (self.width - 1) - 5 {(feature_y - 5 <= y && y <= self.height - 1) || y <= feature_y + 5 - self.height}
                            else {feature_y - 5 <= y && y <= feature_y + 5};
                        // if the feature is too close, go to the next iteration of the loop through tiles
                        // there's probably a cleaner way to do this without the outer loop continue, this
                        // is just easy and matches how the game does it.
                        if in_x_bound && in_y_bound {continue 'tile_loop}
                    }

                    // If we're here, the loop above didn't break out, so we have a valid tile location... almost
                    // we still need to check that all the tiles the feature will be placed on (if it's bigger than 1x1)
                    // are all `spawn_tile`
                    for y2 in y..y+feature_height {
                        for x2 in x..x+feature_width {
                            let wrapped_x2 = if x2 >= self.width {x2 - self.width} else {x2};
                            let wrapped_y2 = if y2 >= self.height {y2 - self.height} else {y2};
                            // Again just going to continue to the outer loop
                            if self.tiles[wrapped_x2 + wrapped_y2 * self.width].id != spawn_tile {continue 'tile_loop}
                        }
                    }

                    // Both checks above succeeded, push the value as a candidate
                    possible_locations.push((x, y));
                }
            }
        }

        // If there's not enough candidates to place all the features requested we return false
        if possible_locations.len() < count {return false}

        // Do a bad shuffle of the candidate locations
        for _ in 0..possible_locations.len()>>1 {
            let a = rng.rand(possible_locations.len() as u32) as usize;
            let b = rng.rand(possible_locations.len() as u32) as usize;
            possible_locations.swap(a, b)
        }

        // For some reason only the first feature placed is added to the list.
        feature_tiles.push(possible_locations[0]);

        // Actually place the features
        for i in 0..count {
            let (x, y) = possible_locations[i];

            for y2 in 0..feature_height {
                for x2 in 0..feature_width {
                    let wrapped_x2 = if x+x2 >= self.width {x+x2 - self.width} else {x+x2};
                    let wrapped_y2 = if y+y2 >= self.height {y+y2 - self.height} else {y+y2};

                    let (tile, rotation) = tiles[x2 + y2 * feature_width];
                    self.tiles[wrapped_x2 + wrapped_y2*self.width].id = tile;
                    if 0 <= rotation && rotation <= 3 {self.tiles[wrapped_x2 + wrapped_y2*self.width].rotation = rotation}
                }
            }
        }

        true
    }

    pub fn print_map(&self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            print!("{}", tile.get_ascii_art());
            if i % self.width == self.width - 1 {
                println!();
            }
        }
    }    
    
    pub fn save_map(&self, s: &str) -> Result<bool, Box<dyn Error>> {        
        // Set the map header
        let mut map_file = vec![0 as u8; 0x23c4];
        let map_header = expected_map_header();
        for i in 0x0..0x18 {
            map_file[0x660 + i] = map_header[i];
        }
        // Write out all the tiles
        for i in 0..self.width*self.height {
            map_file[i*3 + 0x678] = self.tiles[i].id;
            map_file[i*3 + 0x679] = self.tiles[i].rotation as u8;
            map_file[i*3 + 0x67a] = self.tiles[i].height as u8;
        }

        std::fs::write(format!("./genmaps/{}.BIN", s), &map_file[..])?;
        Ok(true)
    }    

    pub fn volcano_to_sealed_to_castle(&self, rotation: u8) -> u8 {
        let len = self.dungeons.len();
        let (_, (vx, vy)) = self.dungeons[len-2]; // Volcano
        let (_, (sx, sy)) = self.dungeons[len-1]; // Sealed
        let (tx, ty) = match rotation {
            0 => (13, 14), //(14 and 15 tiles from UL)
            1 => (14, 35), //(15 and 14 tiles from LL)
            2 => (35, 34), //(14 and 15 tiles from the LR)
            3 => (34, 13), //(15 and 14 tiles from the UR)
            _ => (55, 55) // will always fail the winnow
        };
        let mut distance: u8 = 0;
        distance += (i8::abs((sx as i8)-(vx as i8)) + i8::abs((sy as i8)-(vy as i8))) as u8; // Best here is 6, wrap is irrel        
        distance += (i8::abs((tx as i8)-(sx as i8)) + i8::abs((ty as i8)-(sy as i8))) as u8; // Best here is 3, wrap is irrel
        
        return distance;
    }
}

// memoize the base maps so we're not constantly doing file reads. It doesn't have much
// actual effect, probably because windows does it anyways for you, but it makes me feel better.
std::thread_local!{
    static BASE_MAP_CACHE: RefCell<HashMap<u32, Map>> = RefCell::new(HashMap::new());
}

pub fn expected_map_header() -> [u8; 24]{
    return
        [0x4d, 0x41, 0x50, 0x20, //"MAP "
        0x00, 0x00, 0x1d, 0x64, //size
        0x00, 0x05,
        0x00, 0x05,
        0x00, 0x32, //width
        0x00, 0x32, //height
        0x00, 0x20, 0x00, 0x00,
        0x00, 0x20, 0x00, 0x00
        ];
}

fn map_validity_check(map_file: &Vec<u8>) -> Result<bool, Box<dyn Error>> {
    
    if map_file[0x660..0x678] != expected_map_header() {
        return Err("Header on base map files doesn't match!".into())
    }

    //Confirm the file is the right size
    if map_file.len() != 0x23c4 {
        return Err("Base map file is the wrong size!".into())
    }

    return Ok(true)
}

fn map_file_to_map(map_file: &Vec<u8>) -> Map {
    let mut tiles: Vec<Tile> = Vec::with_capacity(50*50);
        
    for i in 0..50*50 {
        tiles.push(Tile{
            id: map_file[i*3 + 0x678], 
            rotation: map_file[i*3+0x679] as i8, 
            height: map_file[i*3+0x67a] as i8
        });
    }

    return Map {width: 50, height: 50, tiles, dungeons: Vec::new()};
}


/*fn load_map(s: &string) -> Result<Map, Box<dyn Error>> {
    let map_file = std::fs::read(format!("./genmaps/{}.BIN", s))?
    let map = load_core_map_from_vec(map_file)
    Ok(map)
}*/

fn load_core_map_from_vec(map_file: &Vec<u8>) -> Result<Map, Box<dyn Error>> {
    map_validity_check(&map_file)?;
    let map = map_file_to_map(&map_file);
    Ok(map)
}


pub fn load_base_map(n : u32) -> Result<Map, Box<dyn Error>> {
    if let Some(map) = BASE_MAP_CACHE.with(|cache_cell| {
        let cache = cache_cell.borrow();
        cache.get(&n).cloned()
    }) {
        return Ok(map);
    };

    let map_file = std::fs::read(format!("./basemaps/GR_BASE{}.BIN", n))?; // spits out a vector
    let map = load_core_map_from_vec(&map_file)?;

    // Cache a copy of the map so we don't have to reread the file next time
    BASE_MAP_CACHE.with(|cache_cell| {
        let mut cache = cache_cell.borrow_mut();
        cache.insert(n, map.clone());
    });

    Ok(map)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    /// Loads an overworld map dumped directly from Mednafen
    /// Mednafen stores the RAM for the sega saturn in shorts instead of
    /// bytes, so we have to swap the endianness of every 2 bytes
    fn load_mednafen_map(raw_file: &[u8]) -> Map {
        let mut tiles = Vec::with_capacity(50*50);

        // Do two tiles at a time to deal with the endianness swapping
        for i in 0..(50*50*3)/6 {
            let tile1 = Tile {
                id: raw_file[i*6+1],
                rotation: raw_file[i*6+0] as i8,
                height: raw_file[i*6+3] as i8
            };

            let tile2 = Tile {
                id: raw_file[i*6+2],
                rotation: raw_file[i*6+5] as i8,
                height: raw_file[i*6+4] as i8
            };

            tiles.push(tile1);
            tiles.push(tile2);
        }

        Map{width: 50, height: 50, tiles, dungeons: Vec::new()}
    }

    // Just a random seed I generated
    #[test]
    fn FNMCNTLGHF() {
        let dumped_map = include_bytes!("../tests/FNMCNTLGHF.bin");

        let mednafen_map = load_mednafen_map(dumped_map);
        let mut generated_map = Map::from_code("FNMCNTLGHF").unwrap();

        // Replace the 0xff start tile with a default tile
        generated_map.tiles.iter_mut().find(|t| t.id == 0xff).unwrap().id = 1;
        // Replace the dungeons vector b/c the mediafen load doesn't '

        assert!(mednafen_map == generated_map);
    }

    // A map that fails to place the sealed dungeon
    #[test]
    fn GBBBTSMMBB() {
        let dumped_map = include_bytes!("../tests/GBBBTSMMBB.bin");

        let mednafen_map = load_mednafen_map(dumped_map);
        let mut generated_map = Map::from_code("GBBBTSMMBB").unwrap();

        // Replace the 0xff start tile with a default tile
        generated_map.tiles.iter_mut().find(|t| t.id == 0xff).unwrap().id = 1;

        assert!(mednafen_map == generated_map);
    }

    // A map that fails to place the volcano
    #[test]
    fn BBBBNDTLBB() {
        let dumped_map = include_bytes!("../tests/BBBBNDTLBB.bin");

        let mednafen_map = load_mednafen_map(dumped_map);
        let mut generated_map = Map::from_code("BBBBNDTLBB").unwrap();

        // Replace the 0xff start tile with a default tile
        generated_map.tiles.iter_mut().find(|t| t.id == 0xff).unwrap().id = 1;

        assert!(mednafen_map == generated_map);
    }

    // A map that takes 7 attempts to generate
    #[test]
    fn QBBDGRNQBB() {
        let dumped_map = include_bytes!("../tests/QBBDGRNQBB.bin");

        let mednafen_map = load_mednafen_map(dumped_map);
        let mut generated_map = Map::from_code("QBBDGRNQBB").unwrap();

        // Replace the 0xff start tile with a default tile
        generated_map.tiles.iter_mut().find(|t| t.id == 0xff).unwrap().id = 1;

        assert!(mednafen_map == generated_map);
    }
}

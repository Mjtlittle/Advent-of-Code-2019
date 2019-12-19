use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::f64::consts::PI;
use std::collections::HashSet;

struct AsteroidMap {
    locations: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl AsteroidMap {

    fn new() -> AsteroidMap {
        AsteroidMap {
            locations: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn new_from_file(file_path: &str) -> AsteroidMap {
        let mut new_map = AsteroidMap::new();
        
        let file = File::open(file_path).expect("Error opening file");
        let reader = BufReader::new(file);
        
        let mut char_value: u8;
        for line in reader.lines() {

            // make new row
            let mut new_row: Vec<bool> = Vec::new();

            // populate new row
            for chr in line.expect("Expected line").chars() {
                new_row.push(match chr {
                    '#' => true,
                    '.' | _ => false,
                });
            }

            // push the new row
            new_map.locations.push(new_row);
        }

        // set the width and height
        new_map.width = new_map.locations[0].len();
        new_map.height = new_map.locations.len();

        return new_map;
    }

    fn print(&self) {
        for yi in 0..self.height {
            for xi in 0..self.width {
                if self.is_asteroid(xi, yi) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    fn is_within(&self, tx: f64, ty: f64) -> bool {
        (tx >= 0.0) && (ty >= 0.0) && (tx < self.width as f64) && (ty < self.width as f64)
    }

    fn is_asteroid(&self, tx: usize, ty: usize) -> bool {
        self.locations[ty][tx]
    }

    fn count_visible_at(&self, tx: usize, ty: usize) -> usize {

        const rad_steps: usize = 3600;
        
        let mut asteroids_hit: HashSet<(usize, usize)> = HashSet::new();

        // add current asteroid if exists
        if self.is_asteroid(tx, ty) {
            asteroids_hit.insert((tx,ty));
        }

        let mut angle: f64;
        for i in 0..rad_steps {
        
            angle = (i as f64) / (rad_steps as f64) * PI * 2.0;
            
            let result = self.raycast(tx, ty, angle);
            if result.is_some() {
                asteroids_hit.insert(result.unwrap());
            }   
        }

        return asteroids_hit.len();
    }

    fn raycast(&self, tx: usize, ty: usize, angle: f64) -> Option<(usize, usize)> {

        const step_size: f64 = 0.5;

        let mut cx: f64 = (tx as f64) + 0.5;
        let mut cy: f64 = (ty as f64) + 0.5;

        // raycast from point tx,ty
        loop {

            // move the ray
            cx += angle.cos() * step_size;
            cy += angle.sin() * step_size;

            // check to see if out of map
            if !self.is_within(cx, cy){
                return None;
            }

            // round the ray to the nearest cell
            let rcx: usize = cx.floor() as usize;
            let rcy: usize = cy.floor() as usize;

            // make sure the ray is not inside the testing asteroid
            if rcx == tx && rcy == ty {
                continue;
            }

            // check to see if asteroid hit
            if self.is_asteroid(rcx, rcy){
                return Some((rcx, rcy));
            }
        }
    }
}


fn main() {
    let map = AsteroidMap::new_from_file("inputs/day_10/example_1.txt");
    map.print();
    println!("{}", map.count_visible_at(0,2));
    println!("{}", map.count_visible_at(3,4));
}
use std::fs::File;
use std::io::prelude::*;

extern crate spiral;

use spiral::ManhattanIterator;

#[derive(Copy, Clone, Debug)]
struct Dist {
	closest: Option<u8>,
	distance: u32,
}

fn distance(start: (usize, usize), end: (i32, i32)) -> u32 {
	((start.0 as i32 - end.0 as i32).abs() + (start.1 as i32 - end.1 as i32).abs()) as u32
} 

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

	let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut grid = [[Dist{closest: None, distance: 0}; 600]; 600];
    let mut points = Vec::new();
    let mut point_num = 0;

    for line in contents.lines() {
    	let mut vals = line.split(", ");
    	let centerx = vals.next().unwrap().parse::<usize>().unwrap();
    	let centery = vals.next().unwrap().parse::<usize>().unwrap(); 
    	
    	let mut manhat_iter = ManhattanIterator::new(centerx as i32, centery as i32, 600);
    	grid[centerx][centery] = Dist {closest: Some(point_num), distance: 0};
    	manhat_iter.next();    	

    	points.push(0);
    	for (x, y) in manhat_iter {
    		
    		if x >= 600 || y >= 600 || x < 0 || y < 0 {
    			continue;
    		}

    		let mut dist = &mut grid[x as usize][y as usize];
    		let currdist = distance((centerx, centery), (x, y));
    		// println!("{}", dist.distance);


    		if let Some(ref mut closest) = dist.closest {
    			if dist.distance == currdist && *closest != 100 {
					points[*closest as usize] -= 1;
					*closest = 100;
    			} else if dist.distance > currdist {
    				if *closest != 100 {
    					points[*closest as usize] -= 1;
    				}
    				points[point_num as usize] += 1;
    				dist.distance = currdist;
    				*closest = point_num;
    			}
    		} else {
    			points[point_num as usize] += 1;
    			dist.distance = currdist;
    			dist.closest = Some(point_num);
    			// println!("{} {}", dist.distance, currdist);
    		}
    	}
    	// println!("{}: {}", point_num, points[point_num as usize]);
		point_num += 1;	
    }

    for dist in grid[0].iter() {
    	if dist.closest.unwrap() == 100 {
    		continue;
    	}
    	points[dist.closest.unwrap() as usize] = 0;
    }
    for dist in grid[599].iter() {
    	if dist.closest.unwrap() == 100 {
    		continue;
    	}
    	points[dist.closest.unwrap() as usize] = 0;
    }

    for row in grid.iter() {
    	if row[0].closest.unwrap() == 100 {
    		continue;
    	}
    	points[row[0].closest.unwrap() as usize] = 0;
    }
    for row in grid.iter() {
    	if row[599].closest.unwrap() == 100 {
    		continue;
    	}
    	points[row[599].closest.unwrap() as usize] = 0;
    }

    for val in points {
    	println!("{}, ", val);
    }

    
}

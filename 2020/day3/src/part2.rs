use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(res_lines) = read_lines("./input") {
        let topologies = res_lines
            .filter_map(|res_line| res_line.ok())
            .map(|topology| {
                topology.chars()
                .map(|topology_char| topology_char == '#')
                .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();

        let topology_width = topologies[0].len();
        let mountain_height = topologies.len();
        let mut product : usize = 1;
        for (slope_width, slope_height) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let mut height_index = 0;
            let mut width_index = 0;
            let mut trees_hit = 0;
            while height_index < mountain_height {
                if topologies[height_index][width_index] {
                    trees_hit += 1;
                }
                width_index = (width_index + slope_width) % topology_width;
                height_index = height_index + slope_height;
            }
            product *= trees_hit;
        }
        println!("{}", product);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

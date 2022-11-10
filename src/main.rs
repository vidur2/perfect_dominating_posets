#![feature(string_remove_matches)]
#![feature(let_chains)]

mod interval;
mod preprocessing;

use std::{fs::File, io::Read};

use interval::Interval;
use preprocessing::parse_buff;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if let Ok(mut content) = File::open(&args[1]) {
            // Read File into adjacency matrix
            let mut buf = [0u8; 1024];
            let size = content.read(&mut buf).unwrap();
            let file_str = String::from_utf8(buf[0..size].to_vec()).unwrap();
            let (upsets_downsets, topology_list) = parse_buff(file_str);
            let interval = Interval::new(upsets_downsets);

            if let Ok(interval) = interval && let Ok(pds) = interval.find_dominating_set(topology_list){
                println!("{:?}", pds);
            } else {
                println!("Error generating dominating set, poset outside of scope of algorithm")
            }
        } else {
            println!("Error: Please enter a valid path")
        }
    } else {
        println!("Error: Please enter a valid number of arguments")
    }
}

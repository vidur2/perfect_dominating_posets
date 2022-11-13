#![feature(let_chains)]
#![feature(string_remove_matches)]

use std::{fs::File, io::Read};

use perfect_dominating_posets::interval::Interval;
use perfect_dominating_posets::preprocessing::{parse_buff, GraphConvertable, EdgeLike};

struct FileStr(String);

struct EdgeString(String);

impl GraphConvertable<EdgeString> for FileStr {
    fn split(&self) -> Vec<EdgeString> {
        let string = self.0.clone();
        let mut edge_vec: Vec<EdgeString> = Vec::new();
        for part in string.split("\n") {
            edge_vec.push(EdgeString(String::from(part)))
        }
        
        return edge_vec;
    }
}

impl EdgeLike for EdgeString {
    fn split_edge(&self) -> (u8, u8) {
        let mut edge = self.0.clone();
        edge.remove_matches("[");
        edge.remove_matches("]");

        let nums: Vec<&str> = edge.split(",").collect();

        let mut out: [u8; 2] = [0; 2];

        for (idx, num) in nums.iter().enumerate() {
            let num: u8 = num.to_string().parse().unwrap();
            out[idx] = num;
        }

        return (out[0], out[1]);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if let Ok(mut content) = File::open(&args[1]) {
            // Read File into adjacency matrix
            let mut buf = [0u8; 1024];
            let size = content.read(&mut buf).unwrap();
            let file_str = String::from_utf8(buf[0..size].to_vec()).unwrap();
            let (upsets_downsets, topology_list) = parse_buff::<EdgeString, _>(FileStr(file_str));
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

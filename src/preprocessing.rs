use std::{collections::HashMap, cmp::Ordering};

pub struct UpDown {
    pub upset: Set,
    pub downset: Set
}

#[derive(PartialEq, Eq, PartialOrd, Hash, Clone)]
pub struct Set(pub Vec<u8>);

impl Ord for Set {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0.len() > other.0.len() {
            return Ordering::Greater
        } else if self.0.len() < other.0.len() {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}

pub fn parse_buff<'a>(buff: String) -> HashMap<u8, UpDown> {
    let mut map: HashMap<u8, UpDown> = HashMap::new();
    for edge in buff.split("\n") {
        let (v1, v2) = parse_edge(edge);

        if let Some(adj_list) = map.get_mut(&v1) {
            adj_list.upset.0.push(v2);
        } else {
            map.insert(v1, UpDown { downset: Set(Vec::new()), upset: Set(vec![v2]) });
        }

        if let Some(adj_list) = map.get_mut(&v2) {
            adj_list.downset.0.push(v1);
        } else {
            map.insert(v2, UpDown { upset: Set(vec![v1]), downset: Set(Vec::new()) });
        }
    }

    return map;
}

fn parse_edge(edge: &str) -> (u8, u8) {
    let mut edge = edge.to_string();
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
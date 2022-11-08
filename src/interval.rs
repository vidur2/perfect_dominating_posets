use std::collections::HashMap;

use crate::preprocessing::{UpDown, Set};

pub struct Interval {
    interval: Vec<Node>
}

#[derive(Clone)]
struct Node {
    id: u8,
    lower: u8,
    upper: u8,
}

impl Interval {
    pub fn new(map: HashMap<u8, UpDown>) -> Self {
        let mut upset_vec: Vec<&Set> = Vec::new();
        let mut downset_vec: Vec<&Set> = Vec::new();

        for set in map.values() {
            upset_vec.push(&set.upset);
            downset_vec.push(&set.downset);
        }

        let empty_set = Set(Vec::new());
        upset_vec.push(&empty_set);
        downset_vec.push(&empty_set);


        upset_vec.sort();
        downset_vec.sort_by(|a, b| b.cmp(a));

        let mut upset_map: HashMap<Set, u8> = HashMap::new();
        let mut downset_map: HashMap<Set, u8> = HashMap::new();

        for (idx, upset) in upset_vec.iter().enumerate() {
            upset_map.insert(upset.clone().clone(), idx.try_into().unwrap());
        }

        for (idx, downset) in downset_vec.iter().enumerate() {
            downset_map.insert(downset.clone().clone(), idx.try_into().unwrap());
        }

        let mut interval: Vec<Node> = Vec::new();

        for (id, UpDown { upset, downset }) in map.iter() {
            interval.push(Node {
                id: *id,
                lower: *downset_map.get(downset).unwrap(),
                upper: *upset_map.get(upset).unwrap(),
            })
        }

        return Self {
            interval
        }
    }

    pub fn color(&self) -> HashMap<u8, Vec<u8>> {
        let mut interval = self.interval.clone();

        interval.sort_by(|a, b| a.lower.cmp(&b.lower));
        let mut color_map: HashMap<u8, u8> = HashMap::new();
        let mut coloring: HashMap<u8, Vec<u8>> = HashMap::new();

        for node in interval {
            let mut curr_color = 0;
            while let Some(idx) = color_map.get(&curr_color) {
                if idx >= &node.lower {
                    curr_color += 1;
                } else {
                    color_map.insert(curr_color, node.upper);
                    break;
                }
            }

            if let Some(colored_nodes) = coloring.get_mut(&curr_color) {
                colored_nodes.push(node.id)
            } else {
                coloring.insert(curr_color, vec![node.id]);
            }
        }

        return coloring;
    }
}
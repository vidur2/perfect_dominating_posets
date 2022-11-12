use std::{cell::RefCell, cmp::Ordering, collections::HashMap, collections::HashSet, rc::Rc};

use crate::preprocessing::{Set, UpDown, VecInner};

#[derive(Debug)]
pub struct Interval {
    interval: Vec<Node>,
}

#[derive(Clone, Debug)]
struct Node {
    id: u8,
    lower: u8,
    upper: u8,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct UnsharedSet(Vec<u8>);

impl UnsharedSet {
    fn from_set(set: Set) -> Self {
        let mut new_set: Vec<u8> = Vec::new();
        
        for elem in set.0.borrow().iter() {
            Self::inner(&mut new_set, elem);
        }

        return Self(new_set);
    }

    fn inner(new_set: &mut Vec<u8>, elem: &VecInner) {
        match elem {
            VecInner::Num(num) => new_set.push(*num),
            VecInner::Vec(deepset) => {
                for elem in deepset.upgrade().unwrap().borrow().iter() {
                    Self::inner(new_set, elem);
                }
            }
        }
    }
}
#[derive(Debug)]
struct UpDownUnshared {
    upset: UnsharedSet,
    downset: UnsharedSet,
}

pub enum ColoringError {
    TwoPlusTwoFound,
    OutsideAlgoScope
}

impl Interval {
    pub fn new(map: HashMap<u8, UpDown>) -> Result<Self, ColoringError> {
        let mut upset_vec: HashSet<UnsharedSet> = HashSet::new();
        let mut downset_vec: HashSet<UnsharedSet> = HashSet::new();
        let mut unshared_map: HashMap<u8, UpDownUnshared> = HashMap::new();

        for (id, set) in map.iter() {
            let unshared_upset = UnsharedSet::from_set(set.upset.clone());
            let unshared_downset = UnsharedSet::from_set(set.downset.clone());
            upset_vec.insert(unshared_upset.clone());
            downset_vec.insert(unshared_downset.clone());

            unshared_map.insert(
                id.clone(),
                UpDownUnshared {
                    upset: unshared_upset,
                    downset: unshared_downset,
                },
            );
        }

        let mut upset_vec: Vec<UnsharedSet> = upset_vec.into_iter().collect();
        let mut downset_vec: Vec<UnsharedSet> = downset_vec.into_iter().collect();

        if upset_vec.len() != downset_vec.len() {
            return Err(ColoringError::TwoPlusTwoFound);
        }

        upset_vec.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        downset_vec.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

        let mut upset_map: HashMap<UnsharedSet, u8> = HashMap::new();
        let mut downset_map: HashMap<UnsharedSet, u8> = HashMap::new();

        for (idx, upset) in upset_vec.iter().enumerate() {
            upset_map.insert(upset.clone(), idx.try_into().unwrap());
        }

        for (idx, downset) in downset_vec.iter().enumerate() {
            downset_map.insert(downset.clone(), idx.try_into().unwrap());
        }

        let mut interval: Vec<Node> = Vec::new();
        for (id, UpDownUnshared { upset, downset }) in unshared_map.iter() {
            interval.push(Node {
                id: *id,
                lower: *downset_map.get(&downset).unwrap(),
                upper: *upset_map.get(&upset).unwrap(),
            })
        }

        return Ok(Self { interval });
    }

    pub fn color(&self) -> HashMap<u8, HashSet<u8>> {
        let mut interval = self.interval.clone();

        interval.sort_by(|a, b| {
            let ord = a.lower.cmp(&b.lower);

            if Ordering::Equal == ord {
                return (a.upper - a.lower).cmp(&(b.upper - b.lower));
            } else {
                return ord;
            }
        });
        let mut color_map: HashMap<u8, u8> = HashMap::new();
        let mut coloring: HashMap<u8, HashSet<u8>> = HashMap::new();

        for node in interval {
            let mut curr_color = 0;
            while let Some(idx) = color_map.get(&curr_color) {
                if idx >= &node.lower {
                    curr_color += 1;
                } else {
                    break;
                }
            }

            if let Some(colored_nodes) = coloring.get_mut(&curr_color) {
                colored_nodes.insert(node.id);
            } else {
                let mut set: HashSet<u8> = HashSet::new();
                set.insert(node.id);
                coloring.insert(curr_color, set);
            }

            color_map.insert(curr_color, node.upper);
        }

        return coloring;
    }

    pub fn find_dominating_set(&self, deg_vec: HashMap<u8, HashSet<u8>>) -> Result<Vec<u8>, ColoringError>{
        let coloring = self.color();
        let mut values: Vec<HashSet<u8>> = Vec::new();
        let mut deg_vec = deg_vec.clone();
        let mut visited: HashSet<u8> = HashSet::new();

        for val in coloring.values() {
            values.push(val.clone());
        }

        if !Self::verify_chain(&values) {
            return Err(ColoringError::OutsideAlgoScope);
        }
        let mut dominating_set: Vec<u8> = Vec::new();

        values.sort_by(|a, b| b.len().cmp(&a.len()));

        let deg_len = Self::check_len(&values);

        while visited.len() != deg_len {
            for val in values.iter() {
                let mut len = 0;
                let mut adjacents: HashSet<u8> = HashSet::new();
                let mut node_id = u8::MAX;

                for node in val.iter() {
                    if !visited.contains(node) {
                        let vec = deg_vec.get(node).unwrap();

                        if vec.len() > len {
                            adjacents = vec.clone();
                            len = vec.len();
                            node_id = node.clone();
                        }
                    }
                }

                if node_id != u8::MAX {
                    dominating_set.push(node_id);
                    visited.insert(node_id);
                }

                for id in adjacents.iter() {
                    visited.insert(*id);
                    let adj_adj = deg_vec.get(&id).unwrap();

                    for id in adj_adj {
                        visited.insert(*id);
                    }

                    for val in deg_vec.values_mut() {
                        val.remove(id);
                    }
                }

                deg_vec.remove(&node_id);
            }
        }

        return Ok(dominating_set);
    }

    fn check_len(values: &Vec<HashSet<u8>>) -> usize {
        let mut length = 0;
        for val in values.iter() {
            length += val.len();
        }

        return length;
    }

    fn verify_chain(values: &Vec<HashSet<u8>>) -> bool {
        for val in values {
            if val.len() % 2 == 0 {
                return false;
            }
        }

        return true;
    }
}

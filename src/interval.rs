use std::{collections::HashMap, collections::HashSet, cmp::Ordering, cell::RefCell, rc::Rc};

use crate::preprocessing::{UpDown, Set, VecInner};

#[derive(Debug)]
pub struct Interval {
    interval: Vec<Node>
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

        return Self(new_set)
    }

    fn inner(new_set: &mut Vec<u8>, elem: &VecInner) {
        match elem {
            VecInner::Num(num) => new_set.push(*num),
            VecInner::Vec(deepset) => {
                for elem in deepset.borrow().iter() {
                    Self::inner(new_set, elem);
                }
            },
        }
    }
}
#[derive(Debug)]
struct UpDownUnshared {
    upset: UnsharedSet,
    downset: UnsharedSet
}

pub enum ColoringError {
    TwoPlusTwoFound
}

impl Interval {
    pub fn new(map: HashMap<u8, UpDown>) -> Result<Self, ColoringError> {
        let mut upset_vec: HashSet<UnsharedSet> = HashSet::new();
        let mut downset_vec: HashSet<UnsharedSet> = HashSet::new();
        let mut unshared_map: HashMap<u8, UpDownUnshared> = HashMap::new();

        for (id, set) in map.iter() {
            let unshared_upset = UnsharedSet::from_set(set.upset.clone());
            let unshared_downset  = UnsharedSet::from_set(set.downset.clone());
            upset_vec.insert(unshared_upset.clone());
            downset_vec.insert(unshared_downset.clone());

            unshared_map.insert(id.clone(), UpDownUnshared {
                upset: unshared_upset,
                downset: unshared_downset,
            });
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

        return Ok(Self {
            interval
        })
    }

    pub fn color(&self) -> HashMap<u8, HashSet<u8>> {
        let mut interval = self.interval.clone();

        interval.sort_by(|a, b| {
            let ord = a.lower.cmp(&b.lower);

            if Ordering::Equal == ord {
                return (a.upper - a.lower).cmp(&(b.upper - b.lower))
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

    pub fn find_dominating_set(&self, deg_vec: HashMap<u8, HashSet<u8>>) -> Vec<u8> {
        let coloring = self.color();
        let values: Rc<RefCell<Vec<HashSet<u8>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut deg_vec = deg_vec.clone();

        for val in coloring.values() {
            values.borrow_mut().push(val.clone());
        }
        let mut dominating_set: Vec<u8> = Vec::new();

        Rc::clone(&values).borrow_mut().sort_by(|a, b| b.len().cmp(&a.len()));
        for val in Rc::clone(&values).borrow().iter() {
            let mut len = 0;
            let mut adjacents: HashSet<u8> = HashSet::new();
            let mut node_id = u8::MAX;

            for node in val.iter() {
                let vec = deg_vec.get(node).unwrap();

                if vec.len() > len {
                    adjacents = vec.clone();
                    len = vec.len();
                    node_id = node.clone();
                }

            }

            drop(val);

            dominating_set.push(node_id);

            for id in adjacents.iter() {
                for set in Rc::clone(&values).borrow_mut().iter_mut() {
                    set.remove(id);
                }

                for val in deg_vec.values_mut() {
                    val.remove(id);
                }
            }
        }

        return dominating_set;
    }
}
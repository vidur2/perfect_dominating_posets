use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;


// Stores upsets and downsets
#[derive(Debug)]
pub struct UpDown {
    pub upset: Set,
    pub downset: Set,
}

// Insert shared memory into Set Vec
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub enum VecInner {
    Num(u8),
    Vec(Rc<RefCell<Vec<VecInner>>>),
}

// Struct to perform derive operations on (access to Ordering and cloning)
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Set(pub Rc<RefCell<Vec<VecInner>>>);

// Parsing method, reads file into Upsets and Downsets
pub fn parse_buff(buff: String) -> (HashMap<u8, UpDown>, HashMap<u8, HashSet<u8>>) {

    // Declaring of upset/downset and degree list
    let mut map: HashMap<u8, UpDown> = HashMap::new();
    let mut topology_list: HashMap<u8, HashSet<u8>> = HashMap::new();

    // Iterating through file
    for edge in buff.split("\n") {

        // Splitting file into meaningful edges
        let (v1, v2) = parse_edge(edge);

        // Adding to adjacency list
        if let Some(deg) = topology_list.get_mut(&v1) {
            deg.insert(v2);
        } else {
            let mut deg = HashSet::new();
            deg.insert(v2);
            topology_list.insert(v1, deg);
        }

        // Same as above
        if let Some(deg) = topology_list.get_mut(&v2) {
            deg.insert(v1);
        } else {
            let mut deg = HashSet::new();
            deg.insert(v1);
            topology_list.insert(v2, deg);
        }

        // Adding to upset/downsets
        if let Some(adj_list) = map.get_mut(&v1) {
            adj_list.upset.0.borrow_mut().push(VecInner::Num(v2));
        } else {
            map.insert(
                v1,
                UpDown {
                    downset: Set(Rc::new(RefCell::new(Vec::new()))),
                    upset: Set(Rc::new(RefCell::new(vec![VecInner::Num(v2)]))),
                },
            );
        }

        let new_item2 = Rc::clone(&map.get(&v1).unwrap().downset.0);
        if let Some(adj_list) = map.get_mut(&v2) {
            let mut borrow = adj_list.downset.0.borrow_mut();
            borrow.push(VecInner::Num(v1));
            borrow.push(VecInner::Vec(new_item2));
        } else {
            map.insert(
                v2,
                UpDown {
                    downset: Set(Rc::new(RefCell::new(vec![
                        VecInner::Num(v1),
                        VecInner::Vec(new_item2),
                    ]))),
                    upset: Set(Rc::new(RefCell::new(Vec::new()))),
                },
            );
        }

        let new_item = Rc::clone(&map.get(&v2).unwrap().upset.0);
        let mut downlist = map.get_mut(&v1).unwrap().upset.0.borrow_mut();

        downlist.push(VecInner::Vec(new_item));
    }

    return (map, topology_list);
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

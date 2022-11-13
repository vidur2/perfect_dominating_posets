use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::rc::Weak;


// Stores upsets and downsets
#[derive(Debug, Clone)]
pub struct UpDown {
    pub upset: Set,
    pub downset: Set,
}

// Insert shared memory into Set Vec
#[derive(Debug, Clone)]
pub enum VecInner {
    Num(u8),
    Vec(Weak<RefCell<Vec<VecInner>>>),
}

// Struct to perform derive operations on (access to Ordering and cloning)
#[derive(Debug, Clone)]
pub struct Set(pub Rc<RefCell<Vec<VecInner>>>);

pub trait GraphConvertable<T: EdgeLike> {
    fn split(&self) -> Vec<T>;
}

pub trait EdgeLike {
    fn split_edge(&self) -> (u8, u8);
}

// Parsing method, reads file into Upsets and Downsets
pub fn parse_buff<I: EdgeLike, T: GraphConvertable<I>>(buff: T) -> (HashMap<u8, UpDown>, HashMap<u8, HashSet<u8>>) {

    // Declaring of upset/downset and degree list
    let mut map: HashMap<u8, UpDown> = HashMap::new();
    let mut topology_list: HashMap<u8, HashSet<u8>> = HashMap::new();

    // Iterating through file
    for edge in buff.split() {

        // Splitting file into meaningful edges
        let (v1, v2) = edge.split_edge();

        // Adding to adjacency list
        add_to_toplist(&mut topology_list, v1, v2);

        // Same as above
        add_to_toplist(&mut topology_list, v2, v1);

        // Adding to upset/downsets
        add_to_updown(&mut map, v1, v2);

        add_to_updown2nd(&mut map, v1, v2);

        let new_item = Rc::clone(&map.get(&v2).unwrap().upset.0);
        let mut downlist = map.get_mut(&v1).unwrap().upset.0.borrow_mut();

        downlist.push(VecInner::Vec(Rc::downgrade(&new_item)));
    }

    return (map, topology_list);
}

fn add_to_updown2nd(map: &mut HashMap<u8, UpDown>, v1: u8, v2: u8) {
    let new_item2 = Rc::clone(&map.get(&v1).unwrap().downset.0);
    if let Some(adj_list) = map.get_mut(&v2) {
        let mut borrow = adj_list.downset.0.borrow_mut();
        borrow.push(VecInner::Num(v1));
        borrow.push(VecInner::Vec(Rc::downgrade(&new_item2)));
    } else {
        map.insert(
            v2,
            UpDown {
                downset: Set(Rc::new(RefCell::new(vec![
                    VecInner::Num(v1),
                    VecInner::Vec(Rc::downgrade(&new_item2)),
                ]))),
                upset: Set(Rc::new(RefCell::new(Vec::new()))),
            },
        );
    }
}

fn add_to_updown(map: &mut HashMap<u8, UpDown>, v1: u8, v2: u8) {
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
}

fn add_to_toplist(topology_list: &mut HashMap<u8, HashSet<u8>>, v1: u8, v2: u8) {
    if let Some(deg) = topology_list.get_mut(&v1) {
        deg.insert(v2);
    } else {
        let mut deg = HashSet::new();
        deg.insert(v2);
        topology_list.insert(v1, deg);
    }
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

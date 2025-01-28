use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

fn main() {
    let mut v = vec![1,2,3,4,5];
    v.remove(1);
    println!("{:?}", v);
    v.swap_remove(0);
    println!("{:?}", v);
    v.insert(0, 99);
    println!("{:?}", v);
    let mut vd = VecDeque::from([1,2,3,4,5]);
    vd.push_back(6);
    vd.push_back(7);
    vd.push_back(8);
    vd.pop_back();
    vd.pop_front();

    let mut hm = HashMap::new();
    hm.insert("Ísland", 30);
    hm.insert("Grænland", 30);
    hm.insert("Færeyjar", 30);
    hm.insert("Danmörk", 30);
    hm.insert("Noregur", 30);
    hm.insert("England", 23);

    println!("{:?}", hm);

    let mut lond = HashSet::new();
    lond.insert("Ísland");
    lond.insert("Ísland");
    lond.insert("Ísland");
    lond.insert("Ísland");
    lond.insert("Ísland");
    lond.insert("Danmörk");
    println!("{:?}", lond);

    let mut bm = BTreeMap::new();
    bm.insert("Ísland".to_string(), 30);
    bm.insert("Grænland".to_string(), 30);
    bm.insert("Færeyjar".to_string(), 30);
    bm.insert("Danmörk".to_string(), 30);
    bm.insert("Noregur".to_string(), 30);
    bm.insert("England".to_string(), 23);

    println!("{:?}", bm);

    let mut bh = BinaryHeap::new();
    bh.push(Reverse(10));
    bh.push(Reverse(5));
    bh.push(Reverse(20));
    println!("{:?}", bh);

}

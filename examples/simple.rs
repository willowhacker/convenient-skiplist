/// Run this example with cargo run --example simple
use convenient_skiplist::SkipList;

fn main() {

    let mut sk = SkipList::new();
    
    println!("insert 0.0: positin:{}",sk.insert(0.));
    println!("insert 1.0: positin:{}",sk.insert(1.));
    println!("insert 1.0: positin:{}",sk.insert(1.));
    println!("insert 2.0: positin:{}",sk.insert(2.));
    for v in [-1.0, 0.0, 1.0, 2.0, 3.0] {
        println!("{:?}, index_of: {:?}，min_rank: {:?}，max_rank: {:?}", &v, sk.index_of(&v), sk.min_rank(&v), sk.max_rank(&v));
    }

    println!("{:?}",sk);

    sk.remove(&1.);

    println!("{:?}",sk);

    sk.remove(&1.);

    println!("{:?}",sk);

    sk.remove(&1.);

    println!("{:?}",sk);

}


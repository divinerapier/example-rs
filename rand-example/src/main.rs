use rand::seq::IteratorRandom;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let slice = vec![0, 1, 2, 3, 4, 5];
    println!("{:?}", slice.iter().choose(&mut rng));
    println!("{:?}", slice.iter().choose(&mut rng));
    println!("{:?}", slice.iter().choose(&mut rng));
    println!("{:?}", slice.iter().choose(&mut rng));
    println!("{:?}", slice.iter().choose(&mut rng));
}

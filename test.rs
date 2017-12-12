fn main() {
    let mut a = (0..10).collect::<Vec<_>>();
    a.drain(0..6);
    println!("{:?}", a);
}
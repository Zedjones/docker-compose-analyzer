use std::env;
use std::fs::File;

fn main() {
    let yaml_path = env::args().nth(1).unwrap();
    let f = File::open(&yaml_path).unwrap();
    let some_data = serde_yaml::from_reader::<File, String>(f).unwrap();
    println!("{:?}", some_data);
}

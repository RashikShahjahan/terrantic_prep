use std::fs::File;
use std::path::Path;
use std::io::Read;

//  Create a Rust program that reads lines from a text file and counts the frequency of each word.
fn wordcount(file_path:&str)->usize{
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    };

    let v: Vec<&str> = s.split(' ').collect();
    let count = v.len();

    return count;
}

fn main(){
    println!("{}",wordcount("hello.txt"));
}
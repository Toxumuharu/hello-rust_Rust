// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};
use rand::{thread_rng, Rng};

fn main() {
    const array_size: usize = 10;
    let mut array1: [i32; array_size] = [-1; array_size];
    let mut array2: [i32; array_size] = [-1; array_size];
    let mut rng = thread_rng();

    for i in 0..array_size {
        array1[i] = rng.gen();
    }

    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();

    sort_array(&mut array1, &mut array2);
}

fn sort_array(array1: &mut [i32], array2: &mut [i32]){
    for i in array1.iter() {
        println!("{}", i);
        for j in array1.iter() {
            print!(" {}",j)
        }
    } 
}


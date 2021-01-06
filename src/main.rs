#[allow(unused_imports)]
use rand::{thread_rng, Rng};

fn main() {
    const ARRAY_SIZE: usize = 10;
    const ARRAY_RANGE_MIN: i32 = 0;
    const ARRAY_RANGE_MAX: i32 = 100;
    let mut array1: [i32; ARRAY_SIZE] = [-1; ARRAY_SIZE];
    let mut rng = thread_rng();

    for i in 0..ARRAY_SIZE {
        array1[i] = rng.gen_range(ARRAY_RANGE_MIN, ARRAY_RANGE_MAX);
    }
    println!("{:?}", array1);

    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();

    sort_array(&mut array1);
    println!("{:?}", array1);
}

fn sort_array(array1: &mut [i32]){
    let mut temp: i32;
    for i in 0..array1.len() {
        for j in 0..array1.len() {
            if array1[i] < array1[j] {
                temp = array1[i];
                array1[i] = array1[j];
                array1[j] = temp;
          }
        }
    } 
}


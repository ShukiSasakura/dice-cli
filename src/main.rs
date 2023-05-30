use rand::Rng;
use std::io;

fn main() {
    println!("please input by dice notation  e.g. 2d6");
    let mut rng = rand::thread_rng();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let v: Vec<&str> = buffer.trim().split('d').collect();
    let n = match v[0].parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("the number of dice is none."),
    };
    let face_num = match v[1].parse::<u32>() {
        Ok(face_num) => face_num,
        Err(_) => panic!("the number of faces is none."),
    };

    let mut sum = 0u32;
    for i in 0..n {
        let random_num = rng.gen_range(1..=face_num);
        sum += random_num;
        println!("random number {} = {}", i, random_num);
    }
    println!("sum = {}", sum);
}

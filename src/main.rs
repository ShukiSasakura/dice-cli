use rand::Rng;
use regex::Regex;
use std::io;

fn main() -> Result<(),()> {
    println!("please input by dice notation  e.g. 2d6");
    let mut rng = rand::thread_rng();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let re = Regex::new(r"^(\d+)d(\d+)$").unwrap();
    let (n_str, face_num_str) = match re.captures(&buffer.trim()) {
        Some(caps) => (caps[1].to_string(), caps[2].to_string()),
        None => {
            eprintln!("Invalid expression. Please input dice cord.  e.g. 2d6");
            return Err(());
        }
    };

    let n = n_str.parse::<u32>().unwrap();
    let face_num = face_num_str.parse::<u32>().unwrap();

    let mut sum = 0u32;
    for i in 0..n {
        let random_num = rng.gen_range(1..=face_num);
        sum += random_num;
        println!("random number {} = {}", i, random_num);
    }
    println!("sum = {}", sum);

    Ok(())
}

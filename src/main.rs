use std::env;
use std::fs::File;
use std::io::{ BufRead, BufReader};
use rand::Rng;

fn main()-> Result<(), Box<dyn std::error::Error>>  {
    let args: Vec<String> = env::args().collect();
    let file=args[2].to_string();
    let mut thinks: Vec<std::string::String> = Vec::new();
    let stri: String = String::from(args[3].to_string());
    let size: i32 = stri.parse().unwrap();
    for result in BufReader::new(File::open(file)?).lines() {
        let l = result?;
        if l!="" {
            thinks.push(l);
        }
    }
    let mut rng = rand::thread_rng();
    let len=thinks.len();
    for i in 0..size {
        println!("{} {}",i,thinks[rng.gen_range(0, len)]);
    }
    Ok(())
}

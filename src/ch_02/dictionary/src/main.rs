use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let dic_file = "dict.txt";

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("[USAGE] .dictionary word");
        process::exit(1);
    }

    let word = &args[1];

    let fp = File::open(dic_file).unwrap();

    let reader = BufReader::new(fp);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(_) = line.find(word) {
            println!("{}", line);
        }
    }
}

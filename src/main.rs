mod file;
use std::io::BufReader;
use std::io::Read;
use std::process::exit;

const FILEPATH: &'static str = "/etc/hosts";

fn main() {
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(FILEPATH);

    match file {
        Ok(f) => {
            let mut buff = BufReader::new(f);
            let mut contents = String::new();

            buff.read_to_string(&mut contents).unwrap();

            let lines = contents
                .lines()
                .into_iter()
                .map(|line| line.split(' ').collect::<Vec<_>>().pop().unwrap())
                .skip(4) // The first two should just be  localhost
                .collect::<Vec<_>>();

            println!("{lines:?}");
        }
        Err(_) => {
            eprintln!("You must run this tool in priveliged mode!");
            exit(126);
        }
    }
}

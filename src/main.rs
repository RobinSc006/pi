use pi::PiCache;
use read_input::prelude::*;

mod pi;

fn main() {
    let pi_cache = PiCache::calculate(1000000);
    println!("generated PI");
    println!("{}", pi_cache.get_digits_in_range_str((0, 69)));

    loop {
        let input_raw: String = input().msg("$ ").get();
        let args: Vec<&str> = input_raw.split(' ').collect();

        match args[0] {
            "search" => {
                println!("index: {}", pi_cache.search(args[1].to_string()));
            }
            "exit" => {
                std::process::exit(0);
            }
            _ => (),
        }
    }
}
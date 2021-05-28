use pi::PiCache;
use read_input::prelude::*;

mod pi;

fn main() {
    let pi_cache = PiCache::calculate(10000000);
    println!("generated PI");
    print_digits_prec(&pi_cache, 15);

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

fn print_digits_prec(pi_cache: &PiCache, prec: usize) {
    let mut digits_str = String::new();

    for ch in pi_cache.get_digits_to_prec(prec).iter() {
        digits_str.push(*ch as char);
    }
    println!("{}", &digits_str);
}

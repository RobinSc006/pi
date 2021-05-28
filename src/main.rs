use pi::PiCache;

mod pi;

fn main() {
    let pi_cache = PiCache::calculate(1000000);

    print!("index: {}", pi_cache.search("9999".to_string()));
}

fn print_digits_prec(pi_cache:PiCache, prec: usize) {
    let mut digits_str = String::new();
    for ch in pi_cache.get_digits_to_prec(prec).iter() {
        digits_str.push(*ch as char);
    }
    println!("{}", &digits_str);
}

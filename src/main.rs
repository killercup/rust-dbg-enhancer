use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let res = rust_dbg_enhancer::enhance(&input).unwrap();
    println!("{}", res);
}

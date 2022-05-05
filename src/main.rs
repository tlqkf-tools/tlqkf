mod arguments;
use arguments::*;

fn main() {
    let args = Arguments(std::env::args().skip(1).collect::<Vec<String>>());
    println!("{:?}", args.contains("test"));
}

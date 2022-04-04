use nothing::{Nothing, Probably, Something};

fn get_args() -> Probably<Vec<String>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0 => Nothing,
        _ => Something(args),
    }
}

fn main() -> Probably<Vec<String>> {
    let args = get_args();
    println!("args = {args:?}");
    args
}

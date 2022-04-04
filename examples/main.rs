//! $ cargo run --example main; echo $?
//! args = Nothing
//! 1
//!
//! $ cargo run --example main Probably Nothing; echo $?
//! args = Something(["Probably", "Nothing"])
//! 0
//!
use nothing::{Nothing, Probably, Something};

fn get_args() -> Probably<Vec<String>> {
    match std::env::args().collect::<Vec<String>>() {
        args @ _ if args.len() > 1 => Something(args),
        _ => Nothing,
    }
}

fn main() -> Probably<Vec<String>> {
    let args = get_args();
    println!("args = {args:?}");
    args
}

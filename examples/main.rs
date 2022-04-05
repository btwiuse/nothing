//! $ cargo run --example main
//! some_args = Nothing
//!
//! $ echo $?
//! 1
//!
//! $ cargo run --example main Probably Nothing
//! some_args = Something(["Probably", "Nothing"])
//! args = ["Probably", "Nothing"]
//!
//! $ echo $?
//! 0
//!
use nothing::{Nothing, Probably, Something};

fn get_args() -> Probably<Vec<String>> {
    match std::env::args().skip(1).collect::<Vec<String>>() {
        args @ _ if args.len() > 0 => Something(args),
        _ => Nothing,
    }
}

fn main() -> Probably<Vec<String>> {
    let some_args = get_args();

    println!("some_args = {some_args:?}");

    // you can use ? operator to extract the inner value, and
    // return immediately if it's Nothing. Roughly equivalent
    // to the following code:
    //
    //   let args = match some_args {
    //       Something(v) => v,
    //       Nothing => return Nothing,
    //   };

    let args = some_args?;

    println!("args = {args:?}");

    Something(args)
}

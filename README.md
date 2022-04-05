nothing
=======

[![crates.io](https://img.shields.io/crates/v/nothing.svg)](https://crates.io/crates/nothing)
[![Documentation](https://docs.rs/nothing/badge.svg)](https://docs.rs/nothing)
[![Build Status](https://travis-ci.org/btwiuse/nothing.svg?branch=master)](https://travis-ci.org/btwiuse/nothing)

nothing::[Probably] is a better [Option].

```
pub enum Probably<T> {
    Nothing,
    Something(T),
}
```

# Why?

The point is that you can use [Probably] as the return type of your main function:

```
use nothing::{Probably, Nothing};

fn main() -> Probably<()> {
    Nothing
}
```

Exit code is `0` if it is [Something], `1` if [Nothing]. 

You can even use the `?` operator the way you would with [Option] and [Result]. See [./examples/main.rs](https://github.com/btwiuse/nothing/blob/master/examples/main.rs)

![Probably::Nothing](https://camo.githubusercontent.com/8bfa566db90d366cb0dd026267f78a7dfca0c3193cb84172b90d05b594b7062c/68747470733a2f2f692e696d6775722e636f6d2f41754464624f4b2e706e67)

Probably nothing.

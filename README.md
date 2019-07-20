Debug Helper
====================

[![Build Status](https://travis-ci.org/magiclen/debug-helper.svg?branch=master)](https://travis-ci.org/magiclen/debug-helper)
[![Build status](https://ci.appveyor.com/api/projects/status/8jtt1s8wpontnmu1/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/debug-helper/branch/master)

This crate provides declarative macros to help you implement the `Debug` trait manually.

## Examples

For structs,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A {
    pub f1: u8,
    pub f2: i16,
    pub f3: f64,
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_struct!(A, f, self, .f1, (.f3, "{:.3}", self.f3));
    }
}

let a = A {
    f1: 1,
    f2: 2,
    f3: std::f64::consts::PI,
};

println!("{:#?}", a);

/*
    A {
        f1: 1,
        f3: 3.142,
    }
*/
```

For tuple structs,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A(pub u8, pub i16, pub f64);

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_tuple_struct!(A, f, self, .0, (.2, "{:.3}", self.2));
    }
}

let a = A(1, 2, std::f64::consts::PI);

println!("{:#?}", a);

/*
    A(
        1,
        3.142,
    )
*/
```

For enums,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub enum A {
    V1,
    V2(u8, i16, f64),
    V3 {
        f1: u8,
        f2: i16,
        f3: f64,
    },
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_enum!(A::{V1, (V2(f1, _, f3): (.f1, (.f3, "{:.3}", f3))), {V3{f1, f2: _, f3}: (.f1, (.f3, "{:.3}", f3))}}, f, self);
    }
}

let a = A::V1;
let b = A::V2(1, 2, std::f64::consts::PI);
let c = A::V3{
    f1: 1,
    f2: 2,
    f3: std::f64::consts::PI,
};

println!("{:#?}", a);
println!("{:#?}", b);
println!("{:#?}", c);

/*
    V1
    V2(
        1,
        3.142,
    )
    V3 {
        f1: 1,
        f3: 3.142,
    }
*/
```

You can add named fields even if they are not defined in your type,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A {
    pub f1: u8,
    pub f2: i16,
    pub f3: f64,
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_struct!(A, f, self, .f1, (.f3, "{:.3}", self.f3), (.sum, "{:.3}", self.f1 as f64 + self.f2 as f64 + self.f3));
    }
}

let a = A {
    f1: 1,
    f2: 2,
    f3: std::f64::consts::PI,
};

println!("{:#?}", a);

/*
    A {
        f1: 1,
        f3: 3.142,
        sum: 6.142,
    }
*/
```

In most cases, you can use the [`derivative`](https://crates.io/crates/derivative) crate to implement the `Debug` trait.

## Crates.io

https://crates.io/crates/debug-helper

## Documentation

https://docs.rs/debug-helper

## License

[MIT](LICENSE)
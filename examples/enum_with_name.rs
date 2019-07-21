#[macro_use(impl_debug_for_enum)]
extern crate debug_helper;

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
        impl_debug_for_enum!({A::V1, (V2(f1, _, f3): (.f1, (.f3, "{:.3}", f3))), {V3{f1, f2: _, f3}: (.f1, (.f3, "{:.3}", f3))}}, f, self);
    }
}

fn main() {
    let a = A::V1;
    let b = A::V2(1, 2, std::f64::consts::PI);
    let c = A::V3 {
        f1: 1,
        f2: 2,
        f3: std::f64::consts::PI,
    };

    println!("{:#?}", a);
    println!("{:#?}", b);
    println!("{:#?}", c);

    /*
        A::V1
        A::V2(
            1,
            3.142,
        )
        A::V3 {
            f1: 1,
            f3: 3.142,
        }
    */
}
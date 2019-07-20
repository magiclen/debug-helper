#[macro_use(impl_debug_for_tuple_struct)]
extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A {
    pub f1: u8,
    pub f2: i16,
    pub f3: f64,
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_tuple_struct!(A, f, self, let .0 = self.f1, let .1 = self.f2, let .2 = self.f3);
    }
}

fn main() {
    let a = A {
        f1: 1,
        f2: 2,
        f3: std::f64::consts::PI,
    };

    println!("{:#?}", a);

    /*
        A(
            1,
            2,
            3.141592653589793,
        )
    */
}
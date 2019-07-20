#[macro_use(impl_debug_for_struct)]
extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A(pub u8, pub i16, pub f64);

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_struct!(A, f, self, let .f1 = self.0, let .f2 = self.1, let .f3 = self.2);
    }
}

fn main() {
    let a = A(1, 2, std::f64::consts::PI);

    println!("{:#?}", a);

    /*
        A {
            f1: 1,
            f2: 2,
            f3: 3.141592653589793,
        }
    */
}
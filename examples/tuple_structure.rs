#[macro_use(impl_debug_for_tuple_struct)]
extern crate debug_helper;

use std::fmt::{self, Debug, Formatter};

pub struct A(pub u8, pub i16, pub f64);

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_tuple_struct!(A, f, self, .0, (.2, "{:.3}", self.2));
    }
}

fn main() {
    let a = A(1, 2, std::f64::consts::PI);

    println!("{:#?}", a);

    /*
        A(
            1,
            3.142,
        )
    */
}

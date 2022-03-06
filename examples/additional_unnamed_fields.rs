use std::fmt::{self, Debug, Formatter};

pub struct A(pub u8, pub i16, pub f64);

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        debug_helper::impl_debug_for_tuple_struct!(A, f, self, .0, (.2, "{:.3}", self.2), (.3, "{:.3}", self.0 as f64 + self.1 as f64 + self.2));
    }
}

fn main() {
    let a = A(1, 2, std::f64::consts::PI);

    println!("{:#?}", a);

    /*
        A(
            1,
            3.142,
            6.142,
        )
    */
}

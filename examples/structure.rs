use std::fmt::{self, Debug, Formatter};

pub struct A {
    pub f1: u8,
    pub f2: i16,
    pub f3: f64,
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        debug_helper::impl_debug_for_struct!(A, f, self, .f1, (.f3, "{:.3}", self.f3));
    }
}

fn main() {
    let a = A {
        f1: 1, f2: 2, f3: std::f64::consts::PI
    };

    println!("{:#?}", a);

    /*
        A {
            f1: 1,
            f3: 3.142,
        }
    */
}

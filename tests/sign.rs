#[macro_use(impl_debug_for_struct)]
extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

#[test]
fn signed_values() {
    #[derive(Debug)]
    struct Inner {
        f1: u32,
        f2: i64,
    }

    struct MyStruct {
        f1: u8,
        f2: i16,
        f3: f64,
        f4: Inner,
    }

    impl Debug for MyStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_struct!(MyStruct, f, self, .f1, .f2, .f3, .f4);
        }
    }

    let my_struct = MyStruct {
        f1: 1,
        f2: -2,
        f3: 3.0,
        f4: Inner {
            f1: 4,
            f2: -5,
        },
    };

    assert_eq!("MyStruct { f1: +1, f2: -2, f3: +3.0, f4: Inner { f1: +4, f2: -5 } }", format!("{:+?}", my_struct));
    assert_eq!("MyStruct {\n    f1: +1,\n    f2: -2,\n    f3: +3.0,\n    f4: Inner {\n        f1: +4,\n        f2: -5,\n    },\n}", format!("{:+#?}", my_struct));
}
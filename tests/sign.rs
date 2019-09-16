#![allow(clippy::unreadable_literal)]

#[macro_use(impl_debug_for_struct)]
extern crate debug_helper;

use std::fmt::{self, Debug, Formatter};

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

    assert_eq!(
        "MyStruct { f1: +1, f2: -2, f3: +3.0, f4: Inner { f1: +4, f2: -5 } }",
        format!("{:+?}", my_struct)
    );
    assert_eq!("MyStruct {\n    f1: +1,\n    f2: -2,\n    f3: +3.0,\n    f4: Inner {\n        f1: +4,\n        f2: -5,\n    },\n}", format!("{:+#?}", my_struct));
}

#[test]
fn zero_values() {
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

    assert_eq!(
        "MyStruct { f1: 00001, f2: -0002, f3: 003.0, f4: Inner { f1: 00004, f2: -0005 } }",
        format!("{:05?}", my_struct)
    );
    assert_eq!(
        "MyStruct { f1: 00001, f2: 000-2, f3: 003.0, f4: Inner { f1: 00004, f2: 000-5 } }",
        format!("{:0>5?}", my_struct)
    );
    assert_eq!(
        "MyStruct { f1: +0001, f2: -0002, f3: +03.0, f4: Inner { f1: +0004, f2: -0005 } }",
        format!("{:+05?}", my_struct)
    );
    assert_eq!(
        "MyStruct { f1: 000+1, f2: 000-2, f3: 0+3.0, f4: Inner { f1: 000+4, f2: 000-5 } }",
        format!("{:0>+5?}", my_struct)
    );
    assert_eq!("MyStruct {\n    f1: 00001,\n    f2: -0002,\n    f3: 003.0,\n    f4: Inner {\n        f1: 00004,\n        f2: -0005,\n    },\n}", format!("{:#05?}", my_struct));
    assert_eq!("MyStruct {\n    f1: 00001,\n    f2: 000-2,\n    f3: 003.0,\n    f4: Inner {\n        f1: 00004,\n        f2: 000-5,\n    },\n}", format!("{:0>#5?}", my_struct));
    assert_eq!("MyStruct {\n    f1: +0001,\n    f2: -0002,\n    f3: +03.0,\n    f4: Inner {\n        f1: +0004,\n        f2: -0005,\n    },\n}", format!("{:+#05?}", my_struct));
    assert_eq!("MyStruct {\n    f1: 000+1,\n    f2: 000-2,\n    f3: 0+3.0,\n    f4: Inner {\n        f1: 000+4,\n        f2: 000-5,\n    },\n}", format!("{:0>+#5?}", my_struct));
}

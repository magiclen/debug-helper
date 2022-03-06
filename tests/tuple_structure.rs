use std::fmt::{self, Debug, Formatter};

#[test]
#[allow(dead_code)]
fn unit() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(Outer, f, self);
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("Outer", format!("{:?}", outer));
    assert_eq!("Outer", format!("{:#?}", outer));
    assert_eq!("Outer", format!("{:010.2?}", outer));
    assert_eq!("Outer", format!("{:0>10.2?}", outer));
    assert_eq!("Outer", format!("{:?^10.2?}", outer));
    assert_eq!("Outer", format!("{:#<10.2?}", outer));
    assert_eq!("Outer", format!("{:#<#10.2?}", outer));
}

#[test]
#[allow(dead_code)]
fn unit_renamed() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(A, f, self);
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("A", format!("{:?}", outer));
    assert_eq!("A", format!("{:#?}", outer));
    assert_eq!("A", format!("{:010.2?}", outer));
    assert_eq!("A", format!("{:0>10.2?}", outer));
    assert_eq!("A", format!("{:?^10.2?}", outer));
    assert_eq!("A", format!("{:#<10.2?}", outer));
    assert_eq!("A", format!("{:#<#10.2?}", outer));
}

#[test]
#[allow(dead_code)]
fn one_field_primitive() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(Outer, f, self, .0);
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("Outer(1.23456789)", format!("{:?}", outer));
    assert_eq!("Outer(\n    1.23456789,\n)", format!("{:#?}", outer));
    assert_eq!("Outer(0000001.23)", format!("{:010.2?}", outer));
    assert_eq!("Outer(0000001.23)", format!("{:0>10.2?}", outer));
    assert_eq!("Outer(???1.23???)", format!("{:?^10.2?}", outer));
    assert_eq!("Outer(1.23######)", format!("{:#<10.2?}", outer));
    assert_eq!("Outer(\n    1.23######,\n)", format!("{:#<#10.2?}", outer));
}

#[test]
#[allow(dead_code)]
fn one_field_nested() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(Outer, f, self, .1);
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("Outer(Inner { f1: 5, f2: 10 })", format!("{:?}", outer));
    assert_eq!(
        "Outer(\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)",
        format!("{:#?}", outer)
    );
    assert_eq!("Outer(Inner { f1: 0000000005, f2: 0000000010 })", format!("{:010.2?}", outer));
    assert_eq!("Outer(Inner { f1: 0000000005, f2: 0000000010 })", format!("{:0>10.2?}", outer));
    assert_eq!("Outer(Inner { f1: ????5?????, f2: ????10???? })", format!("{:?^10.2?}", outer));
    assert_eq!("Outer(Inner { f1: 5#########, f2: 10######## })", format!("{:#<10.2?}", outer));
    assert_eq!(
        "Outer(\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)",
        format!("{:#<#10.2?}", outer)
    );
}

#[test]
fn all_renamed() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(A, f, self, .0, .1);
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("A(1.23456789, Inner { f1: 5, f2: 10 })", format!("{:?}", outer));
    assert_eq!(
        "A(\n    1.23456789,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)",
        format!("{:#?}", outer)
    );
    assert_eq!(
        "A(0000001.23, Inner { f1: 0000000005, f2: 0000000010 })",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "A(0000001.23, Inner { f1: 0000000005, f2: 0000000010 })",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "A(???1.23???, Inner { f1: ????5?????, f2: ????10???? })",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "A(1.23######, Inner { f1: 5#########, f2: 10######## })",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!("A(\n    1.23######,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)", format!("{:#<#10.2?}", outer));
}

#[test]
#[allow(dead_code)]
fn custom_fmt() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Intruder {
        s: &'static str,
    }

    struct Outer(f64, Inner, Intruder);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(Outer, f, self, (.0, "number"), .1, (.2, "{}", self.2.s));
        }
    }

    let outer = Outer(
        1.23456789,
        Inner {
            f1: 5,
            f2: 10,
        },
        Intruder {
            s: "Hi",
        },
    );

    assert_eq!("Outer(number, Inner { f1: 5, f2: 10 }, Hi)", format!("{:?}", outer));
    assert_eq!(
        "Outer(\n    number,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n    Hi,\n)",
        format!("{:#?}", outer)
    );
    assert_eq!(
        "Outer(number, Inner { f1: 0000000005, f2: 0000000010 }, Hi)",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "Outer(number, Inner { f1: 0000000005, f2: 0000000010 }, Hi)",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "Outer(number, Inner { f1: ????5?????, f2: ????10???? }, Hi)",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "Outer(number, Inner { f1: 5#########, f2: 10######## }, Hi)",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!("Outer(\n    number,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    Hi,\n)", format!("{:#<#10.2?}", outer));
}

#[test]
fn additional_fields() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(Outer, f, self, .0, .1, (.2, "Hi"));
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("Outer(1.23456789, Inner { f1: 5, f2: 10 }, Hi)", format!("{:?}", outer));
    assert_eq!(
        "Outer(\n    1.23456789,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n    Hi,\n)",
        format!("{:#?}", outer)
    );
    assert_eq!(
        "Outer(0000001.23, Inner { f1: 0000000005, f2: 0000000010 }, Hi)",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "Outer(0000001.23, Inner { f1: 0000000005, f2: 0000000010 }, Hi)",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "Outer(???1.23???, Inner { f1: ????5?????, f2: ????10???? }, Hi)",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "Outer(1.23######, Inner { f1: 5#########, f2: 10######## }, Hi)",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!("Outer(\n    1.23######,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    Hi,\n)", format!("{:#<#10.2?}", outer));
}

#[test]
#[allow(dead_code)]
fn fake_tuple_struct() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer {
        f1: f64,
        f2: Inner,
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            debug_helper::impl_debug_for_tuple_struct!(Outer, f, self, let .0 = self.f1);
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer(1.23456789)", format!("{:?}", outer));
    assert_eq!("Outer(\n    1.23456789,\n)", format!("{:#?}", outer));
    assert_eq!("Outer(0000001.23)", format!("{:010.2?}", outer));
    assert_eq!("Outer(0000001.23)", format!("{:0>10.2?}", outer));
    assert_eq!("Outer(???1.23???)", format!("{:?^10.2?}", outer));
    assert_eq!("Outer(1.23######)", format!("{:#<10.2?}", outer));
    assert_eq!("Outer(\n    1.23######,\n)", format!("{:#<#10.2?}", outer));
}

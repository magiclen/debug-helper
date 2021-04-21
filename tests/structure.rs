#[macro_use(impl_debug_for_struct)]
extern crate debug_helper;

use std::fmt::{self, Debug, Formatter};

#[test]
#[allow(dead_code)]
fn unit() {
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
            impl_debug_for_struct!(Outer, f, self);
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer", format!("{:?}", outer));
    assert_eq!("Outer", format!("{:#?}", outer));
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

    struct Outer {
        f1: f64,
        f2: Inner,
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_struct!(A, f, self);
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

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

    struct Outer {
        f1: f64,
        f2: Inner,
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_struct!(Outer, f, self, .f1);
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer { f1: 1.23456789 }", format!("{:?}", outer));
    assert_eq!("Outer {\n    f1: 1.23456789,\n}", format!("{:#?}", outer));
    assert_eq!("Outer { f1: 0000001.23 }", format!("{:010.2?}", outer));
    assert_eq!("Outer { f1: 0000001.23 }", format!("{:0>10.2?}", outer));
    assert_eq!("Outer { f1: ???1.23??? }", format!("{:?^10.2?}", outer));
    assert_eq!("Outer { f1: 1.23###### }", format!("{:#<10.2?}", outer));
    assert_eq!("Outer {\n    f1: 1.23######,\n}", format!("{:#<#10.2?}", outer));
}

#[test]
#[allow(dead_code)]
fn one_field_nested() {
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
            impl_debug_for_struct!(Outer, f, self, .f2);
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer { f2: Inner { f1: 5, f2: 10 } }", format!("{:?}", outer));
    assert_eq!(
        "Outer {\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n}",
        format!("{:#?}", outer)
    );
    assert_eq!(
        "Outer { f2: Inner { f1: 0000000005, f2: 0000000010 } }",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "Outer { f2: Inner { f1: 0000000005, f2: 0000000010 } }",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "Outer { f2: Inner { f1: ????5?????, f2: ????10???? } }",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "Outer { f2: Inner { f1: 5#########, f2: 10######## } }",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!(
        "Outer {\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n}",
        format!("{:#<#10.2?}", outer)
    );
}

#[test]
fn all_renamed() {
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
            impl_debug_for_struct!(A, f, self, .f1, .f2);
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("A { f1: 1.23456789, f2: Inner { f1: 5, f2: 10 } }", format!("{:?}", outer));
    assert_eq!(
        "A {\n    f1: 1.23456789,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n}",
        format!("{:#?}", outer)
    );
    assert_eq!(
        "A { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 } }",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "A { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 } }",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "A { f1: ???1.23???, f2: Inner { f1: ????5?????, f2: ????10???? } }",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "A { f1: 1.23######, f2: Inner { f1: 5#########, f2: 10######## } }",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!("A {\n    f1: 1.23######,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n}", format!("{:#<#10.2?}", outer));
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

    struct Outer {
        f1: f64,
        f2: Inner,
        f3: Intruder,
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_struct!(Outer, f, self, (.f1, "number"), .f2, (.f3, "{}", self.f3.s));
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
        f3: Intruder {
            s: "Hi",
        },
    };

    assert_eq!("Outer { f1: number, f2: Inner { f1: 5, f2: 10 }, f3: Hi }", format!("{:?}", outer));
    assert_eq!("Outer {\n    f1: number,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n    f3: Hi,\n}", format!("{:#?}", outer));
    assert_eq!(
        "Outer { f1: number, f2: Inner { f1: 0000000005, f2: 0000000010 }, f3: Hi }",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "Outer { f1: number, f2: Inner { f1: 0000000005, f2: 0000000010 }, f3: Hi }",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "Outer { f1: number, f2: Inner { f1: ????5?????, f2: ????10???? }, f3: Hi }",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "Outer { f1: number, f2: Inner { f1: 5#########, f2: 10######## }, f3: Hi }",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!("Outer {\n    f1: number,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    f3: Hi,\n}", format!("{:#<#10.2?}", outer));
}

#[test]
fn additional_fields() {
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
            impl_debug_for_struct!(Outer, f, self, .f1, .f2, (.f3, "Hi"));
        }
    }

    let outer = Outer {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!(
        "Outer { f1: 1.23456789, f2: Inner { f1: 5, f2: 10 }, f3: Hi }",
        format!("{:?}", outer)
    );
    assert_eq!("Outer {\n    f1: 1.23456789,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n    f3: Hi,\n}", format!("{:#?}", outer));
    assert_eq!(
        "Outer { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 }, f3: Hi }",
        format!("{:010.2?}", outer)
    );
    assert_eq!(
        "Outer { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 }, f3: Hi }",
        format!("{:0>10.2?}", outer)
    );
    assert_eq!(
        "Outer { f1: ???1.23???, f2: Inner { f1: ????5?????, f2: ????10???? }, f3: Hi }",
        format!("{:?^10.2?}", outer)
    );
    assert_eq!(
        "Outer { f1: 1.23######, f2: Inner { f1: 5#########, f2: 10######## }, f3: Hi }",
        format!("{:#<10.2?}", outer)
    );
    assert_eq!("Outer {\n    f1: 1.23######,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    f3: Hi,\n}", format!("{:#<#10.2?}", outer));
}

#[test]
fn fake_struct() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    struct Outer(f64, Inner);

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_struct!(Outer, f, self, let .f1 = self.0);
        }
    }

    let outer = Outer(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });

    assert_eq!("Outer { f1: 1.23456789 }", format!("{:?}", outer));
    assert_eq!("Outer {\n    f1: 1.23456789,\n}", format!("{:#?}", outer));
    assert_eq!("Outer { f1: 0000001.23 }", format!("{:010.2?}", outer));
    assert_eq!("Outer { f1: 0000001.23 }", format!("{:0>10.2?}", outer));
    assert_eq!("Outer { f1: ???1.23??? }", format!("{:?^10.2?}", outer));
    assert_eq!("Outer { f1: 1.23###### }", format!("{:#<10.2?}", outer));
    assert_eq!("Outer {\n    f1: 1.23######,\n}", format!("{:#<#10.2?}", outer));
}

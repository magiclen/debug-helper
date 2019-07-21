#[macro_use(impl_debug_for_enum)]
extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

#[test]
#[allow(dead_code)]
fn unit() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(..)), {F2{..}}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1", format!("{:?}", outer_2));
    assert_eq!("Outer::F1", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2", format!("{:?}", outer_3));
    assert_eq!("Outer::F2", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2", format!("{:#<#10.2?}", outer_3));
}

#[test]
#[allow(dead_code)]
fn unit_renamed() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({A::F0, (F1(..)), {F2{..}}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("A::F0", format!("{:?}", outer_1));
    assert_eq!("A::F0", format!("{:#?}", outer_1));
    assert_eq!("A::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("A::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("A::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("A::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("A::F1", format!("{:?}", outer_2));
    assert_eq!("A::F1", format!("{:#?}", outer_2));
    assert_eq!("A::F1", format!("{:0>10.2?}", outer_2));
    assert_eq!("A::F1", format!("{:?^10.2?}", outer_2));
    assert_eq!("A::F1", format!("{:#<10.2?}", outer_2));
    assert_eq!("A::F1", format!("{:#<#10.2?}", outer_2));

    assert_eq!("A::F2", format!("{:?}", outer_3));
    assert_eq!("A::F2", format!("{:#?}", outer_3));
    assert_eq!("A::F2", format!("{:0>10.2?}", outer_3));
    assert_eq!("A::F2", format!("{:?^10.2?}", outer_3));
    assert_eq!("A::F2", format!("{:#<10.2?}", outer_3));
    assert_eq!("A::F2", format!("{:#<#10.2?}", outer_3));
}

#[test]
#[allow(dead_code)]
fn one_field_primitive() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(f1, ..): (.f1)), {F2{f1, ..}: (.f1)}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1(1.23456789)", format!("{:?}", outer_2));
    assert_eq!("Outer::F1(\n    1.23456789,\n)", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1(0000001.23)", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1(???1.23???)", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1(1.23######)", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1(\n    1.23######,\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2 { f1: 1.23456789 }", format!("{:?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: 1.23456789,\n}", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2 { f1: 0000001.23 }", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: ???1.23??? }", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: 1.23###### }", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: 1.23######,\n}", format!("{:#<#10.2?}", outer_3));
}

#[test]
#[allow(dead_code)]
fn one_field_nested() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(_, f2): (.f2)), {F2{f1: _, f2}: (.f2)}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1(Inner { f1: 5, f2: 10 })", format!("{:?}", outer_2));
    assert_eq!("Outer::F1(\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1(Inner { f1: 0000000005, f2: 0000000010 })", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1(Inner { f1: ????5?????, f2: ????10???? })", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1(Inner { f1: 5#########, f2: 10######## })", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1(\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2 { f2: Inner { f1: 5, f2: 10 } }", format!("{:?}", outer_3));
    assert_eq!("Outer::F2 {\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n}", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2 { f2: Inner { f1: 0000000005, f2: 0000000010 } }", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2 { f2: Inner { f1: ????5?????, f2: ????10???? } }", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2 { f2: Inner { f1: 5#########, f2: 10######## } }", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2 {\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n}", format!("{:#<#10.2?}", outer_3));
}

#[test]
#[allow(dead_code)]
fn all() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(f1, f2): (.f1, .f2)), {F2{f1, f2}: (.f1, .f2)}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1(1.23456789, Inner { f1: 5, f2: 10 })", format!("{:?}", outer_2));
    assert_eq!("Outer::F1(\n    1.23456789,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1(0000001.23, Inner { f1: 0000000005, f2: 0000000010 })", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1(???1.23???, Inner { f1: ????5?????, f2: ????10???? })", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1(1.23######, Inner { f1: 5#########, f2: 10######## })", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1(\n    1.23######,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2 { f1: 1.23456789, f2: Inner { f1: 5, f2: 10 } }", format!("{:?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: 1.23456789,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n}", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2 { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 } }", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: ???1.23???, f2: Inner { f1: ????5?????, f2: ????10???? } }", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: 1.23######, f2: Inner { f1: 5#########, f2: 10######## } }", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: 1.23######,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n}", format!("{:#<#10.2?}", outer_3));
}

#[test]
#[allow(dead_code)]
fn all_renamed() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({A::F0, (F1(f1, f2): (.f1, .f2)), {F2{f1, f2}: (.f1, .f2)}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("A::F0", format!("{:?}", outer_1));
    assert_eq!("A::F0", format!("{:#?}", outer_1));
    assert_eq!("A::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("A::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("A::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("A::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("A::F1(1.23456789, Inner { f1: 5, f2: 10 })", format!("{:?}", outer_2));
    assert_eq!("A::F1(\n    1.23456789,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)", format!("{:#?}", outer_2));
    assert_eq!("A::F1(0000001.23, Inner { f1: 0000000005, f2: 0000000010 })", format!("{:0>10.2?}", outer_2));
    assert_eq!("A::F1(???1.23???, Inner { f1: ????5?????, f2: ????10???? })", format!("{:?^10.2?}", outer_2));
    assert_eq!("A::F1(1.23######, Inner { f1: 5#########, f2: 10######## })", format!("{:#<10.2?}", outer_2));
    assert_eq!("A::F1(\n    1.23######,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("A::F2 { f1: 1.23456789, f2: Inner { f1: 5, f2: 10 } }", format!("{:?}", outer_3));
    assert_eq!("A::F2 {\n    f1: 1.23456789,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n}", format!("{:#?}", outer_3));
    assert_eq!("A::F2 { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 } }", format!("{:0>10.2?}", outer_3));
    assert_eq!("A::F2 { f1: ???1.23???, f2: Inner { f1: ????5?????, f2: ????10???? } }", format!("{:?^10.2?}", outer_3));
    assert_eq!("A::F2 { f1: 1.23######, f2: Inner { f1: 5#########, f2: 10######## } }", format!("{:#<10.2?}", outer_3));
    assert_eq!("A::F2 {\n    f1: 1.23######,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n}", format!("{:#<#10.2?}", outer_3));
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

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
        F3(Intruder),
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(_f1, f2): ((._f1, "number"), .f2)), {F2{f1: _, f2}: ((.f1, "number"), .f2)}, (F3(f3): ((.f3, "{}", f3.s)))}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };
    let outer_4 = Outer::F3(Intruder {
        s: "Hi"
    });

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1(number, Inner { f1: 5, f2: 10 })", format!("{:?}", outer_2));
    assert_eq!("Outer::F1(\n    number,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1(number, Inner { f1: 0000000005, f2: 0000000010 })", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1(number, Inner { f1: ????5?????, f2: ????10???? })", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1(number, Inner { f1: 5#########, f2: 10######## })", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1(\n    number,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2 { f1: number, f2: Inner { f1: 5, f2: 10 } }", format!("{:?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: number,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n}", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2 { f1: number, f2: Inner { f1: 0000000005, f2: 0000000010 } }", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: number, f2: Inner { f1: ????5?????, f2: ????10???? } }", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: number, f2: Inner { f1: 5#########, f2: 10######## } }", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: number,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n}", format!("{:#<#10.2?}", outer_3));

    assert_eq!("Outer::F3(Hi)", format!("{:?}", outer_4));
    assert_eq!("Outer::F3(\n    Hi,\n)", format!("{:#?}", outer_4));
    assert_eq!("Outer::F3(Hi)", format!("{:0>10.2?}", outer_4));
    assert_eq!("Outer::F3(Hi)", format!("{:?^10.2?}", outer_4));
    assert_eq!("Outer::F3(Hi)", format!("{:#<10.2?}", outer_4));
    assert_eq!("Outer::F3(\n    Hi,\n)", format!("{:#<#10.2?}", outer_4));
}

#[test]
#[allow(dead_code)]
fn additional_fields() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(f1, f2): (.f1, .f2)), {F2{f1, f2}: (.f1, .f2, (.f3, "Hi"))}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1(1.23456789, Inner { f1: 5, f2: 10 })", format!("{:?}", outer_2));
    assert_eq!("Outer::F1(\n    1.23456789,\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n)", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1(0000001.23, Inner { f1: 0000000005, f2: 0000000010 })", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1(???1.23???, Inner { f1: ????5?????, f2: ????10???? })", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1(1.23######, Inner { f1: 5#########, f2: 10######## })", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1(\n    1.23######,\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2 { f1: 1.23456789, f2: Inner { f1: 5, f2: 10 }, f3: Hi }", format!("{:?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: 1.23456789,\n    f2: Inner {\n        f1: 5,\n        f2: 10,\n    },\n    f3: Hi,\n}", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2 { f1: 0000001.23, f2: Inner { f1: 0000000005, f2: 0000000010 }, f3: Hi }", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: ???1.23???, f2: Inner { f1: ????5?????, f2: ????10???? }, f3: Hi }", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: 1.23######, f2: Inner { f1: 5#########, f2: 10######## }, f3: Hi }", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: 1.23######,\n    f2: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    f3: Hi,\n}", format!("{:#<#10.2?}", outer_3));
}

#[test]
#[allow(dead_code)]
fn reset() {
    #[derive(Debug)]
    struct Inner {
        f1: u8,
        f2: u8,
    }

    enum Outer {
        F0,
        F1(f64, Inner),
        F2 {
            f1: f64,
            f2: Inner,
        },
    }

    impl Debug for Outer {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            impl_debug_for_enum!({Outer::F0, (F1(f1, f2): (let .f1 = f2, let .f2 = f1)), {F2{f1, f2}: (let .f1 = f2, let .f2 = f1)}}, f, self);
        }
    }

    let outer_1 = Outer::F0;
    let outer_2 = Outer::F1(1.23456789, Inner {
        f1: 5,
        f2: 10,
    });
    let outer_3 = Outer::F2 {
        f1: 1.23456789,
        f2: Inner {
            f1: 5,
            f2: 10,
        },
    };

    assert_eq!("Outer::F0", format!("{:?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#?}", outer_1));
    assert_eq!("Outer::F0", format!("{:0>10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:?^10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<10.2?}", outer_1));
    assert_eq!("Outer::F0", format!("{:#<#10.2?}", outer_1));

    assert_eq!("Outer::F1(Inner { f1: 5, f2: 10 }, 1.23456789)", format!("{:?}", outer_2));
    assert_eq!("Outer::F1(\n    Inner {\n        f1: 5,\n        f2: 10,\n    },\n    1.23456789,\n)", format!("{:#?}", outer_2));
    assert_eq!("Outer::F1(Inner { f1: 0000000005, f2: 0000000010 }, 0000001.23)", format!("{:0>10.2?}", outer_2));
    assert_eq!("Outer::F1(Inner { f1: ????5?????, f2: ????10???? }, ???1.23???)", format!("{:?^10.2?}", outer_2));
    assert_eq!("Outer::F1(Inner { f1: 5#########, f2: 10######## }, 1.23######)", format!("{:#<10.2?}", outer_2));
    assert_eq!("Outer::F1(\n    Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    1.23######,\n)", format!("{:#<#10.2?}", outer_2));

    assert_eq!("Outer::F2 { f1: Inner { f1: 5, f2: 10 }, f2: 1.23456789 }", format!("{:?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: Inner {\n        f1: 5,\n        f2: 10,\n    },\n    f2: 1.23456789,\n}", format!("{:#?}", outer_3));
    assert_eq!("Outer::F2 { f1: Inner { f1: 0000000005, f2: 0000000010 }, f2: 0000001.23 }", format!("{:0>10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: Inner { f1: ????5?????, f2: ????10???? }, f2: ???1.23??? }", format!("{:?^10.2?}", outer_3));
    assert_eq!("Outer::F2 { f1: Inner { f1: 5#########, f2: 10######## }, f2: 1.23###### }", format!("{:#<10.2?}", outer_3));
    assert_eq!("Outer::F2 {\n    f1: Inner {\n        f1: 5#########,\n        f2: 10########,\n    },\n    f2: 1.23######,\n}", format!("{:#<#10.2?}", outer_3));
}
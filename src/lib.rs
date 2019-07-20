/*!
# Debug Helper

This crate provides declarative macros to help you implement the `Debug` trait manually.

## Examples

For structs,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A {
    pub f1: u8,
    pub f2: i16,
    pub f3: f64,
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_struct!(A, f, self, .f1, (.f3, "{:.3}", self.f3));
    }
}

let a = A {
    f1: 1,
    f2: 2,
    f3: std::f64::consts::PI,
};

println!("{:#?}", a);

/*
    A {
        f1: 1,
        f3: 3.142,
    }
*/
```

For tuple structs,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub struct A(pub u8, pub i16, pub f64);

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_tuple_struct!(A, f, self, .0, (.2, "{:.3}", self.2));
    }
}

let a = A(1, 2, std::f64::consts::PI);

println!("{:#?}", a);

/*
    A(
        1,
        3.142,
    )
*/
```

For enums,

```rust
#[macro_use] extern crate debug_helper;

use std::fmt::{self, Formatter, Debug};

pub enum A {
    V1,
    V2(u8, i16, f64),
    V3 {
        f1: u8,
        f2: i16,
        f3: f64,
    },
}

impl Debug for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        impl_debug_for_enum!(A::{V1, (V2(f1, _, f3): (.f1, (.f3, "{:.3}", f3))), {V3{f1, f2: _, f3}: (.f1, (.f3, "{:.3}", f3))}}, f, self);
    }
}

let a = A::V1;
let b = A::V2(1, 2, std::f64::consts::PI);
let c = A::V3{
    f1: 1,
    f2: 2,
    f3: std::f64::consts::PI,
};

println!("{:#?}", a);
println!("{:#?}", b);
println!("{:#?}", c);

/*
    V1
    V2(
        1,
        3.142,
    )
    V3 {
        f1: 1,
        f3: 3.142,
    }
*/
```

In most cases, you can use the [`derivative`](https://crates.io/crates/derivative) crate to implement the `Debug` trait.
*/

use std::fmt::{self, Formatter, Alignment, Debug};

#[doc(hidden)]
pub fn pad(t: &impl Debug, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    if f.alternate() {
        let s = if let Some(width) = f.width() {
            // TODO Not to use replace.

            let s = if let Some(precision) = f.precision() {
                match f.align().unwrap_or(Alignment::Left) {
                    Alignment::Left => {
                        format!("{:\x7F<#width$.precision$?}", t, width = width, precision = precision)
                    }
                    Alignment::Right => {
                        format!("{:\x7F>#width$.precision$?}", t, width = width, precision = precision)
                    }
                    Alignment::Center => {
                        format!("{:\x7F^#width$.precision$?}", t, width = width, precision = precision)
                    }
                }
            } else {
                match f.align().unwrap_or(Alignment::Left) {
                    Alignment::Left => {
                        format!("{:\x7F<#width$?}", t, width = width)
                    }
                    Alignment::Right => {
                        format!("{:\x7F>#width$?}", t, width = width)
                    }
                    Alignment::Center => {
                        format!("{:\x7F^#width$?}", t, width = width)
                    }
                }
            };

            s.replace("\x7F", &f.fill().to_string())
        } else {
            if let Some(precision) = f.precision() {
                format!("{:#.precision$?}", t, precision = precision)
            } else {
                format!("{:#?}", t)
            }
        };

        f.write_str(&s.replace("\n", "\n    "))
    } else {
        Debug::fmt(t, f)
    }
}

#[macro_export]
macro_rules! impl_debug_for_struct {
    ($struct_name:ident, $formatter:expr $(, $self:expr)? $(,)*) => {
        return $formatter.write_str(stringify!($struct_name));
    };
    ($struct_name:ident, $formatter:expr, $self:expr, $(.$first_field:ident)? $((.$first_field_2:ident, $($first_field_2_fmt:tt)+))? $(, $(.$field:ident)? $((.$field_2:ident, $($field_2_fmt:tt)+))?)* $(,)*) => {
        {
            use std::fmt::Write;

            $formatter.write_str(stringify!($struct_name))?;

            let separator = if $formatter.alternate() {
                $formatter.write_str(" {\n    ")?;

                "\n    "
            } else {
                $formatter.write_str(" { ")?;

                ", "
            };

            $(
                $formatter.write_str(stringify!($first_field))?;
                $formatter.write_str(": ")?;

                $crate::pad(&$self.$first_field, $formatter)?;

                if $formatter.alternate() {
                    $formatter.write_char(',')?;
                }
            )?

            $(
                $formatter.write_str(stringify!($first_field_2))?;
                $formatter.write_str(": ")?;

                if $formatter.alternate() {
                    $formatter.write_str(&format!($($first_field_2_fmt)*).replace("\n", "\n    "))?;

                    $formatter.write_char(',')?;
                } else {
                    $formatter.write_fmt(format_args!($($first_field_2_fmt)*))?;
                }
            )?

            $(
                $(
                    $formatter.write_str(separator)?;
                    $formatter.write_str(stringify!($field))?;
                    $formatter.write_str(": ")?;

                    $crate::pad(&$self.$field, $formatter)?;

                    if $formatter.alternate() {
                        $formatter.write_char(',')?;
                    }
                )?

                $(
                    $formatter.write_str(separator)?;
                    $formatter.write_str(stringify!($field_2))?;
                    $formatter.write_str(": ")?;

                    if $formatter.alternate() {
                        $formatter.write_str(&format!($($field_2_fmt)*).replace("\n", "\n    "))?;

                        $formatter.write_char(',')?;
                    } else {
                        $formatter.write_fmt(format_args!($($field_2_fmt)*))?;
                    }
                )?
            )*

            if $formatter.alternate() {
                return $formatter.write_str("\n}");
            } else {
                return $formatter.write_str(" }");
            }
        }
    };
}

#[macro_export]
macro_rules! impl_debug_for_tuple_struct {
    ($struct_name:ident, $formatter:expr $(, $self:expr)? $(,)*) => {
        return $formatter.write_str(stringify!($struct_name));
    };
    ($struct_name:ident, $formatter:expr, $self:expr, $(.$first_field:tt)? $((.$first_field_2:tt, $($first_field_2_fmt:tt)+))? $(, $(.$field:tt)? $((.$field_2:tt, $($field_2_fmt:tt)+))?)* $(,)*) => {
        {
            use std::fmt::Write;

            $formatter.write_str(stringify!($struct_name))?;

            let separator = if $formatter.alternate() {
                $formatter.write_str("(\n    ")?;

                "\n    "
            } else {
                $formatter.write_str("(")?;

                ", "
            };

            $(
                $crate::pad(&$self.$first_field, $formatter)?;

                if $formatter.alternate() {
                    $formatter.write_char(',')?;
                }
            )?

            $(
                if $formatter.alternate() {
                    $formatter.write_str(&format!($($first_field_2_fmt)*).replace("\n", "\n    "))?;

                    $formatter.write_char(',')?;
                } else {
                    $formatter.write_fmt(format_args!($($first_field_2_fmt)*))?;
                }
            )?

            $(
                $(
                    $formatter.write_str(separator)?;
                    $crate::pad(&$self.$field, $formatter)?;

                    if $formatter.alternate() {
                        $formatter.write_char(',')?;
                    }
                )?

                $(
                    $formatter.write_str(separator)?;

                    if $formatter.alternate() {
                        $formatter.write_str(&format!($($field_2_fmt)*).replace("\n", "\n    "))?;

                        $formatter.write_char(',')?;
                    } else {
                        $formatter.write_fmt(format_args!($($field_2_fmt)*))?;
                    }
                )?
            )*

            if $formatter.alternate() {
                return $formatter.write_str("\n)");
            } else {
                return $formatter.write_str(")");
            }
        }
    }
}

#[macro_export]
macro_rules! impl_debug_for_enum {
    ($enum_name:ident::{$( $($variant_unit:ident)? $(($variant_tuple:ident ($($tuple:tt)*) $(:($(.$t_first_field:tt)? $((.$t_first_field_2:tt, $($t_first_field_2_fmt:tt)+))? $(, $(.$t_field:tt)? $((.$t_field_2:tt, $($t_field_2_fmt:tt)+))?)* $(,)*))? ) )? $({$variant_struct:ident {$($struct:tt)*} $(:($(.$s_first_field:tt)? $((.$s_first_field_2:tt, $($s_first_field_2_fmt:tt)+))? $(, $(.$s_field:tt)? $((.$s_field_2:tt, $($s_field_2_fmt:tt)+))?)* $(,)*))? })? ),+ $(,)*}, $formatter:expr, $self:expr $(,)*) => {
        {
            use std::fmt::Write;

            match $self {
                $(
                    $(
                        Self::$variant_unit => {
                            return $formatter.write_str(stringify!($variant_unit));
                        }
                    )?
                    $(
                        Self::$variant_tuple ($($tuple)*)=> {
                            let mut result = $formatter.write_str(stringify!($variant_tuple));

                            $(
                                let separator = if $formatter.alternate() {
                                    $formatter.write_str("(\n    ")?;

                                    "\n    "
                                } else {
                                    $formatter.write_str("(")?;

                                    ", "
                                };

                                $(
                                    $crate::pad(&$t_first_field, $formatter)?;

                                    if $formatter.alternate() {
                                        $formatter.write_char(',')?;
                                    }
                                )?

                                $(
                                    if $formatter.alternate() {
                                        $formatter.write_str(&format!($($t_first_field_2_fmt)*).replace("\n", "\n    "))?;

                                        $formatter.write_char(',')?;
                                    } else {
                                        $formatter.write_fmt(format_args!($($t_first_field_2_fmt)*))?;
                                    }
                                )?

                                $(
                                    $(
                                        $formatter.write_str(separator)?;
                                        $crate::pad(&$t_field, $formatter)?;

                                        if $formatter.alternate() {
                                            $formatter.write_char(',')?;
                                        }
                                    )?

                                    $(
                                        $formatter.write_str(separator)?;

                                        if $formatter.alternate() {
                                            $formatter.write_str(&format!($($t_field_2_fmt)*).replace("\n", "\n    "))?;

                                            $formatter.write_char(',')?;
                                        } else {
                                            $formatter.write_fmt(format_args!($($t_field_2_fmt)*))?;
                                        }
                                    )?
                                )*

                                result = if $formatter.alternate() {
                                    $formatter.write_str("\n)")
                                } else {
                                    $formatter.write_str(")")
                                };
                            )?

                            return result;
                        }
                    )?
                    $(
                        Self::$variant_struct {$($struct)*}=> {
                            let mut result = $formatter.write_str(stringify!($variant_struct));

                            $(
                                let separator = if $formatter.alternate() {
                                    $formatter.write_str(" {\n    ")?;

                                    "\n    "
                                } else {
                                    $formatter.write_str(" { ")?;

                                    ", "
                                };

                                $(
                                    $formatter.write_str(stringify!($s_first_field))?;
                                    $formatter.write_str(": ")?;

                                    $crate::pad(&$s_first_field, $formatter)?;

                                    if $formatter.alternate() {
                                        $formatter.write_char(',')?;
                                    }
                                )?

                                $(
                                    $formatter.write_str(stringify!($s_first_field_2))?;
                                    $formatter.write_str(": ")?;

                                    if $formatter.alternate() {
                                        $formatter.write_str(&format!($($s_first_field_2_fmt)*).replace("\n", "\n    "))?;

                                        $formatter.write_char(',')?;
                                    } else {
                                        $formatter.write_fmt(format_args!($($s_first_field_2_fmt)*))?;
                                    }
                                )?

                                $(
                                    $(
                                        $formatter.write_str(separator)?;
                                        $formatter.write_str(stringify!($s_field))?;
                                        $formatter.write_str(": ")?;

                                        $crate::pad(&$s_field, $formatter)?;

                                        if $formatter.alternate() {
                                            $formatter.write_char(',')?;
                                        }
                                    )?

                                    $(
                                        $formatter.write_str(separator)?;
                                        $formatter.write_str(stringify!($s_field_2))?;
                                        $formatter.write_str(": ")?;

                                        if $formatter.alternate() {
                                            $formatter.write_str(&format!($($s_field_2_fmt)*).replace("\n", "\n    "))?;

                                            $formatter.write_char(',')?;
                                        } else {
                                            $formatter.write_fmt(format_args!($($s_field_2_fmt)*))?;
                                        }
                                    )?
                                )*

                                result = if $formatter.alternate() {
                                    $formatter.write_str("\n}")
                                } else {
                                    $formatter.write_str(" }")
                                };
                            )?

                            return result;
                        }
                    )?
                )+
            }
        }
    };
}
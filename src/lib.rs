#![no_std]
#![recursion_limit = "128"]

#[macro_export]
macro_rules! from {
    // Base case: no arguments
    () => {};

    // Case with function: process one pair and recursively call the macro with the rest
    (
        $from:ty => $to:ty: $func:ident,
        $($rest:tt)*
    ) => {
        impl core::convert::From<$from> for $to {
            fn from(value: $from) -> $to {
               < $to >::$func(value)
            }
        }

        from!($($rest)*);
    };

    // Case without function: process one pair and recursively call the macro with the rest
    (
        $from:ty => $to:ident,
        $($rest:tt)*
    ) => {
        impl core::convert::From<$from> for $to {
            fn from(value: $from) -> $to {
                $to(value)
            }
        }

        from!($($rest)*);
    };
}

#[cfg(test)]
mod test_from {
    use super::*;

    #[test]
    fn test_from_macro() {
        #[derive(Debug, PartialEq)]
        struct A(i32);

        #[derive(Debug, PartialEq)]
        struct B(i32);

        impl B {
            fn from_u64(value: u64) -> B {
                B(value as i32)
            }
        }

        from! {
            i32 => A,
            u64 => B: from_u64,
        }
    }
}

#[macro_export]
macro_rules! into {
    // Base case: no arguments
    () => {};


    // Case without function: process one pair and recursively call the macro with the rest
    (
        $from:ty => $to:ident,
        $($rest:tt)*
    ) => {
        impl core::convert::Into<$to> for $from {
            fn into(self) -> $to {
                $to(self)
            }
        }

        into!($($rest)*);
    };

    // Case with function: process one pair and recursively call the macro with the rest
    (
        $from:ty => $to:ty: $func:ident,
        $($rest:tt)*
    ) => {
        impl core::convert::Into<$to> for $from {
            fn into(self) -> $to {
               < $to >::$func(self)
            }
        }

        into!($($rest)*);
    };
}
#[cfg(test)]
mod test_into {
    #[test]
    fn test_into_macro() {
        #[derive(Debug, PartialEq)]
        struct A(i32);

        #[derive(Debug, PartialEq)]
        struct B(i32);

        struct C(B);

        impl B {
            fn from_u64(value: u64) -> B {
                B(value as i32)
            }
        }

        into! {
            i32 => A,
            u64 => B: from_u64,
            B => C,
        }
    }
}

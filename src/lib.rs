#[macro_export]
macro_rules! from {
    // Base case: no arguments
    () => {};

    // Case with function: process one pair and recursively call the macro with the rest
    (
        $from:ty => $to:ty: $func:ident,
        $($rest:tt)*
    ) => {
        impl From<$from> for $to {
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
        impl From<$from> for $to {
            fn from(value: $from) -> $to {
                let v = value as i32;
                $to(v)
            }
        }

        from!($($rest)*);
    };
}

fn main() {
    #[derive(Debug, PartialEq)]
    struct A(i32);
    #[derive(Debug, PartialEq)]
    struct B(i32);

    let value = 3;

    from! {
            i32 => A,
        }
}

#[cfg(test)]
mod test {
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

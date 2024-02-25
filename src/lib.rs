#![no_std]
#![recursion_limit = "128"]

//! # Usefull macros
//! A collection of useful macros for repetitive task in  Rust.



/// # `from!`
///
/// The `from!` macro is used to generate [From] trait implementations for specified types.
///
/// ## Usage
///
/// ```no_run
/// from! {
///     i32 => A,
///     u64 => B: from_u64,
/// }
/// ```
///
/// ## Details
///
/// - The macro accepts pairs of types separated by `=>`. 
/// - Each pair can optionally be followed by a `:` and a function identifier. If provided, this function will be used for the conversion. If not, the constructor of the target type will be used.
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




/// # `into!` 
///
/// The `into!` macro is used to generate `Into` trait implementations for specified types.
///
/// ## Usage
///
/// ```no_run
/// into! {
///     i32 => A,
///     u64 => B: from_u64,
///     B => C,
/// }
/// ```
///
/// ## Details
///
/// - The macro accepts pairs of types separated by `=>`. 
/// - Each pair can optionally be followed by a `:` and a function identifier. If provided, this function will be used for the conversion. If not, the constructor of the target type will be used.
#[macro_export]macro_rules! into {
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


/// # `deref!` 
///
/// The `deref!` macro is used to generate `Deref` and `DerefMut` trait implementations for specified types.
///
/// ## Usage
///
/// ```
/// deref! {
///     A => i32,
///     B => mut i32,
/// }
/// ```
///
/// ## Details
///
/// - The macro accepts pairs of types separated by `=>`. 
/// - If the target type is preceded by `mut`, the macro generates both `Deref` and `DerefMut` implementations. If not, it only generates a `Deref` implementation.
#[macro_export]macro_rules! deref {
    () => {

    };

    (
        $from:ty => $to:ty,
        $($rest:tt)*
    ) => {
        impl core::ops::Deref for $from {
            type Target = $to;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        deref!($($rest)*);
    };

    {
        $from:ty => mut $to:ty,
        $($rest:tt)*
    } => {
        deref!($from => $to);

        impl core::ops::DerefMut for $from {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        deref!($($rest)*);
    };
}

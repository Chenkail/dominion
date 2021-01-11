// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;

// #[proc_macro_attribute]
// pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let output = quote! {
//         fn macro_test() {
//             // This is a macro
//         }
//     };

//     output.into()
// }


/// Macro for defining a new card
#[macro_export]
macro_rules! card {
    ($struct:ident, $name:expr, $cost:expr) => {
        pub struct $struct;
        impl Card for $struct {
            fn cost(&self) -> i32 {
                return $cost as i32;
            }
            
            fn name(&self) -> &'static str {
                return $name;
            }
        }
    };
}

#[macro_export]
macro_rules! treasure {
    ($struct:ident, $value:expr) => {
        impl Treasure for $struct {
            fn value(&self) -> i32 {
                return $value;
            }
        }
    };
}

#[macro_export]
macro_rules! victory {
    ($struct:ident, $points:expr) => {
        impl Victory for $struct {
            fn points(&self) -> i32 {
                return $points;
            }
        }
    };
}

#[macro_export]
macro_rules! curse {
    ($struct:ident, $points:expr) => {
        impl CurseTrait for $struct {
            fn points(&self) -> i32 {
                return $points;
            }
        }
    };
}

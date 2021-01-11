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
    ($struct_name:ident, $name:expr, $cost:expr) => {
        pub struct $struct_name;
        impl Card for $struct_name {
            fn cost(&self) -> i32 {
                return $cost as i32;
            }
            
            fn name(&self) -> &'static str {
                return $name;
            }
        }
    };
}

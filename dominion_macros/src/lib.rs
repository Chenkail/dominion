extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn card(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
}

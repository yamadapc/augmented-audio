use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ffi_export(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

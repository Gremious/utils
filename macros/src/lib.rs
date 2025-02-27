// use quote::{ToTokens, quote};

#[proc_macro_attribute]
pub fn yoy(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	item
}

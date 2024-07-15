use proc_macro::TokenStream;
mod node;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    node::define_nodes(_attr, item)
}

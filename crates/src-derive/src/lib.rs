use proc_macro::TokenStream;
mod node;
mod walker;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(node::define_nodes(_attr, item))
}

#[proc_macro_attribute]
pub fn walker(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(walker::generate_walker_impl(item))
}

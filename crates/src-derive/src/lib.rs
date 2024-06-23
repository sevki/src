use proc_macro::TokenStream;
mod visitor;

#[proc_macro_attribute]
pub fn visitor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(visitor::generate_visitor_trait(item))
}


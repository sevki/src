use proc_macro::TokenStream;
mod node;

/// Defines a node.
/// ```rust,ignore
/// #[node]
/// struct MyNode {
///    field: Type,
/// }
/// ```
/// which will expand to:
/// ```rust,ignore
/// struct MyNode {
///   field: Spanned<Type>,
/// }
/// ```
#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    node::define_nodes(_attr, item)
}

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_attribute]
pub fn visitor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;

    let visitor_trait_name = syn::Ident::new(&format!("{}Visitor", name), name.span());

    let mut visitor_methods = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        for (index, field) in data_struct.fields.iter().enumerate() {
            if let Some(ident) = &field.ident {
                if let syn::Type::Path(type_path) = &field.ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident == "Spanned" {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                                    let method_name = syn::Ident::new(&format!("visit_{}_{}", ident, index), name.span());
                                    visitor_methods.push(quote! {
                                        fn #method_name(&mut self, value: #inner_type, span: &Span);
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        pub trait #visitor_trait_name {
            #(#visitor_methods)*
        }

        #input
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor() {
        let input = quote! {
            #[visitor]
            pub struct Test {
                field1: Spanned<Ident>,
                field2: Option<Spanned<Ident>>,
                field3: Box<Spanned<Ident>>,
                field4: Vec<Spanned<Ident>>,
                field5: Block<Spanned<Ident>>,
            }
        };

        let expected = quote! {
            pub trait TestVisitor {
                fn visit_ident_field(&mut self, value: Ident, span: &Span);
                fn visit_ident_field2(&mut self, value: Option<Ident>, span: &Span);
                fn visit_ident_field3(&mut self, value: Ident, span: &Span);
                fn visit_ident_field4(&mut self, value: Vec<Ident>, span: &Span);
                fn visit_ident_field5(&mut self, value: Block<Ident>, span: &Span);
            }

            pub struct Test {
                field1: Spanned<Ident>,
                field2: Option<Spanned<Ident>>,
                field3: Box<Spanned<Ident>>,
                field4: Vec<Spanned<Ident>>,
                field5: Block<Spanned<Ident>>,
            }
        };

        let actual = visitor(TokenStream::new(), input).to_string();
        assert_eq!(expected.to_string(), actual);
    }
}
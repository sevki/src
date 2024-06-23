use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};


pub fn generate_visitor_trait(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = proc_macro::TokenStream::from(stream);
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident;

    let visitor_trait_name = format_ident!("{}Visitor", struct_name);

    let field_types: Vec<syn::Type> = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields.named.into_iter().map(|field| field.ty).collect(),
            Fields::Unnamed(fields) => fields.unnamed.into_iter().map(|field| field.ty).collect(),
            Fields::Unit => vec![],
        },
        _ => panic!("visitor macro only supports structs"),
    };

    let field_visitor_traits = field_types.iter().map(|ty| {
        let ty_str = quote!(#ty).to_string();
        let visitor_trait_name = format_ident!("{}Visitor", ty_str);
        quote! { + #visitor_trait_name }
    });

    let visitor_methods = field_types.iter().map(|ty| {
        let ty_str = quote!(#ty).to_string();
        let method_name = format_ident!("visit_{}", ty_str);
        quote! {
            fn #method_name(self, node: #ty, span: Range<Location>);
        }
    });

    let expanded = quote! {
        trait #visitor_trait_name: NodeVisitor #(#field_visitor_traits)* {
            #(#visitor_methods)*
        }
    };

    proc_macro::TokenStream::from(expanded)
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

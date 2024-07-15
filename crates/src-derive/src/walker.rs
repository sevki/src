use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, TypePath,
};

pub fn generate_walker_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let type_name = &input.ident;
    let walker_name = format_ident!("{}Walker", type_name);
    let visitor_name = format_ident!("{}Visitor", type_name);

    let walk_impl = generate_walk_impl(type_name, &input.data);

    let expanded = quote! {
        #input

        pub struct #walker_name;

        impl #walker_name {
            pub fn walk<V: #visitor_name>(node: &Spanned<#type_name>, visitor: &mut V) {
                #walk_impl
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_walk_impl(type_name: &Ident, data: &Data) -> TokenStream {
    match data {
        Data::Struct(data_struct) => generate_struct_walk_impl(data_struct),
        Data::Enum(data_enum) => generate_enum_walk_impl(type_name, data_enum),
        Data::Union(_) => panic!("Unions are not supported"),
    }
}
fn generate_field_visit(ty: &Type, field_access: TokenStream) -> TokenStream {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            if let Some(segment) = path.segments.last() {
                match segment.ident.to_string().as_str() {
                    "Spanned" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                let visit_method = format_ident!(
                                    "visit_{}",
                                    inner_type.to_token_stream().to_string().to_lowercase()
                                );
                                return quote! {
                                    visitor.#visit_method(&(#field_access).1, (#field_access).span());
                                };
                            }
                        }
                    }
                    "Box" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                let inner_visit =
                                    generate_field_visit(inner_type, quote! { (*#field_access) });
                                return quote! {
                                    #inner_visit
                                };
                            }
                        }
                    }
                    "Option" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                let inner_visit =
                                    generate_field_visit(inner_type, quote! { inner });
                                return quote! {
                                    if let Some(inner) = #field_access.as_ref() {
                                        #inner_visit
                                    }
                                };
                            }
                        }
                    }
                    "Vec" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                let inner_visit = generate_field_visit(inner_type, quote! { item });
                                return quote! {
                                    for item in #field_access.iter() {
                                        #inner_visit
                                    }
                                };
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    quote! {}
}

fn generate_struct_walk_impl(data_struct: &syn::DataStruct) -> TokenStream {
    let field_visits = data_struct
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            generate_field_visit(&field.ty, quote!(node.1.#field_name))
        })
        .collect::<Vec<_>>();

    quote! {
        #(#field_visits)*
    }
}

fn generate_enum_walk_impl(enum_name: &Ident, data_enum: &syn::DataEnum) -> TokenStream {
    let variant_matches = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field_visit = generate_field_visit(&fields.unnamed[0].ty, quote!(value));
                    quote! {
                        #enum_name::#variant_name(value) => {
                            #field_visit
                        }
                    }
                }
                _ => quote! {
                    #enum_name::#variant_name { .. } => {}
                },
            }
        })
        .collect::<Vec<_>>();

    quote! {
        match &node.1 {
            #(#variant_matches)*
        }
    }
}

fn is_spanned_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Spanned";
        }
    }
    false
}

fn is_box_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Box";
        }
    }
    false
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

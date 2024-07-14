use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type,
    TypePath,
};

pub fn define_nodes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => {
            return quote! {
                compile_error!("define_nodes can only be used on structs");
            }
            .into()
        }
    };

    let spanned_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        match field_type {
            Type::Path(path) => {
                let expanded_type = wrap_path_in_spanned(path);
                quote! {
                    #field_name: #expanded_type,
                }
            }
            _ => {
                panic!(
                    "Only named fields are supported which {} is not",
                    field_type.into_token_stream()
                );
            }
        }
    });

    let expanded_impl = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_span_getter = format_ident!("{}_span", field_name.as_ref().unwrap());
        let field_node_getter = format_ident!("{}_node", field_name.as_ref().unwrap());
        let field_type = &field.ty;
        match field_type {
            Type::Path(path) => {
                let expanded_type = wrap_path_in_spanned(path);
                let expanded_range_type = wrap_range_location(path);
                quote! {
                    fn #field_node_getter(&self) -> &#expanded_type {
                        todo!()
                    }
                    fn #field_span_getter(&self) -> &#expanded_range_type {
                                            todo!()
                    }
                }
            }
            _ => {
                panic!(
                    "Only named fields are supported which {} is not",
                    field_type.into_token_stream()
                );
            }
        }
    });

    let visitor_name = format_ident!("{}Visitor", struct_name);
    let visitor_trait_stub = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_visit = format_ident!("visit_{}", field_name.as_ref().unwrap());
        let field_type = &field.ty;
        match field_type {
            Type::Path(path) => {
                let unwrapped_type = unwrap_path(path);
                quote! {
                    fn #field_visit(&self, node: &#unwrapped_type, span: &Range<Location>) -> ops::traversal::Result;
                }
            }
            _ => {
                panic!(
                    "Only named fields are supported which {} is not",
                    field_type.into_token_stream()
                );
            }
        }
    });

    let accept_impl = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        match field_type {
            Type::Path(path) => {
                let visit_fn = format_ident!("visit_{}", field_name.as_ref().unwrap());
                match path.path.segments.last() {
                    Some(syn::PathSegment { ident, arguments }) => match arguments {
                        PathArguments::None => {
                            quote! {
                                if let cont = visitor.#visit_fn(
                                    &self.#field_name.1,
                                    &(self.#field_name.0..self.#field_name.2)
                                ) {
                                    return;
                                }
                            }
                        }
                        PathArguments::AngleBracketed(args) => match args.args.first() {
                            Some(GenericArgument::Type(_)) => match ident.to_string().as_str() {
                                "Option" => {
                                    quote! {
                                        if let Some(inner) = &self.#field_name {
                                            if let cont = visitor.#visit_fn(
                                                &inner.1,
                                                &(inner.0..inner.2)
                                            ) {
                                                return;
                                            }
                                        }
                                    }
                                }
                                "Vec" => {
                                    quote! {
                                        for inner in self.#field_name.iter() {
                                            if let cont = visitor.#visit_fn(
                                                &inner.1,
                                                &(inner.0..inner.2)
                                            ) {
                                                return;
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    quote! {
                                        if let cont =visitor.#visit_fn(
                                            &self.#field_name.1,
                                            &(self.#field_name.0..self.#field_name.2)
                                        ) {
                                            return;
                                        }
                                    }
                                }
                            },
                            _ => {
                                quote! {
                                    if let cont = visitor.#visit_fn(
                                        self.1,
                                        self.0..self.2
                                    ) {
                                        return;
                                    }
                                }
                            }
                        },
                        PathArguments::Parenthesized(_) => todo!(),
                    },
                    None => {
                        quote! {
                            compile_error!("No path segments found");
                        }
                    }
                }
            }
            _ => {
                panic!(
                    "Only named fields are supported which {} is not",
                    field_type.into_token_stream()
                );
            }
        }
    });

    let field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name
        }
    });
    let not_spanned_fileds = spanned_fields.clone();
    let field_types = fields.iter().map(|field| {
        let field_type = &field.ty;
        match field_type {
            Type::Path(path) => {
                let expanded_type = wrap_path_in_spanned(path);
                quote! {
                    #expanded_type
                }
            }
            _ => {
                panic!(
                    "Only named fields are supported which {} is not",
                    field_type.into_token_stream()
                );
            }
        }
    });
    let field_types_clone = field_types.clone();
    let struct_name_lower = format_ident!("{}", struct_name.to_string().to_lowercase());
    let field_ids = fields.iter().enumerate().map(|field| {
        let field_name = syn::Index::from(field.0);
        quote! {
            #struct_name_lower.#field_name
        }
    });
    let vis = &input.vis;
    let expanded = quote! {
        #[derive(Debug)]
        #vis struct #struct_name {
            #(#spanned_fields)*
        }
        #vis trait #visitor_name {
            #(#visitor_trait_stub)*
        }
        impl From<
            (#(#field_types),*)
        > for #struct_name {
            fn from(
                #struct_name_lower: (
                            #(#field_types_clone),*
                        )
            ) -> Self {
                Self::new(
                    #(#field_ids),*
                )
            }
        }

        impl #struct_name {
            fn new(#(#not_spanned_fileds)*) -> Self {
                Self {
                    #(#field_names,)*
                }
            }

        }
        impl #struct_name {
            fn accept(&self, visitor: &impl #visitor_name) {
                #(#accept_impl)*
            }
        }
    };

    expanded.into()
}
fn wrap_range_location(path: &TypePath) -> impl ToTokens {
    match path.path.segments.last() {
        Some(syn::PathSegment { ident, arguments }) => {
            if let syn::PathArguments::AngleBracketed(args) = arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    quote! {
                        #ident<Range<Location>>
                    }
                } else {
                    quote! {
                        Range<Location>
                    }
                }
            } else {
                quote! {
                    Range<Location>
                }
            }
        }
        _ => {
            quote! {
                Range<Location>
            }
        }
    }
}

fn wrap_path_in_spanned(path: &TypePath) -> impl ToTokens {
    match path.path.segments.last() {
        Some(syn::PathSegment { ident, arguments }) => {
            if let syn::PathArguments::AngleBracketed(args) = arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    quote! {
                        #ident<Spanned<#inner_ty>>
                    }
                } else {
                    quote! {
                        Spanned<#ident>
                    }
                }
            } else {
                quote! {
                    Spanned<#ident>
                }
            }
        }
        _ => {
            quote! {
                Spanned<#path>
            }
        }
    }
}

fn unwrap_path(path: &TypePath) -> impl ToTokens {
    match path.path.segments.last() {
        Some(syn::PathSegment { ident, arguments }) => {
            if let syn::PathArguments::AngleBracketed(args) = arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    quote! {
                        #inner_ty
                    }
                } else {
                    quote! {
                        #ident
                    }
                }
            } else {
                quote! {
                    #ident
                }
            }
        }
        _ => {
            quote! {
                #path
            }
        }
    }
}

fn wrap_type_in_spanned(ty: &Type, field_name: &Option<Ident>) -> impl ToTokens {
    match (ty, field_name) {
        (Type::Path(path), Some(field_name)) => {
            let ty = wrap_path_in_spanned(path);
            quote! {
                #field_name: #ty,
            }
        }
        (Type::Path(path), None) => {
            let ty = wrap_path_in_spanned(path);
            quote! {
                #ty,
            }
        }
        _ => {
            quote! {
                compile_error!("Only named fields are supported");
            }
        }
    }
}

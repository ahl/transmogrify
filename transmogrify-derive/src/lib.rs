use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Transmogrify, attributes(transmogrify))]
pub fn transmogrify_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match do_transmogrify_derive(input) {
        Ok(output) => output.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

struct TransmogrifyAttr {
    path: syn::Path,
}

impl syn::parse::Parse for TransmogrifyAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let prefix = input.parse::<syn::Ident>()?;
        if prefix != "prefix" {
            return Err(syn::Error::new(prefix.span(), "expected `prefix`"));
        }
        let _ = input.parse::<syn::Token![=]>()?;
        let path = input.parse()?;
        Ok(Self { path })
    }
}

fn do_transmogrify_derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let mut errors = Vec::new();
    let mut prefix = TokenStream::new();
    let mut found = false;

    for attr @ syn::Attribute { meta, .. } in &input.attrs {
        match meta {
            syn::Meta::List(syn::MetaList { path, tokens, .. })
                if path.segments.len() == 1
                    && path.segments.last().unwrap().ident == "transmogrify" =>
            {
                found = true;
                match syn::parse2::<TransmogrifyAttr>(tokens.clone()) {
                    Ok(TransmogrifyAttr { path, .. }) => {
                        prefix = path.to_token_stream();
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }
            syn::Meta::Path(path) | syn::Meta::NameValue(syn::MetaNameValue { path, .. })
                if path.segments.len() == 1
                    && path.segments.last().unwrap().ident == "transmogrify" =>
            {
                found = true;
                errors.push(syn::Error::new(
                    attr.span(),
                    "must be of the form #[transmogrify(prefix = <path>)]",
                ));
            }
            _ => (),
        }
    }

    if !found {
        errors.push(syn::Error::new(
            input.span(),
            "must specify a path prefix #[transmogrify(prefix = <path>)]",
        ));
    }

    // Do validation of the input types.
    match &input.vis {
        syn::Visibility::Public(_) => {}
        _ => {
            errors.push(syn::Error::new(
                input.span(),
                "the type must be pub for consumers to use Transmogrify output",
            ));
        }
    }
    match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            for field in fields {
                match &field.vis {
                    syn::Visibility::Public(_) => {}
                    _ => {
                        errors.push(syn::Error::new(field.span(), "struct fields must be pub"));
                    }
                }
            }
        }
        syn::Data::Enum(_) => {}
        syn::Data::Union(_) => {
            return Err(syn::Error::new(
                input.span(),
                "Transmogrify my not be derived from unions",
            ))
        }
    }

    let name = &input.ident;
    let transmogrify = quote! {
        ::transmogrify::Transmogrify::transmogrify
    };
    let pound = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);

    let body = match &input.data {
        syn::Data::Struct(s) => {
            let fields = &s.fields;
            match fields {
                syn::Fields::Named(fields) => {
                    let field = fields
                        .named
                        .iter()
                        .map(|syn::Field { ident, .. }| ident)
                        .collect::<Vec<_>>();
                    quote! {
                        #(
                            let #field = #transmogrify(&self.#field);
                        )*
                        quote::quote! {
                            #prefix::#name {
                                #( #field: #pound #field, )*
                            }
                        }
                    }
                }
                syn::Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
                    quote! {
                        quote::quote! {
                            #prefix::#name()
                        }
                    }
                }
                syn::Fields::Unnamed(fields) => {
                    let (index, var): (Vec<_>, Vec<_>) = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(ii, _)| (syn::Index::from(ii), format_ident!("value_{}", ii)))
                        .unzip();
                    quote! {
                        #(
                            let #var = #transmogrify(&self.#index);
                        )*
                        quote::quote! {
                            #prefix::#name (
                                #( #pound #var, )*
                            )
                        }
                    }
                }
                syn::Fields::Unit => quote! {
                    quote::quote!{
                        #prefix::#name
                    }
                },
            }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let variants = variants
                .into_iter()
                .map(|syn::Variant { ident, fields, .. }| match fields {
                    syn::Fields::Named(fields) => {
                        let field = fields
                            .named
                            .iter()
                            .map(|syn::Field { ident, .. }| ident)
                            .collect::<Vec<_>>();
                        quote! {
                            Self::#ident{ #( #field, )* } => {
                                #(
                                    let #field = #transmogrify(#field);
                                )*
                                quote::quote! {
                                    #prefix::#name::#ident {
                                        #( #field: #pound #field, )*
                                    }
                                }
                            }
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let field = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(ii, _)| format_ident!("x{}", ii))
                            .collect::<Vec<_>>();
                        quote! {
                            Self::#ident( #( #field, )* ) => {
                                #(
                                    let #field = #transmogrify(#field);
                                )*
                                quote::quote! {
                                    #prefix::#name::#ident (
                                        #( #pound #field, )*
                                    )
                                }
                            }
                        }
                    }
                    syn::Fields::Unit => quote! {
                            Self::#ident => {
                                quote::quote! {
                                    #prefix::#name::#ident
                                }
                            }

                    },
                });
            quote! {
                match self {
                    #( #variants, )*
                }
            }
        }
        syn::Data::Union(_) => unreachable!(),
    };

    let error_out = errors.into_iter().map(|x| x.into_compile_error());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        #( #error_out )*

        impl #impl_generics ::transmogrify::Transmogrify
            for #name #ty_generics #where_clause
        {
            fn transmogrify(&self) -> proc_macro2::TokenStream {
                #body
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use heck::ToSnakeCase;
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::do_transmogrify_derive;

    fn type_tester(item: TokenStream) {
        let input: syn::DeriveInput = syn::parse_quote! {
            #item
        };

        let file_name = format!(
            "tests/data/type_{}.rs",
            input.ident.to_string().to_snake_case()
        );

        let output = do_transmogrify_derive(input.clone()).expect("invalid type");

        let file = syn::parse_quote! {
            #input
            #output
        };

        let text = prettyplease::unparse(&file);
        expectorate::assert_contents(file_name, &text);
    }

    #[test]
    fn test_simple_struct() {
        type_tester(quote! {
            #[transmogrify(prefix = foo_crate)]
            pub struct SimpleStruct {
                pub foo: String,
            }
        });
    }

    #[test]
    fn test_tuple_struct() {
        type_tester(quote! {
            #[transmogrify(prefix = foo_crate)]
            pub struct TupleStruct(pub String);
        });
    }

    #[test]
    fn test_empty_struct() {
        type_tester(quote! {
            #[transmogrify(prefix = foo_crate)]
            pub struct EmptyStruct {}
        });
    }
    #[test]
    fn test_empty_struct_tuple() {
        type_tester(quote! {
            #[transmogrify(prefix = foo_crate)]
            pub struct EmptyStructTuple();
        });
    }
    #[test]
    fn test_marker_struct() {
        type_tester(quote! {
            #[transmogrify(prefix = foo_crate)]
            pub struct MarkerStruct;
        });
    }

    #[test]
    fn test_simple_enum() {
        type_tester(quote! {
            #[transmogrify(prefix = foo_crate)]
            pub enum SimpleEnum {
                A,
                B(),
                C(String),
                D {
                    foo: String,
                }
            }
        });
    }

    #[test]
    fn test_error_no_attr() {
        type_tester(quote! {
            pub struct ErrorNoAttr {}
        });
    }
}

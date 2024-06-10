//! This macro library is intended only for internal use by the transmogrify
//! crate. It provides the [`transmogrify`] macro that is used to assist
//! implementation of the Transmogrify trait on foreign types.

use quote::{format_ident, ToTokens};
use syn::{spanned::Spanned, Expr, ImplItem, Item, Member, PatStruct, PatTupleStruct, Stmt};

#[proc_macro_attribute]
pub fn transmogrify(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match do_transmogrify(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn do_transmogrify(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut item = syn::parse2::<Item>(item)?;

    let Item::Impl(item_impl) = &mut item else {
        // This is a hard error: the user has applied the macro to a item of
        // the wrong kind and we can't recover.
        return Err(syn::Error::new_spanned(
            item,
            "#[transmogrify] must be applied to an \
            `impl transmogrify for ... {}` block.",
        ));
    };

    let mut soft_errors = Vec::new();

    let self_ty = get_self_type(item_impl.self_ty.as_ref());

    // If the user specifies extraneous items, we'll still try our best so as
    // to minimize superfluous compilation errors.
    if item_impl.items.len() > 1 {
        let mut tokens = item_impl.items[1].to_token_stream();
        item_impl.items[2..]
            .iter()
            .for_each(|x| tokens.extend(x.to_token_stream()));
        soft_errors.push(
            syn::Error::new_spanned(
                tokens,
                format!(
                    "{}\n{}",
                    "expected a single item of the form",
                    "fn transmogrify(&self) -> proc_macro2::TokenStream {}"
                ),
            )
            .into_compile_error(),
        );
    }

    let Some(ImplItem::Fn(t_fn)) = item_impl.items.first_mut() else {
        let err = syn::Error::new(
            item_impl.brace_token.span.join(),
            format!(
                "{}\n{}",
                "expected a single item of the form",
                "fn transmogrify(&self) -> proc_macro2::TokenStream {}"
            ),
        )
        .into_compile_error();
        item_impl.items.push(syn::parse_quote! {
            fn transmogrify(&self) -> proc_macro2::TokenStream {
                todo!()
            }
        });
        return Ok(quote::quote! {
            #item
            #err
        });
    };

    // TODO Confirm type signature
    let block = &mut t_fn.block;

    if let Some(Stmt::Expr(Expr::Match(expr), _)) = block.stmts.first_mut() {
        // We assume a single statement of the form `match self { .. }``
        translate_match_expr(self_ty, expr)
    } else if let Some(Stmt::Local(_)) = block.stmts.first() {
        let stmts = std::mem::take(&mut block.stmts);
        let Some(Stmt::Local(local)) = stmts.first() else {
            unreachable!()
        };
        if let Err(e) = translate_local_expr(self_ty, block, local) {
            block.stmts = stmts;
            soft_errors.push(e.into_compile_error());
        }
    } else {
        let message = "#[transmogrify] requires a single statement \
        in fn transmogrify that is a match self { .. } with all cases \
        populated; rust-analyzer can help auto-generated this";
        return Err(syn::Error::new_spanned(block, message));
    }

    Ok(quote::quote! {
        #item
        #( #soft_errors )*
    })
}

fn get_self_type(self_ty: &syn::Type) -> syn::Type {
    match self_ty {
        syn::Type::Path(p) => {
            // Strip out type path arguments
            let segments = p
                .path
                .segments
                .iter()
                .map(|seg| syn::PathSegment {
                    ident: seg.ident.clone(),
                    arguments: syn::PathArguments::None,
                })
                .collect();
            let path = syn::Path {
                leading_colon: p.path.leading_colon,
                segments,
            };
            let tp = syn::TypePath { qself: None, path };
            syn::Type::Path(tp)
        }
        any => any.clone(),
    }
}

fn translate_local_expr(
    self_ty: syn::Type,
    block: &mut syn::Block,
    local: &syn::Local,
) -> syn::Result<()> {
    match &local.pat {
        syn::Pat::Struct(PatStruct {
            attrs: _,
            qself,
            path: _,
            brace_token: _,
            fields,
            rest,
        }) => {
            assert!(qself.is_none());
            assert!(rest.is_none());

            let pound = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);
            let members = fields
                .iter()
                .map(|field| {
                    let Member::Named(member) = &field.member else {
                        // TODO I don't think this is possible from a `PatStruct`
                        panic!("bad member")
                    };

                    member
                })
                .collect::<Vec<_>>();

            // Transmogrify the values; we use the span of the member so
            // that untransmogrifiable types point their error at the name
            // of the member.
            let transitive_transmogrification = members.iter().map(|member| {
                quote::quote_spanned! {member.span()=>
                    let #member = #member.transmogrify()
                }
            });
            // Replace the old body which should be something like:
            // {
            //     let Self { ... } = self;
            //     todo!()
            // }
            *block = syn::parse_quote! {
                {
                    #local
                    #(
                        #transitive_transmogrification;
                    )*
                    quote::quote!{
                        #self_ty {
                            #( #members: #pound #members, )*
                        }
                    }
                }
            };
            Ok(())
        }
        pat => Err(syn::Error::new_spanned(
            pat,
            "transmogrify! expected `Self { /* all members */ }",
        )),
    }
}

fn translate_match_expr(self_ty: syn::Type, expr: &mut syn::ExprMatch) {
    expr.arms.iter_mut().for_each(|arm| {
        match &mut arm.pat {
            syn::Pat::Path(syn::PatPath { path, .. }) => {
                let path = xxx_path(path, &self_ty);
                arm.body = syn::parse_quote! {
                    quote::quote!{
                        #path
                    }
                };
            }

            syn::Pat::TupleStruct(PatTupleStruct {
                attrs,
                qself,
                path,
                paren_token: _,
                elems,
            }) => {
                // TODO convert to syn::Error
                assert!(attrs.is_empty());
                assert!(qself.is_none());

                let path = xxx_path(path, &self_ty);

                let pound = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);

                // Transmogrify the values; we use the span of the member so
                // that untransmogrifiable types point their error at the name
                // of the member.
                let transitive_transmogrification = elems
                    .iter()
                    .enumerate()
                    .map(|(i, pat)| {
                        let value = format_ident!("value_{}", i);
                        quote::quote_spanned! {pat.span()=>
                            let #value = #value.transmogrify()
                        }
                    })
                    .collect::<Vec<_>>();
                elems.iter_mut().enumerate().for_each(|(i, pat)| {
                    // TODO make this an error or do some validation pass
                    // first.
                    if matches!(pat, syn::Pat::Rest(_)) {
                        panic!("this needs to be some error; need each element")
                    }
                    let value = format_ident!("value_{}", i);
                    *pat = syn::parse_quote_spanned! {pat.span()=>
                        #value
                    };
                });
                let xxx = elems.iter().collect::<Vec<_>>();
                arm.body = syn::parse_quote! {
                    {
                        #(
                            #transitive_transmogrification;
                        )*
                        quote::quote!{
                            #path ( #( #pound #xxx, )* )
                        }
                    }
                };
            }
            syn::Pat::Struct(PatStruct {
                attrs,
                qself,
                path,
                brace_token: _,
                fields,
                rest,
            }) => {
                // TODO convert to syn::Error
                assert!(attrs.is_empty());
                assert!(qself.is_none());
                assert!(rest.is_none());

                let path = xxx_path(path, &self_ty);

                let pound = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);
                let members = fields
                    .iter()
                    .map(|field| {
                        let Member::Named(member) = &field.member else {
                            // TODO I don't think this is possible from a `PatStruct`
                            panic!("bad named member")
                        };

                        member
                    })
                    .collect::<Vec<_>>();

                // Transmogrify the values; we use the span of the member so
                // that untransmogrifiable types point their error at the name
                // of the member.
                let transitive_transmogrification = members.iter().map(|member| {
                    quote::quote_spanned! {member.span()=>
                        let #member = #member.transmogrify()
                    }
                });
                // Replace the old body which should be something like `todo!()`
                arm.body = syn::parse_quote! {
                    {
                        #(
                            #transitive_transmogrification;
                        )*
                        quote::quote!{
                            #path {
                                #( #members: #pound #members, )*
                            }
                        }
                    }
                };
            }
            syn::Pat::Const(_) => todo!(),
            syn::Pat::Ident(_) => todo!(),
            syn::Pat::Lit(_) => todo!(),
            syn::Pat::Macro(_) => todo!(),
            syn::Pat::Or(_) => todo!(),
            syn::Pat::Paren(_) => todo!(),
            syn::Pat::Range(_) => todo!(),
            syn::Pat::Reference(_) => todo!(),
            syn::Pat::Rest(_) => todo!(),
            syn::Pat::Slice(_) => todo!(),
            syn::Pat::Tuple(_) => todo!(),
            syn::Pat::Type(_) => todo!(),
            syn::Pat::Verbatim(_) => todo!(),
            syn::Pat::Wild(_) => todo!(),
            _ => todo!(),
        }
    });
}

fn xxx_path(path: &mut syn::Path, self_ty: &syn::Type) -> proc_macro2::TokenStream {
    let mut xxx = path.segments.iter();
    let Some(first) = xxx.next() else {
        unreachable!();
    };

    if first.arguments.is_none() && first.ident == "Self" {
        quote::quote! {
            #self_ty #( ::#xxx)*
        }
    } else {
        quote::quote! {
            #first #( ::#xxx)*
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::do_transmogrify;

    use quote::quote;

    #[test]
    fn test_transmogrify_enum() {
        let input = quote! {
            impl Transmogrify for TestEnum {
                fn transmogrify(&self) -> proc_macro2::TokenStream {
                    match self {
                        Self::A => todo!(),
                        TestEnum::B(_, _) => todo!(),
                        TestEnum::C { a, b } => todo!(),
                    }
                }
            }
        };
        let output = do_transmogrify(input).unwrap();

        let pound = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);
        let expected = quote! {
            impl Transmogrify for TestEnum {
                fn transmogrify(&self) -> proc_macro2::TokenStream {
                    match self {
                        Self::A => {
                            quote::quote! {
                                TestEnum::A
                            }
                        }
                        TestEnum::B(value_0, value_1) => {
                            let value_0 = value_0.transmogrify();
                            let value_1 = value_1.transmogrify();
                            quote::quote! {
                                TestEnum::B(#pound value_0, #pound value_1,)
                            }
                        }
                        TestEnum::C { a, b } => {
                            let a = a.transmogrify();
                            let b = b.transmogrify();
                            quote::quote! {
                                TestEnum::C {
                                    a: #pound a,
                                    b: #pound b,
                                }
                            }
                        }
                    }
                }
            }
        };

        let output_str = prettyplease::unparse(&syn::parse2(output).unwrap());
        let expected_str = prettyplease::unparse(&syn::parse2(expected).unwrap());

        pretty_assertions::assert_eq!(output_str, expected_str);
    }

    #[test]
    fn test_transmogrify_struct() {
        let input = quote! {
            impl Transmogrify for TestStruct {
                fn transmogrify(&self) -> proc_macro2::TokenStream {
                    let Self { a, b } = self;
                }
            }
        };
        let output = do_transmogrify(input).unwrap();

        let pound = proc_macro2::Punct::new('#', proc_macro2::Spacing::Joint);
        let expected = quote! {
            impl Transmogrify for TestStruct {
                fn transmogrify(&self) -> proc_macro2::TokenStream {
                    let Self { a, b } = self;
                    let a = a.transmogrify();
                    let b = b.transmogrify();
                    quote::quote! {
                        TestStruct { a: #pound a, b: #pound b, }
                    }
                }
            }
        };

        let output_str = prettyplease::unparse(&syn::parse2(output).unwrap());
        let expected_str = prettyplease::unparse(&syn::parse2(expected).unwrap());

        pretty_assertions::assert_eq!(output_str, expected_str);
    }
}

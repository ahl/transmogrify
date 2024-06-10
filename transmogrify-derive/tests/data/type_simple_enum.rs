#[transmogrify(prefix = foo_crate)]
enum SimpleEnum {
    A,
    B(),
    C(String),
    D { foo: String },
}
impl ::transmogrify::Transmogrify for SimpleEnum {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self {
            Self::A => {
                quote::quote! {
                    foo_crate::SimpleEnum::A
                }
            }
            Self::B() => {
                quote::quote! {
                    foo_crate::SimpleEnum::B()
                }
            }
            Self::C(x0) => {
                let x0 = ::transmogrify::Transmogrify::transmogrify(x0);
                quote::quote! {
                    foo_crate::SimpleEnum::C(#x0,)
                }
            }
            Self::D { foo } => {
                let foo = ::transmogrify::Transmogrify::transmogrify(foo);
                quote::quote! {
                    foo_crate::SimpleEnum::D { foo : #foo, }
                }
            }
        }
    }
}

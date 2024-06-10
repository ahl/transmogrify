use crate::Transmogrify;

use quote::quote;

impl<T: Transmogrify> Transmogrify for Option<T> {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        match self.as_ref().map(Transmogrify::transmogrify) {
            Some(t) => quote! { Some(#t) },
            None => quote! { None },
        }
    }
}

impl<T: Transmogrify> Transmogrify for Box<T> {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        let t = self.as_ref().transmogrify();
        quote! {
            Box::new(#t)
        }
    }
}

impl<T: Transmogrify> Transmogrify for Vec<T> {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        if self.is_empty() {
            quote! {
                <std::vec::Vec<_>>::new()
            }
        } else {
            let items = self.iter().map(Transmogrify::transmogrify);
            quote! {
                <std::vec::Vec<_>>::from([
                    #( #items, )*
                ])
            }
        }
    }
}

impl<K: Transmogrify, V: Transmogrify> Transmogrify for std::collections::BTreeMap<K, V> {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        if self.is_empty() {
            quote! {
                <std::collections::BTreeMap<_, _>>::new()
            }
        } else {
            let kv = self.iter().map(|(k, v)| {
                let k = k.transmogrify();
                let v = v.transmogrify();
                quote! { (#k, #v) }
            });

            quote! {
               <std::collections::BTreeMap<_, _>>::from([
                    #( #kv, )*
                ])
            }
        }
    }
}

impl<T: Transmogrify> Transmogrify for std::collections::BTreeSet<T> {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        if self.is_empty() {
            quote! {
               <std::collections::BTreeSet<_, _>>::new()
            }
        } else {
            let values = self.iter().map(|value| value.transmogrify());
            quote! {
               <std::collections::BTreeSet<_, _>>::from([
                    #( #values, )*
                ])
            }
        }
    }
}

impl Transmogrify for String {
    fn transmogrify(&self) -> proc_macro2::TokenStream {
        quote! {
            String::from(#self)
        }
    }
}

macro_rules! quote_impl {
    ($ty:ident) => {
        impl Transmogrify for $ty {
            fn transmogrify(&self) -> proc_macro2::TokenStream {
                quote! {
                    #self
                }
            }
        }
    };
}

quote_impl!(bool);
quote_impl!(i8);
quote_impl!(i16);
quote_impl!(i32);
quote_impl!(i64);
quote_impl!(u8);
quote_impl!(u16);
quote_impl!(u32);
quote_impl!(u64);
quote_impl!(f32);
quote_impl!(f64);

macro_rules! non_zero_impl {
    ($ty:ty, $inner:ident) => {
        impl Transmogrify for $ty {
            fn transmogrify(&self) -> proc_macro2::TokenStream {
                let value = self.get();
                quote! {
                    $ty::new(#value).unwrap()
                }
            }
        }
    };
}

non_zero_impl!(std::num::NonZeroU8, u8);
non_zero_impl!(std::num::NonZeroU16, u16);
non_zero_impl!(std::num::NonZeroU32, u32);
non_zero_impl!(std::num::NonZeroU64, u64);
non_zero_impl!(std::num::NonZeroU128, u128);
non_zero_impl!(std::num::NonZeroUsize, usize);
non_zero_impl!(std::num::NonZeroI8, i8);
non_zero_impl!(std::num::NonZeroI16, i16);
non_zero_impl!(std::num::NonZeroI32, i32);
non_zero_impl!(std::num::NonZeroI64, i64);
non_zero_impl!(std::num::NonZeroI128, i128);
non_zero_impl!(std::num::NonZeroIsize, isize);

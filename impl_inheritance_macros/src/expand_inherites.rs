use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident,Span};

use syn::{ItemStruct};
use crate::common::*;

pub(crate) fn expand(item: ItemStruct) -> TokenStream2 {
    use syn::Fields::*;
    let fields = match item.fields {
        Named(x) => x.clone(),
        Unnamed(_x) => panic!("structs with unnamed fields are not supported"),
        Unit => panic!("unic structs are not supported"),
    };
    let super_fields: Vec<_> = fields.named.iter()
    .filter(|x| x.attrs.iter().any(|y| y.path.is_ident("super_data")))
    .collect();
    if super_fields.len() != 1 {
        panic!("exacly one field must be marker with #[super_data] attribute");
    }
    let field = super_fields.into_iter().next().unwrap();
    let attr = field.attrs.iter().filter(|x| x.path.is_ident("super_data"))
    .next().unwrap().clone();

    let constrait_ts = (0..MAX_CONSTRAITS).into_iter().map( |i| {
        let ident_prev = Ident::new(&format!("Constrait{}", i), Span::call_site());
        let ident = Ident::new(&format!("Constrait{}", i + 1), Span::call_site());
        quote!{ type #ident = <Self::#ident_prev as ::impl_inheritance::SuperType>::SupType;}
    }).collect::<TokenStream2>();

    let struct_ident = &item.ident;
    let field_ident = &field.ident;
    let field_type = &field.ty;

    let self_decompose = match attr.tts.to_string().trim() {
        "" => quote!{ self.#field_ident},
        //TODO: better implementation of parsing, other options (swap with default/unimplemented)
        "( clone )" => quote!{self.#field_ident.clone()},
        _ => panic!("unsupported #[super_data] value {}", attr.tts.to_string().trim()),
    };

    quote!{
        impl ::impl_inheritance::SuperBorrow<#field_type> for #struct_ident
        {
            fn super_ref(&self) -> & #field_type {
                & self.#field_ident
            }

            fn super_ref_mut(& mut self) -> & mut #field_type {
                & mut self.#field_ident
            }
            fn super_value(self) -> #field_type{
                #self_decompose
            }
        }

        impl<T> ::impl_inheritance::SuperBorrow<T> for #struct_ident
        where T : ::impl_inheritance::IsSuperBorrowableTo<#field_type> ,
        T:  Sized
        {
            fn super_ref(&self) -> & T {
                ::impl_inheritance::IsSuperBorrowableTo::get_part(&self.#field_ident)
            }

            fn super_ref_mut(& mut self) -> & mut T {
                ::impl_inheritance::IsSuperBorrowableTo::get_part_mut(& mut self.#field_ident)
            }

            fn super_value(self) -> T {
                ::impl_inheritance::IsSuperBorrowableTo::get_part_value(#self_decompose)
            }
        }

        impl  ::impl_inheritance::SuperType for #struct_ident {
            type SupType = #field_type;
        }

        impl ::impl_inheritance::Constraits for  #struct_ident {
            type Constrait0 = Self;
            #constrait_ts
        }
    }
}


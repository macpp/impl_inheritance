
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident,Span};
use syn::{ItemImpl,punctuated::Punctuated,FnArg,token::Comma};
use crate::common::*;

pub(crate)fn expand(trait_impl: ItemImpl) -> TokenStream2 {
    let trait_ident = match trait_impl.trait_ {
        None => panic!("override is only for trait impls"),
        Some(x) => x.1,
    };
    let struct_ident = trait_impl.self_ty;

    let mut con_ts = TokenStream2::new();
    for i in 0..=MAX_CONSTRAITS {
        let ident = Ident::new(&format!("Constrait{}",i), Span::call_site());
        con_ts.extend(quote!{T: ::impl_inheritance::SuperBorrow< < #struct_ident as  ::impl_inheritance::Constraits>:: #ident > ,})
    }

    let impl_items = trait_impl.items.iter().map( |item| {
        use syn::ImplItem;
        match item {
            ImplItem::Method(data) => {
                //TODO: support for const and unsafe
                //TODO: support for generic fn
                let ident = &data.sig.ident;
                let inputs = &data.sig.decl.inputs;
                let return_type = &data.sig.decl.output;

                if let Some((super_method, ident_prefix)) = get_super_method(inputs.iter().next()) {

                    let mut unpacked_inputs = unpack_fn_arg(inputs).into_iter();
                    unpacked_inputs.next();
                    let unpacked_inputs : Punctuated<FnArg, Comma> = unpacked_inputs.collect();

                    //TODO: support super_ref_mut and super_value
                    //TODO: support mut self
                    quote!{
                        default fn #ident(#inputs) #return_type {
                            let data : #ident_prefix #struct_ident = self.#super_method();
                            data.#ident(#unpacked_inputs)
                        }
                    }
                } else {
                    let unpacked_inputs = unpack_fn_arg(inputs);
                    quote!{
                        default fn #ident(#inputs) #return_type{
                           <#struct_ident as #trait_ident> :: #ident(#unpacked_inputs)
                        }
                    }
                }
            },
            ImplItem::Type(data) => {
                let ident = &data.ident;
                quote!{
                    default type #ident = <#struct_ident as #trait_ident>:: #ident;
                }

            },
            ImplItem::Const(data) => {
                let ident = &data.ident;
                let type_name = &data.ty;
                quote!{
                    default const #ident : #type_name = <#struct_ident as #trait_ident>:: #ident;
                }
            }
            ImplItem::Macro(_data) => panic!("macros in trait implementations are not supported"),
            ImplItem::Verbatim(_data) => panic!("verbatim tokens in trait implementations are not supported"),
            ImplItem::Existential(_data) => panic!("exssistential types in trait implementations are not supported")
        }
    }).collect::<TokenStream2>();

    let stub_ident = get_stub_path(trait_ident.clone());

    quote!{
        impl <T> #trait_ident for T
        where
        T: #stub_ident,
        #con_ts
        {
            #impl_items
        }
        
    }
}



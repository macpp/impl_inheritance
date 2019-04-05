
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident,Span};
use crate::MAX_CONSTRAITS;
use syn::{ItemImpl,punctuated::Punctuated,FnArg,token::Comma};

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

    let mut impl_items = TokenStream2::new();
        use syn::ImplItem;
        for item in trait_impl.items.iter() {
        match item {
            ImplItem::Method(data) => {
                //TODO: support for const and unsafe
                //TODO: support for generic fn
                let ident = &data.sig.ident;
                let inputs = &data.sig.decl.inputs;
                let return_type = &data.sig.decl.output;

                use syn::FnArg::*;
                let super_method : Ident = match inputs.iter().next(){
                    None => panic!("trait with no methods are not supported"),
                    Some(SelfRef(x)) => {
                            if x.mutability.is_some() 
                            {
                                Ident::new("super_ref_mut", Span::call_site())
                            } 
                            else 
                            {
                                Ident::new("super_ref", Span::call_site())
                            }
                        },
                    Some(SelfValue(_x)) => panic!("methods with `self` by value are not yet supported"),//Ident::new("super_value", Span::call_site()),
                    _ => panic!("methods in trait with no self are not supported!")
                };

                let mut unpacked_inputs = crate::unpack_fn_arg(inputs).into_iter();
                unpacked_inputs.next();
                let unpacked_inputs : Punctuated<FnArg, Comma> = unpacked_inputs.collect();

                //TODO: support super_ref_mut and super_value
                //TODO: support mut self
                impl_items.extend(quote!{
                    default fn #ident(#inputs) #return_type {
                        let data : & #struct_ident = self.#super_method();
                        data.#ident(#unpacked_inputs)
                    }
                });
            },
            ImplItem::Type(data) => {
                let ident = &data.ident;
                impl_items.extend(quote!{
                    default type #ident = <#struct_ident as #trait_ident>:: #ident;
                });

            },
            ImplItem::Const(data) => {
                let ident = &data.ident;
                let type_name = &data.ty;
                impl_items.extend(quote!{
                    default const #ident : #type_name = <#struct_ident as #trait_ident>:: #ident;
                });
            }
            ImplItem::Macro(_data) => panic!("macros in trait implementations are not supported"),
            ImplItem::Verbatim(_data) => panic!("verbatim tokens in trait implementations are not supported"),
            ImplItem::Existential(_data) => panic!("exssistential types in trait implementations are not supported")
        }
    }

    quote!{
        impl <T> #trait_ident for T
        where
        #con_ts
        {
            #impl_items
        }
        
    }
}



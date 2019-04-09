
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident};
use crate::common::*;

use syn::{ItemTrait,punctuated::Punctuated,FnArg,token::Comma};

pub (crate) fn extract_ident(attr: TokenStream2) -> Option<Ident> {
    let attr_ts : Vec<_> = attr.into_iter().collect();
    if attr_ts.len() != 1 {
        return None;
    }
    match &attr_ts[0] {
        proc_macro2::TokenTree::Ident(id) => Some(id.clone()),
        _ => None
    }
}

pub(crate)fn expand(trait_item: ItemTrait, struct_ident: Ident) -> TokenStream2 {
    //TODO: generics
    //TODO: generate documentation

    let trait_ident = &trait_item.ident;
    let parent_trait_constraint = trait_item.supertraits.iter()
    .map(|x| {
        let ident = match x {
                    syn::TypeParamBound::Trait(t) => &t.path,
                    syn::TypeParamBound::Lifetime(_l) => panic!("lifetime bounds in trait inheritance not supported")
            };
            quote!{ T : #ident,}
    })
    .collect::<TokenStream2>();
    
    let impl_items = trait_item.items.iter(). map( |item| {
        use syn::TraitItem;
        match item {
            TraitItem::Method(data) => {
                //TODO: support for const and unsafe
                //TODO: support for generic fn
                let ident = &data.sig.ident;
                let inputs = &data.sig.decl.inputs;
                let return_type = &data.sig.decl.output;

                if let Some((super_method, _)) = get_super_method(inputs.iter().next()) {

                    let mut unpacked_inputs = unpack_fn_arg(inputs).into_iter();
                    unpacked_inputs.next();
                    let unpacked_inputs : Punctuated<FnArg, Comma> = unpacked_inputs.collect();

                    //TODO: support super_value
                    //TODO: consider option for non including `default` by default, but by option
                    quote!{
                        default fn #ident(#inputs) #return_type {
                            self.#super_method().#ident(#unpacked_inputs)
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
            TraitItem::Type(data) => {
                let ident = &data.ident;
                quote!{
                    default type #ident = <#struct_ident as #trait_ident>:: #ident;
                }
            },
            TraitItem::Const(data) => {
                let ident = &data.ident;
                let type_name = &data.ty;
                quote!{
                    default const #ident : #type_name = <#struct_ident as #trait_ident>:: #ident;
                }
            },
            TraitItem::Macro(_data) => panic!("macros in trait declarations are not supported"),
            TraitItem::Verbatim(_data) => panic!("verbatim tokens in trait declarations are not supported")
        }
    } ).collect::<TokenStream2>();

    let stub_ident = get_stub_ident(trait_ident);
    let supertraits = &trait_item.supertraits;

    quote!{
        impl <T> #trait_ident for T 
        where T : ::impl_inheritance::SuperBorrow<#struct_ident>, 
        T : #stub_ident
        {
            #impl_items
        }

        #[doc(hidden)]
        trait #stub_ident : #supertraits {}

        impl <T> #stub_ident for T 
        where 
        #parent_trait_constraint {}
    }
}



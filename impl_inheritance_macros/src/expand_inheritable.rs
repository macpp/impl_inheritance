
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident,Span};

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
    let mut fns = TokenStream2::new();

    let trait_ident = &trait_item.ident;

    let parent_trait_constraint = {
        let supertr = trait_item.supertraits.iter()
        .map(|x| x.clone())
        .collect::<Vec<_>>();
        match supertr.len() {
            0 => quote!{},
            1 => {
                let ident = match &supertr[0] {
                    syn::TypeParamBound::Trait(t) => &t.path,
                    syn::TypeParamBound::Lifetime(_l) => panic!("lifetime bounds in trait inheritance not supported")
                };
                quote!{ T : #ident,}
            },
            _ =>  panic!("traits with more than one supertrait are not supported")
        }
    };

    for item in trait_item.items.iter() {
        match item {
            syn::TraitItem::Method(data) => {
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
                    Some(SelfValue(_x)) => Ident::new("super_value", Span::call_site()),
                    _ => panic!("methods in trait with no self are not supported!")
                };

                let mut unpacked_inputs = crate::unpack_fn_arg(inputs).into_iter();
                unpacked_inputs.next();
                let unpacked_inputs : Punctuated<FnArg, Comma> = unpacked_inputs.collect();

                //TODO: support super_value
                //TODO: consider option for non including `default` by default, but by option
                fns.extend(quote!{
                    default fn #ident(#inputs) #return_type {
                        self.#super_method().#ident(#unpacked_inputs)
                    }
                });
            },
            //TODO: remove for assosiated types and constants
            _ => return syn::Error::new_spanned(item,"this trait item is not supported!").to_compile_error()
        }
    }

    quote!{
        impl <T> #trait_ident for T 
        where T : ::impl_inheritance::SuperBorrow<#struct_ident>, 
        #parent_trait_constraint
        {
            #fns
        }
    }
}



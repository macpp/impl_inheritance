 use syn::{punctuated::Punctuated,FnArg,token::Comma,Ident};
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
///changes list of fn arguments from form of declaration to form of invocation
pub fn unpack_fn_arg( input: &Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
    use FnArg::*;
    input.iter()
    .map(|x| match x {
        SelfRef(arg) => {
            let arg = arg.clone();
            SelfValue(syn::ArgSelf{mutability: arg.mutability, self_token: arg.self_token})
        },
        Captured(arg) => {
            let arg = arg.clone();
            Inferred(arg.pat)
            
        }
        _ => x.clone()
    })
    .collect()
}

pub static  MAX_CONSTRAITS : u8 = 10;

pub fn get_super_method(x : Option<&FnArg>) -> Option<(Ident,TokenStream2)> {
    use syn::FnArg::*;
    match x{
        None => None,
        Some(SelfRef(x)) => {
                if x.mutability.is_some() 
                {
                    Some((Ident::new("super_ref_mut", Span::call_site()), quote!{& mut }))
                } 
                else 
                {
                    Some((Ident::new("super_ref", Span::call_site()), quote!{& }))
                }
            },
        Some(SelfValue(_x)) => {
                //panic!("methods with `self` by value are not yet supported")
                Some((Ident::new("super_value", Span::call_site()), quote!{}))
        },
        Some(Captured(_x)) => None,
        _ => panic!("some of the fn arguments are not supported!")
    }
}
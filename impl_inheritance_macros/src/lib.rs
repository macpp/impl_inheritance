#![recursion_limit="128"]
extern crate proc_macro;

mod expand_inheritable;
mod expand_inherites;
mod expand_base; 
mod expand_override;
mod expand_constraits;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

 use syn::{ItemTrait,ItemStruct,punctuated::Punctuated,FnArg,token::Comma,ItemImpl};

 use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn inheritable(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut out  = input.clone();
    let item = parse_macro_input!(input as ItemTrait);

    //let ident = parse_macro_input!(attr as Ident);
    let ident = expand_inheritable::extract_ident(attr.into()).expect("attribute must contain ident of the struct");
    let expanded : TokenStream = expand_inheritable::expand(item,ident).into();
    //println!("EXPANDED INHERITABLE: {}", &expanded);
    out.extend(expanded);
    out    
}

#[proc_macro_attribute]
pub fn overrides(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut out  = input.clone();
    let item = parse_macro_input!(input as ItemImpl);
    let expanded : TokenStream = expand_override::expand(item).into();
    //println!("EXPANDED: {}", &expanded);
    out.extend(expanded);
    out 
}

#[proc_macro_derive(Inherites, attributes(super_data))]
pub fn inherites(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let expanded : TokenStream =expand_inherites::expand(item).into();
    //println!("EXPANDED INHERITES: {}", &expanded);
    expanded
}

#[proc_macro_derive(Base, attributes(super_data))]
pub fn base(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let expanded : TokenStream =expand_base::expand(item).into();
    //println!("EXPANDED: {}", &expanded);
    expanded
}


pub (crate) static  MAX_CONSTRAITS : u8 = 10;

#[proc_macro]
pub fn expand_constraits_def(_in: TokenStream) -> TokenStream {
    let result = expand_constraits::expand();
    //println!("EXPANDED: {}", result.to_string());
    result.into()
}


///changes list of fn arguments from form of declaration to form of invocation
pub(crate) fn unpack_fn_arg( input: &Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
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


use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident,Span};
use crate::common::*;

pub(crate)fn expand() -> TokenStream2 {
    let ts = (0..=MAX_CONSTRAITS).into_iter().map(|i| {
        let ident = Ident::new(&format!("Constrait{}",i), Span::call_site());
        quote!{type #ident ;}
    }).collect::<TokenStream2>();

    quote!{
        #[doc(hidden)]
       pub trait Constraits {
           #ts
        } 
    }
}

//example generated output
/*
            type Constrait0;
            type Constrait1;
            type Constrait2;
            type Constrait3;
            type Constrait4;
            type Constrait5;
            type Constrait6;
            type Constrait7;
            type Constrait8;
            type Constrait9;
            type Constrait10;
*/


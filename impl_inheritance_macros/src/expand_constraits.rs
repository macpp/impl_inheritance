
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident,Span};
use crate::MAX_CONSTRAITS;

pub(crate)fn expand() -> TokenStream2 {
    let mut ts = TokenStream2::new();
    for i in 0..=MAX_CONSTRAITS {
        let ident = Ident::new(&format!("Constrait{}",i), Span::call_site());
        ts.extend(quote!{type #ident ;})
    }

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


use proc_macro2::TokenStream as TokenStream2;

use syn::{ItemStruct};


pub(crate) fn expand(item: ItemStruct) -> TokenStream2 {
    let struct_ident = &item.ident;
    quote!{
        impl ::impl_inheritance::SuperType for #struct_ident {
            type SupType = ::impl_inheritance::Placeholder;
        }

        impl ::impl_inheritance::SuperBorrow<::impl_inheritance::Placeholder> for #struct_ident
        {
            fn super_ref(&self) -> & ::impl_inheritance::Placeholder  {
                unimplemented!()
            }

            fn super_ref_mut(& mut self) -> & mut ::impl_inheritance::Placeholder {
                unimplemented!()
            }
            fn super_value(self) -> ::impl_inheritance::Placeholder {
                unimplemented!()
            }
        }
    }
}
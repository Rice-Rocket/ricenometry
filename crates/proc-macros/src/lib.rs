extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;


#[proc_macro_derive(IterEnum)]
pub fn derive_iter_enum(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    let Data::Enum(enum_data) = data else { panic!("Cannot derive IterEnum for non-enums") };
    let variant_idents = enum_data.variants.iter().map(|x| &x.ident);

    let iter_ident = quote::format_ident!("{}Iter", ident);

    quote! {
        impl #generics #ident #generics #where_clause {
            pub fn iter_fields() -> #iter_ident {
                #iter_ident { iter: vec![#(#ident::#variant_idents),*].into_iter() }
            }
        }

        pub struct #iter_ident #generics #where_clause {
            iter: std::vec::IntoIter<#ident>,
        }

        impl Iterator for #iter_ident {
            type Item = #ident;

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
        }
    }.into()
}

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Meta};
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

    let Data::Enum(enum_data) = data else { return quote! { () }.into() };
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


#[proc_macro_derive(StringifyEnum)]
pub fn derive_stringify_enum(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);
    
    let where_clause = &generics.where_clause;

    let Data::Enum(enum_data) = data else { return quote! { () }.into() };
    let variant_idents = enum_data.variants.iter().map(|x| &x.ident);
    let variant_fields = enum_data.variants.iter().map(|x| &x.fields);
    let variant_idents1 = variant_idents.clone();
    let variant_fields1 = variant_fields.clone();
    let variant_docs = enum_data.variants.iter()
        .map(|x| { 
            if x.attrs.is_empty() {
                None
            } else {
                Some(x.attrs.iter().map(|x| if let Meta::NameValue(v) = &x.meta { Some(v) } else { None }).map(|y| {
                    if let Some(x) = y {
                        if let Some(p) = x.path.get_ident() { 
                            if *p == "doc" { 
                                let Expr::Lit(lit) = &x.value else { unreachable!() };
                                let syn::Lit::Str(litstr) = &lit.lit else { unreachable!() };
                                litstr.value().trim().to_string()
                            } else { "".to_string() } 
                        } else { "".to_string() } 
                    } else { "".to_string() }
                }))
            }
        }).map(|x| if let Some(mut s) = x { s.next().unwrap() } else { "".to_string() });
    let variant_strs = variant_idents.clone().map(|x| x.to_string());

    quote! {
        impl #generics #ident #generics #where_clause {
            pub fn stringify_field(self) -> &'static str {
                match self {
                    #(
                        #ident::#variant_idents #variant_fields => #variant_strs,
                    )*
                }
            }

            pub fn stringify_pretty(self) -> &'static str {
                match self {
                    #(
                        #ident::#variant_idents1 #variant_fields1 => #variant_docs,
                    )*
                }
            }
        }
    }.into()
}

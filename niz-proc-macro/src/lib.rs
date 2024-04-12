use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Ident};

mod util;

#[proc_macro_derive(Hashable, attributes(niz))]
pub fn derive_hashable(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ident = &ast.ident;

    match &ast.data {
        Data::Struct(data) => expand_derive_hashable_for_struct(ident, data),
        Data::Enum(data) => expand_derive_hashable_for_enum(ident, data),
        _ => panic!("hashable can only be derived for structs and enums"),
    }
}

fn expand_derive_hashable_for_struct(ident: &Ident, data: &DataStruct) -> TokenStream {
    let hash_fields_impl = match &data.fields {
        Fields::Named(named) => named.named.iter().filter_map(|field| {
            if util::has_skip_attr(&field.attrs) {
                return None;
            }

            let field_ident = field.ident.as_ref().unwrap();

            Some(if util::has_json_attr(&field.attrs) {
                quote! {
                    {
                        let mut field_output = [0u8; 32];
                        let mut field_hasher = Sha3::v256();
                        field_hasher.update(&::niz::hash::prefix(stringify!(#field_ident)));
                        field_hasher.update(&::serde_json::to_value(&self.#field_ident).unwrap().hash());
                        field_hasher.finalize(&mut field_output);
                        hasher.update(&field_output);
                    }
                }
            } else {
                quote! {
                    {
                        let mut field_output = [0u8; 32];
                        let mut field_hasher = Sha3::v256();
                        field_hasher.update(&::niz::hash::prefix(stringify!(#field_ident)));
                        field_hasher.update(&self.#field_ident.hash());
                        field_hasher.finalize(&mut field_output);
                        hasher.update(&field_output);
                    }
                }
            })
        }),
        _ => panic!("hashable can only be derived for structs with named fields"),
    };

    let expanded = quote! {
        impl ::niz::hash::Hashable for #ident {
            fn hash(&self) -> [u8; 32] {
                use ::niz::tiny_keccak::{Hasher, Sha3};

                let mut output = [0u8; 32];
                let mut hasher = Sha3::v256();
                hasher.update(&::niz::hash::prefix(stringify!(#ident)));

                #(#hash_fields_impl)*

                hasher.finalize(&mut output);
                output
            }
        }
    };

    TokenStream::from(expanded)
}

fn expand_derive_hashable_for_enum(ident: &Ident, data: &DataEnum) -> TokenStream {
    let is_enum = data
        .variants
        .iter()
        .all(|variant| matches!(variant.fields, Fields::Unit));
    if !is_enum {
        panic!("hashable can only be derived for enums that can be cast to `u8`");
    }

    let hash_variants_impl = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        variant.discriminant.as_ref().map_or_else(
            || {
                quote! {
                    Self::#variant_ident => {
                        let mut variant_output = [0u8; 32];
                        let mut variant_hasher = Sha3::v256();
                        variant_hasher.update(&::niz::hash::prefix(stringify!(#variant_ident)));
                        variant_hasher.update(&(*self as u8).hash());
                        variant_hasher.finalize(&mut variant_output);
                        hasher.update(&variant_output);
                    }
                }
            },
            |(_, expr)| {
                quote! {
                    Self::#variant_ident => {
                        let mut variant_output = [0u8; 32];
                        let mut variant_hasher = Sha3::v256();
                        variant_hasher.update(&::niz::hash::prefix(stringify!(#variant_ident)));
                        variant_hasher.update(&(#expr).hash());
                        variant_hasher.finalize(&mut variant_output);
                        hasher.update(&variant_output);
                    }
                }
            },
        )
    });

    let expanded = quote! {
        impl ::niz::hash::Hashable for #ident {
            fn hash(&self) -> [u8; 32] {
                use ::niz::tiny_keccak::{Hasher, Sha3};

                let mut output = [0u8; 32];
                let mut hasher = Sha3::v256();
                hasher.update(&::niz::hash::prefix(stringify!(#ident)));

                match self {
                #(#hash_variants_impl)*
                }

                hasher.finalize(&mut output);
                output
            }
        }
    };

    TokenStream::from(expanded)
}

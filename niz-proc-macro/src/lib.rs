use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Hashable)]
pub fn derive_hasable(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ident = &ast.ident;

    let fields_hash_impl = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => named.named.iter().map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                quote! {
                    {
                        let mut field_output = [0u8; 32];
                        let mut field_hasher = Sha3::v256();
                        field_hasher.update(&::niz_core::prefix(stringify!(#field_ident)));
                        field_hasher.update(&self.#field_ident.hash());
                        field_hasher.finalize(&mut field_output);
                        hasher.update(&field_output);
                    }
                }
            }),
            _ => panic!("hashable can only be derived for named fields"),
        },
        _ => panic!("hashable can only be derived for structs"),
    };

    let expanded = quote! {
        impl ::niz_core::Hashable for #ident {
            fn hash(&self) -> [u8; 32] {
                use ::niz_core::tiny_keccak::{Hasher, Sha3};

                let mut ty_output = [0u8; 32];
                let mut ty_hasher = Sha3::v256();
                ty_hasher.update(stringify!(#ident).as_bytes());
                ty_hasher.finalize(&mut ty_output);

                let mut output = [0u8; 32];
                let mut hasher = Sha3::v256();
                hasher.update(&ty_output);

                #(#fields_hash_impl)*

                hasher.finalize(&mut output);
                output
            }
        }
    };

    TokenStream::from(expanded)
}

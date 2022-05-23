use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Ident, TypePath};

use crate::error::GeneratorResult;

#[derive(FromVariant)]
#[darling(attributes(py), forward_attrs(doc))]
struct EnumItem {
    ident: Ident,
    fields: Fields<Ignored>,

    #[darling(default)]
    from: Option<Ident>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(py), forward_attrs(doc))]
struct EnumArgs {
    ident: Ident,
    data: Data<EnumItem, Ignored>,

    from: TypePath,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let EnumArgs { ident, data, from } = EnumArgs::from_derive_input(&args)?;

    let e = match data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(ident, "Can only be applied to an enum.").into()),
    };

    let mut from_remote = Vec::new();
    let mut from_local = Vec::new();

    for variant in e {
        if !variant.fields.is_empty() {
            return Err(Error::new_spanned(
                &variant.ident,
                format!("Invalid enum variant {}.", variant.ident),
            )
            .into());
        }

        let item_ident = &variant.ident;
        let remote_ident = variant.from.as_ref().unwrap_or(&variant.ident);

        from_remote.push(quote! {
            #from::#remote_ident => #ident::#item_ident,
        });
        from_local.push(quote! {
            #ident::#item_ident => #from::#remote_ident,
        });
    }

    let expanded = quote! {
        impl ::std::convert::From<#from> for #ident {
            fn from(value: #from) -> #ident {
                match value {
                    #(#from_remote)*
                }
            }
        }

        impl ::std::convert::From<#ident> for #from {
            fn from(value: #ident) -> #from {
                match value {
                    #(#from_local)*
                }
            }
        }
    };

    Ok(expanded)
}

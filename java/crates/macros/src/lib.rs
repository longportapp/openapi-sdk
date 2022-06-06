use darling::FromMeta;
use inflector::Inflector;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, punctuated::Punctuated, token::Comma, Error, Expr, ExprArray, ExprLit, ExprPath,
    Lit,
};

#[derive(FromMeta, Debug, Default)]
struct FieldArgs {
    #[darling(default)]
    objarray: bool,
    #[darling(default)]
    priarray: bool,
}

#[proc_macro]
pub fn impl_java_class(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Comma>::parse_separated_nonempty;
    let mut exprs = match parser.parse(input) {
        Ok(exprs) => exprs.into_iter(),
        Err(err) => return err.to_compile_error().into(),
    };

    let classname = {
        let expr = exprs.next();
        match &expr {
            Some(Expr::Lit(ExprLit {
                lit: Lit::Str(str), ..
            })) => str.value(),
            _ => {
                return Error::new_spanned(&expr, "missing class name")
                    .to_compile_error()
                    .into();
            }
        }
    };

    let type_path = {
        let expr = exprs.next();
        match &expr {
            Some(Expr::Path(ExprPath { path, .. })) => path.clone(),
            _ => {
                return Error::new_spanned(&expr, "missing remote type")
                    .to_compile_error()
                    .into()
            }
        }
    };

    let fields = {
        let expr = exprs.next();
        let mut fields = Vec::new();

        match &expr {
            Some(Expr::Array(ExprArray { elems, .. })) => {
                for elem in elems {
                    match elem {
                        Expr::Path(ExprPath { path, attrs, .. }) => {
                            let args = attrs
                                .iter()
                                .find(|attr| attr.path.is_ident("java"))
                                .and_then(|attr| attr.parse_meta().ok())
                                .and_then(|meta| FieldArgs::from_meta(&meta).ok())
                                .unwrap_or_default();
                            fields.push((path.clone(), args));
                        }
                        _ => {
                            return Error::new_spanned(elem, "invalid field")
                                .to_compile_error()
                                .into()
                        }
                    }
                }
            }
            _ => {
                return Error::new_spanned(&expr, "missing field list")
                    .to_compile_error()
                    .into()
            }
        }

        fields
    };

    let signature = format!("L{};", classname);
    let mut set_fields = Vec::new();

    for (field, args) in &fields {
        let ident = match field.get_ident() {
            Some(ident) => ident,
            None => {
                return Error::new_spanned(&field, "invalid field")
                    .to_compile_error()
                    .into()
            }
        };
        let java_field = ident.to_string().to_camel_case();

        if args.objarray {
            set_fields.push(quote! {
                crate::types::set_field(env, obj, #java_field, crate::types::ObjectArray(#ident))?;
            });
        } else if args.priarray {
            set_fields.push(quote! {
                crate::types::set_field(env, obj, #java_field, crate::types::PrimaryArray(#ident))?;
            });
        } else {
            set_fields.push(quote! {
                crate::types::set_field(env, obj, #java_field, #ident)?;
            });
        }
    }

    let field_names = fields.iter().map(|(field, _)| field).collect::<Vec<_>>();

    let expanded = quote! {
        impl crate::types::JClassName for #type_path {
            const CLASSNAME: &'static str = #classname;
        }

        impl crate::types::JSignature for #type_path {
            fn signature() -> ::std::borrow::Cow<'static, str> {
                #signature.into()
            }
        }

        impl crate::types::IntoJValue for #type_path {
            fn into_jvalue<'a>(self, env: &jni::JNIEnv<'a>) -> jni::errors::Result<jni::objects::JValue<'a>> {
                use jni::descriptors::Desc;
                let #type_path { #(#field_names),* } = self;
                let cls: jni::objects::JClass = #classname.lookup(env)?;
                let obj = env.new_object(cls, "()V", &[])?;
                #(#set_fields)*
                Ok(obj.into())
            }
        }
    };

    expanded.into()
}

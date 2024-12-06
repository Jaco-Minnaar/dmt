use std::os::unix::ffi::OsStrExt;
use std::{env, fs};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

#[proc_macro]
pub fn migrate(input: TokenStream) -> TokenStream {
    match migrate_inner(input.into()) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn migrate_inner(input: TokenStream2) -> syn::Result<TokenStream2> {
    let args = <Punctuated<syn::LitStr, Token![,]>>::parse_terminated
        .parse(input.into())
        .expect("expected macro to be called with a comma-separated list of string literals");

    if !args.is_empty() {
        return Err(syn::Error::new(
            args.span(),
            "migrate! does not take any arguments",
        ));
    }

    let Ok(dir_path) = env::current_dir() else {
        return Ok(quote! {
            compile_error!("Could not get path to current dir to find dmt config file.");
        });
    };

    let Ok(dir) = fs::read_dir(&dir_path) else {
        let msg = format!(
            "Could not open directory {}",
            String::from_utf8_lossy(dir_path.as_os_str().as_bytes())
        );
        return Ok(quote! {
            compile_error!(#msg);
        });
    };

    let mut file_names = dir.filter_map(|e| e.ok()).map(|e| e.path()).filter(|path| {
        path.file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not get file name")
            .starts_with("dmt.config")
    });

    let Some(file_name) = file_names.next() else {
        return Ok(quote! {
            compile_error!("No DMT config file could be found.");
        });
    };

    todo!()
}

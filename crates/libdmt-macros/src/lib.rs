use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, io};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

#[proc_macro]
pub fn migrate(input: TokenStream) -> TokenStream {
    let args = <Punctuated<syn::LitStr, Token![,]>>::parse_terminated
        .parse(input)
        .expect("expected macro to be called with a comma-separated list of string literals");

    if !args.is_empty() {
        return syn::Error::new(args.span(), "migrate! does not take any arguments")
            .to_compile_error()
            .into();
    }

    migrate_inner().into()
}

fn migrate_inner() -> TokenStream2 {
    let Ok(dir_path) = env::var("CARGO_MANIFEST_DIR") else {
        return quote! {
            compile_error!("Could not get path to current dir to find dmt config file.");
        };
    };

    let mut path = PathBuf::from(dir_path);
    path.push("dmt.config.toml");

    let Ok(contents) = get_file_contents(path) else {
        return quote! {
            compile_error!("Could not read dmt config file.");
        };
    };

    quote! {
        {
            use ::libdmt::{DmtConfig as __DmtConfig, MigrationDatabase as __MigrationDb, run_migrations as __run_dmt};
            use ::std::{str::FromStr as __FromStr, convert::TryFrom as __TryFrom};

            let __dmt_config_contents = #contents;
            let __dmt_config = <__DmtConfig as __FromStr>::from_str(__dmt_config_contents).unwrap();
            let mut __dmt_db = <__MigrationDb as __TryFrom<&__DmtConfig>>::try_from(&__dmt_config).unwrap();
            __run_dmt(&mut __dmt_db, &__dmt_config.migration.migration_path).unwrap();
        }
    }
}

fn get_file_contents(path: impl AsRef<Path>) -> Result<String, io::Error> {
    let mut config_contents = String::new();

    File::open(path)?.read_to_string(&mut config_contents)?;

    Ok(config_contents)
}

#[cfg(test)]
mod test {
    use quote::quote;

    use crate::migrate_inner;

    #[test]
    fn migrate_inner_correct() {
        let config = r#"[migration]
migrationPath = "./migrationsssss"

[connection]
database = "turso"

[connection.turso]
url = "TEST_URL"
token = "TEST_TOKEN"
"#;

        let output = migrate_inner();
        let expected = quote! {
            {
                let __dmt_config_contents = #config;
                let __dmt_config = <::libdmt::DmtConfig as ::std::str::FromStr>::from_str(__dmt_config_contents);
                let mut __dmt_db = <::libdmt::MigrationDatabase as ::std::convert::TryFrom<::libdmt::DmtConfig>>::try_from(__dmt_config).unwrap();
                ::libdmt::run_migrations(__dmt_db, &__dmt_config.migration.migration_path).unwrap();
            }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}

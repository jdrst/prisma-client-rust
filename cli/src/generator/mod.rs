mod client;
mod enums;
mod header;
mod internal_enums;
mod models;
mod read_filters;

use prisma_client_rust_sdk::prelude::*;
use serde::Serialize;

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(serde::Deserialize)]
pub struct PrismaClientRustGenerator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse module_path")]
    InvalidModulePath,
}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "../src/prisma.rs";

    type Error = Error;

    fn generate(self, args: GenerateArgs) -> Result<String, Self::Error> {
        let header = header::generate(&args);

        let models = models::generate(
            &args,
            self.module_path
                .parse()
                .map_err(|_| Error::InvalidModulePath)?,
        );

        let internal_enums = internal_enums::generate(&args);
        let client = client::generate(&args);

        let read_filters_module = read_filters::generate_module(&args, quote!(super::));
        let enums = enums::generate(&args);

        let tokens = quote! {
            #header

            #(#models)*

            pub mod _prisma {
                #client
                #internal_enums
                #read_filters_module
            }

            pub use _prisma::*;

            #enums
        };

        Ok(tokens.to_string())
    }
}

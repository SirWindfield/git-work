use anyhow::Result;
use heck::CamelCase;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{fs, path::Path};

// Deserialization type.
#[derive(Debug, Deserialize)]
struct CommitType {
    pub r#type: String,
    pub desc: String,
}

// Deserialization type.
type CommitTypes = Vec<CommitType>;

fn main() -> Result<()> {
    // Read in spinner data.
    let file_content = fs::read_to_string("./types.json")?;
    let commits: CommitTypes = serde_json::from_str(&file_content)?;

    let enum_items = commits
        .iter()
        .map(|commit| {
            let r#type = format_ident!("{}", commit.r#type.to_camel_case());
            let desc = &commit.desc;

            quote! {
                #[doc=#desc]
                #r#type (CommitTypeArgs)
            }
        })
        .collect::<Vec<_>>();

    let module_to_write = quote! {
        #[derive(Clap, Debug)]
        enum SubCommand {
            /// Generate shell completion scripts for git-work.
            Completions(CompletionsArgs),

            #(#enum_items),*
        }
    };

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("subcommand.rs");
    fs::write(&dest_path, module_to_write.to_string())?;

    // Only re-run if the actual spinner data has changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=spinners.json");
    Ok(())
}

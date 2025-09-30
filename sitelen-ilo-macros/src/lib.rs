use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{ToTokens, quote};
use syn::{LitChar, LitStr};

use crate::tables::{PUNCT_TABLE, SP_TABLE};

mod parser;
mod tables;

/// Translates *sitelen Lasina* to *sitelen pona* by the UCSUR encoding.
#[proc_macro]
pub fn sp(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as LitStr);
    let lasina = input.value();
    let parse_result =
        parser::sitelen_lasina(&lasina).expect("string should parse as sitelen Lasina");

    let out_literal = LitStr::new(&parse_result.1, Span::call_site());
    out_literal.to_token_stream().into()
}

/// Translates one word of *sitelen Lasina* to a *sitelen pona* character by the UCSUR encoding.
#[proc_macro]
pub fn sp_c(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as LitStr);
    let lasina = input.value();

    let value = PUNCT_TABLE
        .get(&lasina)
        .or_else(|| SP_TABLE.get(&lasina))
        .cloned()
        .expect("string should be sitelen Lasina or punctuation");

    let out_literal = LitChar::new(value, Span::call_site());
    out_literal.to_token_stream().into()
}

use quote::{Tokens, ToTokens};
use syn::Expr;
use proc_macro2::Span;

use dissect::{Context, Fragment};
use error::Result;

struct I<'a>(&'a Expr);

pub fn expand(context: &Context) -> Result<Tokens> {
    let fragments = &context.fragments;

    Ok(quote! { {
        extern crate interp;
        interp::Interpolator::new(|w| {
            #(#fragments)*
            Ok(())
        })
    } })
}

impl<'a> ToTokens for I<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append_all(self.0.into_tokens().into_iter().map(|mut t| {
            t.span = Span::call_site();
            t
        }));
    }
}

impl ToTokens for Fragment {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Fragment::String(ref s) => {
                quote!(write!(w, #s)?;).to_tokens(tokens);
            }
            Fragment::Interpolation(ref e) => {
                let e = I(e);
                quote!(write!(w, "{}", #e)?;).to_tokens(tokens);
            }
        }
    }
}

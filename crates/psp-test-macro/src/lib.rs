extern crate proc_macro;

use proc_macro2::*;
use quote::format_ident;
use quote::quote;
use quote::quote_spanned;
use std::sync::atomic::*;

static CNT: AtomicUsize = AtomicUsize::new(0);

#[proc_macro_attribute]
pub fn test(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut attributes = Attributes::default();
    let attribute_parser = syn::meta::parser(|meta| attributes.parse(meta));

    syn::parse_macro_input!(attr with attribute_parser);
    let mut should_panic = None;
    let mut ignore = None;

    let mut body = TokenStream::from(body).into_iter().peekable();

    // Skip over other attributes to `fn #ident ...`, and extract `#ident`
    let mut leading_tokens = Vec::new();
    while let Some(token) = body.next() {
        match parse_should_panic(&mut body, &token) {
            Ok(Some((new_should_panic, span))) => {
                if should_panic.replace(new_should_panic).is_some() {
                    return compile_error(span, "duplicate `should_panic` attribute");
                }

                // If we found a `should_panic`, we should skip the `#` and `[...]`.
                // The `[...]` is skipped here, the current `#` is skipped by using `continue`.
                body.next();
                continue;
            }
            Ok(None) => (),
            Err(error) => return error,
        }

        match parse_ignore(&mut body, &token) {
            Ok(Some((new_ignore, span))) => {
                if ignore.replace(new_ignore).is_some() {
                    return compile_error(span, "duplicate `ignore` attribute");
                }

                // If we found a `new_ignore`, we should skip the `#` and `[...]`.
                // The `[...]` is skipped here, the current `#` is skipped by using `continue`.
                body.next();
                continue;
            }
            Ok(None) => (),
            Err(error) => return error,
        }

        leading_tokens.push(token.clone());
        if let TokenTree::Ident(token) = token {
            if token == "fn" {
                break;
            }
        }
    }
    let ident = find_ident(&mut body).expect("expected a function name");

    let mut tokens = Vec::<TokenTree>::new();

    let should_panic = match should_panic {
        Some(Some(lit)) => {
            quote! { ::core::option::Option::Some(::core::option::Option::Some(#lit)) }
        }
        Some(None) => quote! { ::core::option::Option::Some(::core::option::Option::None) },
        None => quote! { ::core::option::Option::None },
    };

    let ignore = match ignore {
        Some(Some(lit)) => {
            quote! { ::core::option::Option::Some(::core::option::Option::Some(#lit)) }
        }
        Some(None) => quote! { ::core::option::Option::Some(::core::option::Option::None) },
        None => quote! { ::core::option::Option::None },
    };

    let name = format_ident!("__pspt_{}_{}", ident, CNT.fetch_add(1, Ordering::SeqCst));
    let psp_test_path = attributes.psp_test_path;
    tokens.extend(quote! {
        #[#psp_test_path::linkme::distributed_slice(#psp_test_path::TESTS)]
        #[linkme(crate = #psp_test_path::linkme)]
        pub fn #name(context: &mut #psp_test_path::PspTestContext) {
            context.execute(concat!(::core::module_path!(), "::", ::core::stringify!(#ident)), #ident, #should_panic, #ignore);
        }
    });

    tokens.extend(leading_tokens);
    tokens.push(ident.into());
    tokens.extend(body);

    tokens.into_iter().collect::<TokenStream>().into()
}

fn parse_should_panic(
    body: &mut std::iter::Peekable<token_stream::IntoIter>,
    token: &TokenTree,
) -> Result<Option<(Option<Literal>, Span)>, proc_macro::TokenStream> {
    // Start by parsing the `#`
    match token {
        TokenTree::Punct(op) if op.as_char() == '#' => (),
        _ => return Ok(None),
    }

    // Parse `[...]`
    let group = match body.peek() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => group,
        _ => return Ok(None),
    };

    let mut stream = group.stream().into_iter();

    // Parse `should_panic`
    let mut span = match stream.next() {
        Some(TokenTree::Ident(token)) if token == "should_panic" => token.span(),
        _ => return Ok(None),
    };

    let should_panic = span;

    // We are interested in the `expected` attribute or string if there is any
    match stream.next() {
        // Parse the `(...)` in `#[should_panic(...)]`
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            let span = group.span();
            stream = group.stream().into_iter();

            // Parse `expected`
            match stream.next() {
                Some(TokenTree::Ident(token)) if token == "expected" => (),
                _ => {
                    return Err(compile_error(
                        span,
                        "malformed `#[should_panic(...)]` attribute",
                    ))
                }
            }

            // Parse `=`
            match stream.next() {
                Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
                _ => {
                    return Err(compile_error(
                        span,
                        "malformed `#[should_panic(...)]` attribute",
                    ))
                }
            }
        }
        // Parse `=`
        Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
        Some(token) => {
            return Err(compile_error(
                token.span(),
                "malformed `#[should_panic = \"...\"]` attribute",
            ))
        }
        None => {
            return Ok(Some((None, should_panic)));
        }
    }

    // Parse string in `#[should_panic(expected = "string")]` or `#[should_panic = "string"]`
    if let Some(TokenTree::Literal(lit)) = stream.next() {
        span = lit.span();
        let string = lit.to_string();

        // Verify it's a string.
        if string.starts_with('"') && string.ends_with('"') {
            return Ok(Some((Some(lit), should_panic)));
        }
    }

    Err(compile_error(span, "malformed `#[should_panic]` attribute"))
}

fn parse_ignore(
    body: &mut std::iter::Peekable<token_stream::IntoIter>,
    token: &TokenTree,
) -> Result<Option<(Option<Literal>, Span)>, proc_macro::TokenStream> {
    // Start by parsing the `#`
    match token {
        TokenTree::Punct(op) if op.as_char() == '#' => (),
        _ => return Ok(None),
    }

    // Parse `[...]`
    let group = match body.peek() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => group,
        _ => return Ok(None),
    };

    let mut stream = group.stream().into_iter();

    // Parse `ignore`
    let mut span = match stream.next() {
        Some(TokenTree::Ident(token)) if token == "ignore" => token.span(),
        _ => return Ok(None),
    };

    let ignore = span;

    // We are interested in the reason string if there is any
    match stream.next() {
        // Parse `=`
        Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
        Some(token) => {
            return Err(compile_error(
                token.span(),
                "malformed `#[ignore = \"...\"]` attribute",
            ))
        }
        None => {
            return Ok(Some((None, ignore)));
        }
    }

    // Parse string in `#[ignore = "string"]`
    if let Some(TokenTree::Literal(lit)) = stream.next() {
        span = lit.span();
        let string = lit.to_string();

        // Verify it's a string.
        if string.starts_with('"') && string.ends_with('"') {
            return Ok(Some((Some(lit), ignore)));
        }
    }

    Err(compile_error(span, "malformed `#[ignore]` attribute"))
}

fn find_ident(iter: &mut impl Iterator<Item = TokenTree>) -> Option<Ident> {
    match iter.next()? {
        TokenTree::Ident(i) => Some(i),
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
            find_ident(&mut g.stream().into_iter())
        }
        _ => None,
    }
}

fn compile_error(span: Span, msg: &str) -> proc_macro::TokenStream {
    quote_spanned! { span => compile_error!(#msg); }.into()
}

struct Attributes {
    psp_test_path: syn::Path,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            psp_test_path: syn::parse_quote!(::psp_test),
        }
    }
}

impl Attributes {
    fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("crate") {
            self.psp_test_path = meta.value()?.parse::<syn::Path>()?;
        } else {
            return Err(meta.error("unknown attribute"));
        }
        Ok(())
    }
}

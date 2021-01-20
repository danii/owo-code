//! No documentation right now because I cannot be bothered. I mean, *does* this
//! really need any documentation, doesn't the README.md explain the 79 lines of
//! code well enough, lol? Also hi there, hope you've been having a good day.

#![feature(decl_macro)]
use proc_macro::{Group, Ident, TokenStream, TokenTree};

/// Macro for a macro. Nice.
macro replace_keywords($upon:ident {$($owo:ident => $rust:ident),*}) {
	match $upon {
		$(
			TokenTree::Ident(ident) if ident.to_string() == stringify!($owo) =>
				TokenTree::Ident(Ident::new(stringify!($rust), ident.span())),
			TokenTree::Ident(ident) if ident.to_string() == stringify!($rust) =>
				TokenTree::Ident(Ident::new_raw(stringify!($rust), ident.span())),
		)*

		token => token
	}
}

#[proc_macro]
pub fn owo_code(input: TokenStream) -> TokenStream {
	iterate_tokens(input)
}

fn iterate_tokens(tokens: TokenStream) -> TokenStream {
	tokens.into_iter()
		.map(|token| match token {
			TokenTree::Group(group) =>
				Group::new(group.delimiter(), iterate_tokens(group.stream())).into(),
			token => parse_token(token)
		})
		.collect()
}

fn parse_token(token: TokenTree) -> TokenTree {
	replace_keywords!(token {
		Me_Irl => Self,
		ass => as,
		assync => async,
		owoit => await,
		buwueak => break,
		cowonst => const,
		cowontinue => continue,
		cock => crate,
		swow => dyn,
		yelse => else,
		enyum => enum,
		extewn => extern,
		fawse => false,
		fuwun => fn,
		yiff => if,
		fillme => impl,
		penetrate => in,
		wet => let,
		hecc => loop,
		sex => match,
		discowdmodewatow => mod,
		movemedaddy => move,
		mutt => mut,
		pubes => pub,
		wef => ref,
		yEET => return,
		me_irl => self,
		etewnyaw => static,
		stwuct => struct,
		soup => super,
		twait => trait,
		twue => true,
		tippitytap => type,
		communyism => union,
		AAAAAAA => unsafe,
		usemedaddy => use,
		howl => where,
		whine => while,
		fuwwy => hvh,
	})
}

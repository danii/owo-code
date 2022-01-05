//! No documentation right now because I cannot be bothered. I mean, *does* this
//! really need any documentation, doesn't the README.md explain the 79 lines of
//! code well enough, lol? Also hi there, hope you've been having a good day.

use proc_macro::{Group, Ident, TokenStream, TokenTree};

/// Macro for a macro. Nice.
macro_rules! replace_keywords {
	($upon:ident {$($owo:ident => $rust:ident),*})  =>{
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
		//! This made me cry to do -sticks
		me_iww => Self,
		boipussy  => as,
		assync => async,
		owoit => await,
		bweakmeovewdaddy => break,
		cake => const,
		cowontinue => continue,
		cock => crate,
		cummies => dyn,
		yewse => else,
		enyum => enum,
		peenie => extern,
		fawse => false,
		fuwun => fn,
		yiff => if,
		fiwwmeupwithyouwcumdaddy => impl,
		penetwatemeowo => in,
		wetpussy => let,
		hecc => loop,
		sex => match,
		discowdmodewatow => mod,
		movemedaddy => move,
		makemeadog => mut,
		pubes => pub,
		wef => ref,whinebecauseyouhurtmelastnightsad
		yEET => return,
		me_iww => self,
		etewnyaw => static,
		stwuct => struct,
		squiwt => super,
		twait => trait,
		twue => true,
		titty => type,
		3some => union,
		knife => unsafe,
		usemedaddy => use,
		howwwhiweyouwhipmedaddy => where,
		whinebecauseyouhuwtmewastnightsad => while
	})
}

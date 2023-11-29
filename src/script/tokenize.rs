use std::iter::zip;

use super::parse::{Token, HoldStr, HoldUsize};


pub fn tokenizer<'i>(input: &'i str) -> Vec<Token<'i>> {
	let cs: Vec<char> = input.chars().collect::<Vec<_>>();
	let cks = cs
		.into_iter()
		.map(|c| {
			let k: TokenKind = c.try_into().unwrap();
			(k, c)
		})
		.collect::<Vec<_>>();
	
	(&cks).group_by(|(sk,s) ,(dk,d)| {
		sk == dk
	})
	.into_iter()
	.filter_map(|cks| {
		let (ks, cs): (Vec<_>, String) = cks.into_iter().map(|d| *d).unzip();
		ks.first().map(move |k| kind_to_token(*k, cs))
	})
	.collect()
	
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
	BraceL,
	BraceR,
	BracketL,
	BracketR,
	ParenL,
	ParenR,
	QuoteDouble,
	QuoteSingle,
	Period,
	Comma,
	Colon,
	Semicolon,
	Linebreak,
	Octothorpe,
	Hyphen,
	Equal,
	Tilda,
	LessThan,
	QuestionMark,
	
	Tab,
	AsciiSpace,
	AsciiAlpha,
	AsciiDigit,
	Alpha,
	Space,
	
	Else,
}

impl TryFrom<char> for TokenKind {
	type Error = String;
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			' ' => Ok(Self::AsciiSpace),
			'[' => Ok(Self::BracketL),
			']'=> Ok(Self::BracketR),
			'(' => Ok(Self::ParenL),
			')' => Ok(Self::ParenR),
			'{' => Ok(Self::BraceL),
			'}' => Ok(Self::BraceR),
			'.' => Ok(Self::Period),
			',' => Ok(Self::Comma),
			':' => Ok(Self::Colon),
			';' => Ok(Self::Semicolon),
			'\t' => Ok(Self::Tab),
			'\n' => Ok(Self::Linebreak),
			'\'' => Ok(Self::QuoteSingle),
			'"' => Ok(Self::QuoteDouble),
			'#' => Ok(Self::Octothorpe),
			'-' => Ok(Self::Hyphen),
			'=' => Ok(Self::Equal),
			'~' => Ok(Self::Tilda),
			'<' => Ok(Self::LessThan),
			'?' => Ok(Self::QuestionMark),
			
			c =>
				if false {
					Err("never".to_string())
				}
				else if c.is_ascii_digit() { Ok(Self::AsciiDigit) }
				else if c.is_ascii_alphabetic() { Ok(Self::AsciiAlpha) }
				else if c.is_alphabetic() { Ok(Self::Alpha) }
				else if c.is_whitespace() { Ok(Self::Space)}
				else { Ok(Self::Else) }
		}
	}
}

fn kind_to_token<'i>(
	k: TokenKind,
	s: String
) -> Token<'i> {
	match k {
		TokenKind::AsciiDigit => Token::HoldStr(HoldStr::Numeric(&s)),
		TokenKind::AsciiAlpha => Token::HoldStr(HoldStr::AsciiAlpha(&s)),
		TokenKind::Alpha => Token::HoldStr(HoldStr::NonAsciiNonWhite(&s)),
		TokenKind::AsciiSpace => Token::HoldUsize(HoldUsize::Spaces(s.len())),
		_ => Token::Else
	}
}
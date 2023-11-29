use std::iter::Peekable;

use cstree::{
	build::{GreenNodeBuilder, NodeCache},
	green::GreenNode,
	syntax::{ResolvedNode, SyntaxNode},
};

use crate::reveal;

use super::tokenize::{kind_to_token, tokenizerstring, TokenKind};

// use super::tokenize::tokenizer;

pub type BuilderFinish<'i> = (GreenNode, Option<NodeCache<'i>>);

pub fn parse_cozo<'i>(root: SyntaxCozo, input: &'i str) -> BuilderFinish<'i> {
	let kindstring = tokenizerstring(input);
	reveal!(kindstring);
	let (tree, cache) = parse_cozo_inner(kindstring, root);
	(tree, cache)
}

pub fn parse_cozo_inner<'i>(
	kindstring: Vec<(TokenKind, String)>,
	root: SyntaxCozo,
) -> BuilderFinish<'i> {
	let sl = kindstring
		.as_slice()
		.into_iter()
		.map(|(k, s)| kind_to_token(*k, s.as_str()))
		.collect::<Vec<_>>();
	let mut parser = CSTreeParser::new(sl.as_slice());
	if let Err(e) = parser.parse(root) {
		println!("parse error: {}", e);
	}
	parser.builder.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, cstree::Syntax)]
#[repr(u32)]
enum Bracket {
	L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, cstree::Syntax)]
#[repr(u32)]
pub enum SyntaxCozo {
	// Token
	#[static_text("[")]
	BracketL,
	#[static_text("]")]
	BracketR,
	#[static_text("{")]
	BraceL,
	#[static_text("}")]
	BraceR,
	#[static_text("(")]
	ParenL,
	#[static_text(")")]
	ParenR,
	#[static_text("'")]
	QuoteSingle,
	#[static_text("\"")]
	QuoteDouble,
	#[static_text(",")]
	Comma,
	#[static_text("<-")]
	ArrowLeft,
	#[static_text("=")]
	Equal,
	#[static_text("?")]
	QuestionMark,
	#[static_text(" ")]
	Space,

	Numeric,
	AsciiAlpha,
	NonAsciiNonWhite,

	// Node

	//DataValue
	Bool,
	Json,
	List,
	Str,

	Var,
	VarDelimitedBySpaces,
	CommaSepareted,
	InBrace,
	InBracket,
	InParen,

	DataValue,
	DataValueDelimitedBySpaces,
	DataValueMidop,
	Func,

	Rule,
	HornClause,
	HornClauseMidop,
	HornClausePreop,
	Expr,
	Sentence,
	Script,
}

impl ToString for SyntaxCozo {
	fn to_string(&self) -> String {
		match self {
			// Token
			Self::BracketL => "BracketL",
			Self::BracketR => "BracketR",
			Self::BraceL => "BraceL",
			Self::BraceR => "BraceR",
			Self::ParenL => "ParenL",
			Self::ParenR => "ParenR",
			Self::QuoteSingle => "QuoteSingle",
			Self::QuoteDouble => "QuoteDouble",
			Self::Comma => "Comma",
			Self::ArrowLeft => "ArrowLeft",
			Self::Equal => "Equal",
			Self::QuestionMark => "QuestionMark",
			Self::Space => "Space",
			Self::Numeric => "Numeric",
			Self::AsciiAlpha => "AsciiAlpha",
			Self::NonAsciiNonWhite => "NonAsciiNonWhite",
			Self::Bool => "Bool",
			Self::Json => "Json",
			Self::List => "List",
			Self::Str => "Str",
			Self::Var => "Var",
			Self::VarDelimitedBySpaces => "VarDelimitedBySpaces",
			Self::CommaSepareted => "CommaSepareted",
			Self::InBrace => "InBrace",
			Self::InBracket => "InBracket",
			Self::InParen => "InParen",
			Self::DataValue => "DataValue",
			Self::DataValueDelimitedBySpaces => "DataValueDelimitedBySpaces",
			Self::DataValueMidop => "DataValueMidop",
			Self::Func => "Func",
			Self::Rule => "Rule",
			Self::HornClause => "HornClause",
			Self::HornClauseMidop => "HornClauseMidop",
			Self::HornClausePreop => "HornClausePreop",
			Self::Expr => "Expr",
			Self::Sentence => "Sentence",
			Self::Script => "Script",
			_ => "UNKNOWN",
		}
		.to_string()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataExpr<'i> {
	Bool(bool),
	Bot,
	Bytes,
	Json(&'i str),
	List(&'i [DataExpr<'i>]),
	Null,
	Num(&'i str),
	Regex(&'i str),
	Str(&'i str),
	Uuid,
	Validity(&'i str),
	Vec(&'i str),
	Set,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<'i> {
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
	HoldStr(HoldStr<'i>),
	HoldUsize(HoldUsize),
	Else,
}

impl<'i> Token<'i> {
	fn hold_str(&self) -> Option<HoldStr<'i>> {
		match self {
			Self::HoldStr(h) => Some(*h),
			_ => None,
		}
	}

	fn hold_u8(&self) -> Option<HoldUsize> {
		match self {
			Self::HoldUsize(h) => Some(*h),
			_ => None,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoldStr<'i> {
	Numeric(&'i str),
	AsciiAlpha(&'i str),
	NonAsciiNonWhite(&'i str),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoldUsize {
	Spaces(usize),
	Tabs(usize),
}

impl<'i> HoldStr<'i> {
	fn unwrap(&self) -> &'i str {
		match self {
			HoldStr::Numeric(s) => s,
			HoldStr::NonAsciiNonWhite(s) => s,
			HoldStr::AsciiAlpha(s) => s,
		}
	}

	fn is_var_tail(&self) -> bool {
		match self {
			HoldStr::Numeric(_) => true,
			HoldStr::AsciiAlpha(_) => true,
			HoldStr::NonAsciiNonWhite(_) => true,
		}
	}

	fn is_var_head(&self) -> bool {
		match self {
			HoldStr::Numeric(_) => false,
			HoldStr::AsciiAlpha(_) => true,
			HoldStr::NonAsciiNonWhite(_) => true,
		}
	}
}

impl HoldUsize {
	fn unwrap(&self) -> usize {
		match self {
			HoldUsize::Spaces(u) => *u,
			HoldUsize::Tabs(u) => *u,
		}
	}
}

trait TokenIter<'i> = Iterator<Item = Token<'i>>;

pub struct CSTreeParser<'i, 'a> {
	lexer: Peekable<std::slice::Iter<'a, Token<'i>>>,
	// &'i[Token<'i>],
	builder: GreenNodeBuilder<'static, 'static, SyntaxCozo>,
}

impl<'i, 'a> CSTreeParser<'i, 'a> {
	pub fn new(tokens: &'a [Token<'i>]) -> Self {
		Self {
			builder: GreenNodeBuilder::new(),
			lexer: tokens.iter().peekable(),
		}
	}

	fn next_ascii_spaces(&mut self) -> Option<&'a Token<'i>> {
		self.lexer.next_if(|&t| match *t {
			Token::HoldUsize(HoldUsize::Spaces(_)) => true,
			_ => false,
		})
	}

	fn skip_ascii_spaces(&mut self) {
		if self.next_ascii_spaces().is_some() {
			self.builder.static_token(SyntaxCozo::Space);
		}
	}

	fn delimited_ascii_spaces0(&mut self, kind: SyntaxCozo) -> Result<(), String> {
		self.skip_ascii_spaces();
		self.parse(kind)?;
		self.skip_ascii_spaces();
		Ok(())
	}

	fn separeted_by_comma(&mut self, item: SyntaxCozo, endsyntax: SyntaxCozo) -> Result<(), String> {
		loop {
			match self.parse(item) {
				Ok(_) => {
					if self.lexer.next_if(|t| **t == Token::Comma).is_some() {
						self.builder.static_token(SyntaxCozo::Comma);
					} else {
						match self.parse(endsyntax) {
							Ok(_) => {
								return Ok(());
							}
							Err(e) => {}
						}
					}
				}
				Err(e) => {
					return match self.parse(endsyntax) {
						Ok(_) => Ok(()),
						Err(e) => Err(e + ": separeted_by_comma failed to match endsyntax"),
					};
				}
			}
		}
	}

	pub fn parse(&mut self, root: SyntaxCozo) -> Result<(), String> {
		if let Some(nextoken) = self.lexer.peek() {
			match root {
				SyntaxCozo::InBrace => match *nextoken {
					Token::BraceL => {
						self.builder.start_node(root);
						self.lexer.next();
						self.separeted_by_comma(SyntaxCozo::VarDelimitedBySpaces, SyntaxCozo::BraceR)?;
						self.builder.finish_node();
					}
					_ => return Err("missing BraceL({)".to_string()),
				},

				SyntaxCozo::InBracket => match *nextoken {
					Token::BracketL => {
						self.builder.start_node(root);
						self.lexer.next();
						self
							.separeted_by_comma(SyntaxCozo::DataValueDelimitedBySpaces, SyntaxCozo::BracketR)?;
						self.builder.finish_node();
					}
					_ => return Err("missing BraceL([)".to_string()),
				},

				SyntaxCozo::InParen => match *nextoken {
					Token::ParenL => {
						self.builder.start_node(root);
						self.lexer.next();
						self.separeted_by_comma(SyntaxCozo::DataValueDelimitedBySpaces, SyntaxCozo::ParenR)?;
						self.builder.finish_node();
					}
					_ => return Err("missing ParenL(()".to_string()),
				},

				SyntaxCozo::VarDelimitedBySpaces => match *nextoken {
					Token::HoldUsize(HoldUsize::Spaces(i)) => {
						self.builder.start_node(root);
						self.lexer.next();
						self.parse(SyntaxCozo::VarDelimitedBySpaces)?;
					}

					Token::Linebreak => {
						self.builder.start_node(root);
						self.lexer.next();
						self.parse(SyntaxCozo::VarDelimitedBySpaces)?;
					}

					_ => {
						return Err("missing space or linebreak".to_string());
					}
				},

				SyntaxCozo::Var => {
					let heads = [SyntaxCozo::AsciiAlpha, SyntaxCozo::NonAsciiNonWhite];

					let tails = [
						SyntaxCozo::Numeric,
						SyntaxCozo::AsciiAlpha,
						SyntaxCozo::NonAsciiNonWhite,
					];

					let mut any = false;
					let ch = self.builder.checkpoint();
					for s in heads {
						if let Err(e) = self.parse(s) {
							continue;
						} else {
							any = true;
							self.builder.start_node_at(ch, root);
							loop {
								let errs = tails.into_iter().map_while(|s| self.parse(s).err()).count();
								if errs == tails.len() {
									break;
								}
							}
						}
					}
					if !any {
						return Err("no valid chars for Var".to_string());
					}

					// if heads.map(|x| self.parse(x)).into_iter().any(|r| r.is_ok()) {
					// 	self.builder.start_node(root);
					// 	loop {
					// 		if tails.map(|x| self.parse(x)).into_iter().any(|r| r.is_ok()) {
					// 		} else {
					// 			break;
					// 		}
					// 	}
					// } else {
					// 	return Err("var starts with AsciiAlpha/NonAsciiNonWhite".to_string());
					// }
				}

				SyntaxCozo::AsciiAlpha => {
					let e = Err("not AsciiAlpha".to_string());
					if let Some(h) = nextoken.hold_str() {
						match h {
							HoldStr::AsciiAlpha(s) => {
								self.builder.start_node(root);
								self.lexer.next();
								self.builder.token(root, s);
							}
							_ => {
								return e;
							}
						}
					} else {
						return e;
					}
				}

				SyntaxCozo::NonAsciiNonWhite => {
					let e = Err("not NonAsciiNonWhite".to_string());
					if let Some(h) = nextoken.hold_str() {
						match h {
							HoldStr::NonAsciiNonWhite(s) => {
								self.builder.start_node(root);
								self.lexer.next();
								self.builder.token(root, s);
							}
							_ => {
								return e;
							}
						}
					} else {
						return e;
					}
				}

				SyntaxCozo::Numeric => {
					let e = Err("not Numeric".to_string());
					if let Some(h) = nextoken.hold_str() {
						match h {
							HoldStr::Numeric(s) => {
								self.builder.start_node(root);
								self.lexer.next();
								self.builder.token(root, s);
							}
							_ => {
								return e;
							}
						}
					} else {
						return e;
					}
				}

				_else => return Err("yet implemented".to_string()),
			}

			// 	match *nextoken {
			// 	Token::HoldStr(HoldStr::AsciiAlpha(s)) => {
			// 		self.lexer.next();

			// 		// let mut atleastone = false;
			// 		// let mut var = String::new();
			// 		// loop {
			// 		// if let Some(&t) = self.lexer.next_if(|t| match **t {
			// 		// 	Token::HoldStr(ho) => true,
			// 		// 	_ => false
			// 		// }) {
			// 		// 	match t {
			// 		// 		Token::HoldStr(ho) => {
			// 		// 			var += ho.unwrap();
			// 		// 			atleastone = true;
			// 		// 		}
			// 		// 		,_ => {}
			// 		// 	}
			// 		// } else {
			// 		// 	 break;
			// 		// }}
			// 		// if atleastone {
			// 		// 	self.builder.token(SyntaxCozo::Var, &var);
			// 		// 	self.
			// 		// }

			// 	}
			// }

			self.builder.finish_node();
			Ok(())
		} else {
			Err("input end".to_string())
		}
	}
}

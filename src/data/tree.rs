use std::iter::repeat;

use cstree::{
	green::{GreenNode, GreenToken},
	interning::Resolver,
	syntax::{ResolvedNode, SyntaxNode},
	util::NodeOrToken,
	RawSyntaxKind, Syntax,
};
use display_tree::DisplayTree;
use pretty::RcDoc;

use crate::script::parse::{parse_cozo, SyntaxCozo};

pub struct GreenTokenTree<'r, 'g, I, S> {
	token: &'g GreenToken,
	resolver: &'r I,
	s: S,
}

pub struct GreenNodeTree<'r, 'g, I, S> {
	pub node: &'g GreenNode,
	pub s: S,
	pub resolver: &'r I,
	depth: usize,
}


impl<'r, 'g, I, S> GreenNodeTree<'r, 'g, I, S>
where
	I: Resolver,
	S: Syntax + ToString,
{
	pub fn new(
		node: &'g GreenNode,
		resolver: &'r I,
		s: S,
	)-> Self {
		Self { node, resolver, s , depth: 0}
	}
}

impl<'r, 'g, I, S> ToString for GreenTokenTree<'r, 'g, I, S>
where
	I: Resolver,
	S: Syntax + ToString,
{
	fn to_string(&self) -> String {
		let kind = S::from_raw(self.token.kind());
		let mut s  = kind.to_string();
		if let Some(t) = self.token.text(self.resolver) {
			format!("`{t}`")
		} else {
			String::new()
		}
	}
}

impl<'r, 'g, I, S> DisplayTree for GreenNodeTree<'r, 'g, I, S>
where
	S: Syntax + ToString,
	I: Resolver,
{
	fn fmt(&self, f: &mut std::fmt::Formatter, style: display_tree::Style) -> std::fmt::Result {
		let syntax = S::from_raw(self.node.kind());
		write!(f, "{}\n", syntax.to_string()).expect("syntax fmt failed");
		
		self.node.children().for_each(|c| {
			let hor = 
			repeat(style.char_set.horizontal)
				.take(style.indentation as usize)
				.collect::<String>();
			let indent: String = repeat(style.char_set.vertical.to_string()+
				repeat(" ").take(style.indentation as usize).collect::<String>().as_str()
			).take(self.depth).collect();
			write!(f, "{}{}{}", indent, style.char_set.connector, hor);
			match c {
			NodeOrToken::Node(d) => {
				GreenNodeTree {
					node: d,
					s: self.s,
					resolver: self.resolver,
					depth: self.depth+1
				}
				.fmt(f, style)
				.expect("GreenNode info fmt err");
			}
			NodeOrToken::Token(d) => {
				write!( f, "{}\n" , GreenTokenTree {
						resolver: self.resolver,
						token: d,
						s: self.s,
					}.to_string()
				)
				.expect("indentation+render connector failed");
				
			}
		}});

		Ok(())
	}
}

pub struct GreenNodePretty(GreenNode);

impl GreenNodePretty {
	pub fn to_doc<S>(&self) -> RcDoc<()>
	where
		S: Syntax + ToString,
	{
		let k = S::from_raw(self.0.kind());
		RcDoc::text(k.to_string())
	}

	// fn tree_inner<S,I>(&self, resolver: I)
	// -> RcDoc
	// where S: Syntax + ToString,
	// 	I: Resolver
	// {
	// 	self.0.children()
	// 		.map(|g| {
	// 			let s = S::from_raw(g.kind());
	// 			match g {
	// 				cstree::util::NodeOrToken::Node(d) => {
	// 					let doc = GreenNodePretty(*d).tree_inner(resolver);

	// 				},
	// 				NodeOrToken::Token(d) => {
	// 					d.text(resolver)
	// 				}
	// 			}
	// 		})
	// }
}

// pub fn parse_cozo_tree<'i>(
// 	root: SyntaxCozo,
// 	input: &'i str,
// )
// {
// 	let (green, cache) = parse_cozo(root, input);
// 	let interner = cache.unwrap().into_interner().unwrap();
// 	let resolved: ResolvedNode<SyntaxCozo> = SyntaxNode::new_root_with_resolver(green, interner);

// }

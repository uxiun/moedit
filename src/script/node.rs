use cstree::{
	green::GreenNode,
	syntax::{ResolvedNode, SyntaxNode},
	Syntax,
};
use display_tree::AsTree;

use crate::data::tree::{GreenNodeTree, GreenNodePretty};

use super::parse::{parse_cozo, SyntaxCozo, BuilderFinish};

pub fn parse_cozo_tree<'i>(root: SyntaxCozo, input: &'i str) {
	let (green, cache) = parse_cozo(root, input);
	let interner = cache.unwrap().into_interner().unwrap();
	let resolved: ResolvedNode<SyntaxCozo> = SyntaxNode::new_root_with_resolver(green, interner);
}

fn tree_inner(green: GreenNode) {
	let raw = green.kind();
	let s = SyntaxCozo::from_raw(raw);
}

pub fn parse_cozo_and_print<'i>(
	root: SyntaxCozo,
	input: &'i str
)-> GreenNode
{
	
	let (tree, cache) = parse_cozo(root, input);
	if let Some(cache) = cache {
		
		let resolver = cache.into_interner().unwrap();
		let binding = GreenNodeTree::new(
			&tree,
			&resolver,
			root,
		);
		let astree = AsTree::new(&binding);
	
		println!("{}", astree);
	}
	
	tree
}
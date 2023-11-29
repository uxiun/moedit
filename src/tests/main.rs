use crate::{
	reveal,
	util::func::{partition_toggle, partition_toggle_tuple},
};

mod parsetest {
	use cstree::Syntax;
	use display_tree::AsTree;

	use crate::{
		data::tree::GreenNodeTree,
		reveal,
		script::{parse::{parse_cozo, SyntaxCozo}, node::parse_cozo_and_print},
	};


	
	#[test]
	fn varmany() {
		[
			"あいう4hello",
			"world_is",
			"あ、の",
			"44dd",
		]
			.into_iter()
			.for_each(|input| {
				let g = parse_cozo_and_print(SyntaxCozo::Var, input);
			})
	}
}

#[test]
fn fswatch() {
	crate::io::file::main();
}

#[test]
fn nonascii() {
	let t = " hello, 世界world";
	let p = partition_toggle(t.chars(), &mut |c| c.is_ascii());

	// let mut ac = p.array_chunks().map(|[s,d]| [s, d]).collect::<Vec<_>>();
	// let (ac, rem) = p.as_chunks();
	// let acv = ac.into_iter().map(|[s,d]| (s, d)).collect::<Vec<_>>();
	// reveal!(acv);
	// reveal!(rem);

	let pt = partition_toggle_tuple(t.chars(), &mut |c| c.is_ascii());
	reveal!(pt);
}

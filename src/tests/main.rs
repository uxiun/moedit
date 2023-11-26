use crate::{
	reveal,
	util::func::{partition_toggle, partition_toggle_tuple},
};

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

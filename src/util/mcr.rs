use std::{
	collections::{HashMap, HashSet},
	hash::Hash,
};

// use itertools::Itertools;

#[macro_export]
macro_rules! show {
	($x:expr) => {
		println!("▶ {} ↓", stringify!($x));
		println!("{:#?}", $x);
		println!("——————————")
	};
}

#[macro_export]
macro_rules! showshort {
	($x:expr) => {
		println!("▶ {} ↓", stringify!($x));
		println!("{:?}", $x);
		println!("——————————")
	};
}

#[macro_export]
macro_rules! reveal {
	($val:expr $(,)?) => {
		dbg!(&$val);
	};
}

pub fn give_set_item_index<I, T>(set: I) -> HashMap<<I as Iterator>::Item, usize>
where
	I: Iterator,
	<I as Iterator>::Item: Hash + Eq,
{
	let mut h = HashMap::new();
	set.enumerate().fold(h, |mut h, (i, x)| {
		h.insert(x, i);
		h
	})
}

// pub fn fill_empty_key_hashmap<K, V>(
// 	default: V,
// 	all_key: Vec<K>,
// 	hm: &HashMap<K, V>,
// ) -> HashMap<K, V>
// where
// 	K: Eq + Hash + Clone,
// 	V: Clone, // V: Copy
// 	          // HashMap<K,V>: Extend<&K,&V>
// {
// 	// h.extend(hm.into_iter());
// 	HashMap::from_iter(
// 		[
// 			all_key
// 				.into_iter()
// 				.filter(|k| !hm.keys().contains(k))
// 				.map(|k| (k, &default))
// 				.collect::<Vec<_>>(),
// 			hm.into_iter().map(|(k, v)| (k.clone(), v)).collect(),
// 		]
// 		.into_iter()
// 		.flatten()
// 		.map(|(k, v)| (k, v.clone())), // hm.into_iter().fold(HashMap::new(), |h, (k,v)| {
// 		                               // 	if
// 		                               // })
// 	)
// }

// pub fn unions_hashmap<K, V>(hs: &[HashMap<K, V>]) -> HashMap<&K, &V>
// where
// 	K: Hash + Eq,
// {
// 	// HashMap::from_iter()
// 	hs.into_iter().flat_map(|h| h.into_iter()).collect()
// }

// pub fn hashmap_deep_map<F, K, J, V, W>(
// 	h: &HashMap<K, HashMap<J, V>>,
// 	f: F,
// ) -> HashMap<K, HashMap<J, W>>
// where
// 	F: Fn(&K, &J, &V) -> W,
// 	J: Eq + Hash + Clone,
// 	K: Eq + Hash + Clone,
// {
// 	h.into_iter()
// 		.map(|(k, row)| {
// 			(
// 				k.clone(),
// 				row
// 					.into_iter()
// 					.map(|(j, v)| (j.clone(), f(k, j, v)))
// 					.collect(),
// 			)
// 		})
// 		.collect()
// }

// pub fn id<T>(t: T) -> T {
// 	t
// }

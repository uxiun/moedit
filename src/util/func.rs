// pub fn map_toowned<I,T>(v: I)
// where
// 	I: IntoIterator<Item = T>,
// 	T: Copy
// {
// 	v.into_iter().map(|t| *t)
// }

use std::fmt::Debug;

pub fn split_partition<I, F, T>(v: I, sep: &mut F) -> Vec<Vec<T>>
where
	I: IntoIterator<Item = T>,
	F: FnMut(&T) -> bool,
{
	partition_toggle(v, sep)
		.into_iter()
		.enumerate()
		.filter_map(|(i, k)| if i % 2 == 0 { Some(k) } else { None })
		.collect()
}

/// apply f: Item -> bool, return partitioned vec which item's item result = false, true, false...
pub fn partition_toggle<I, F, T>(v: I, f: &mut F) -> Vec<Vec<T>>
where
	I: IntoIterator<Item = T>,
	F: FnMut(&T) -> bool,
{
	let mut rs = v.into_iter().map(|t| if f(&t) { Ok(t) } else { Err(t) });

	let mut last_result_isok: Option<bool> = None;

	let mut vvec = vec![];
	let mut vec = vec![];
	while let Some(r) = rs.next() {
		let risok = r.is_ok();
		if last_result_isok.map(|isok| isok == risok).unwrap_or(true) {
			if vec.len() > 0 || risok && last_result_isok == None {
				vvec.push(vec);
			}
			vec = vec![result_unwrap(r)];
			last_result_isok = Some(risok);
		} else {
			vec.push(result_unwrap(r));
		}
	}
	if vec.len() > 0 {
		vvec.push(vec);
	}
	vvec

	// while ilen > 0 {
	// 	let errs = rs.take_while(|r| r.is_err()).filter_map(|r| r.err()).collect();

	// 	vec.push(errs.collect());

	// 	let oks = rs.take_while(|r| r.is_ok()).filter_map(|r| r.ok());
	// 	vec.push(oks.collect());
	// }
	// vec
}

/// apply f: Item -> bool, return partitioned vec which item's item result = false, true, false...
pub fn partition_toggle_tuple<I, F, T>(v: I, f: &mut F) -> Vec<(Vec<T>, Vec<T>)>
where
	I: IntoIterator<Item = T>,
	F: FnMut(&T) -> bool,
{
	let mut rs = v.into_iter().map(|t| if f(&t) { Ok(t) } else { Err(t) });

	let mut last_result_isok: Option<bool> = None;

	let mut vvec = vec![];
	let mut tup = (vec![], vec![]);
	while let Some(r) = rs.next() {
		let risok = r.is_ok();
		if !risok && last_result_isok.map(|d| d != risok).unwrap_or(false) {
			vvec.push(tup);
			tup = (vec![], vec![]);
		}
		if risok {
			tup.1.push(result_unwrap(r));
		} else {
			tup.0.push(result_unwrap(r));
		}
		last_result_isok = Some(risok);
	}

	if [tup.0.iter(), tup.1.iter()].into_iter().flatten().count() > 0 {
		vvec.push(tup);
	}

	vvec

	// while ilen > 0 {
	// 	let errs = rs.take_while(|r| r.is_err()).filter_map(|r| r.err()).collect();

	// 	vec.push(errs.collect());

	// 	let oks = rs.take_while(|r| r.is_ok()).filter_map(|r| r.ok());
	// 	vec.push(oks.collect());
	// }
	// vec
}

pub fn result_unwrap<T>(r: Result<T, T>) -> T {
	match r {
		Err(d) => d,
		Ok(d) => d,
	}
}

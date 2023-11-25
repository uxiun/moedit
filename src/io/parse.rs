use std::{
	fs::File,
	io::{BufRead, BufReader},
	str::pattern::Pattern,
};

fn linechunks<'a>(sepline: &'a str, b: BufReader<File>) -> Vec<Vec<String>> {
	let (mut v, cv) = b.lines().fold((vec![], vec![]), |(mut v, mut cv), line| {
		if let Ok(l) = line {
			if &l == sepline {
				v.push(cv);
				(v, vec![])
			} else {
				cv.push(l);
				(v, cv)
			}
		} else {
			(v, cv)
		}
	});

	v.push(cv);
	v
}

/// 空行を区切りとする塊に分ける
pub fn blocks(b: BufReader<File>) -> Vec<Vec<String>> {
	let ls = linechunks("", b);
	ls.into_iter().filter(|v| v.len() > 0).collect()
}

pub fn active_inactive_blocks<'a>(
	active_block_marker_suffix: &'a str,
	b: BufReader<File>,
) -> (Vec<String>, Vec<String>) {
	let (actives, inactives): (Vec<_>, Vec<_>) = blocks(b).into_iter().partition(|ls| {
		ls.last()
			.map(|s| active_block_marker_suffix.is_suffix_of(&s))
			.unwrap_or(false)
	});

	(
		actives
			.into_iter()
			.map(|ls| {
				let mut s = ls
					.into_iter()
					.fold(String::new(), |s, l| s + l.as_str() + "\n");
				s.pop();
				s.strip_suffix(active_block_marker_suffix)
					.map(|s| s.to_string())
					.unwrap_or(s)
			})
			.collect(),
		inactives
			.into_iter()
			.map(|ls| {
				let mut s = ls
					.into_iter()
					.fold(String::new(), |s, l| s + l.as_str() + "\n");
				s.pop();
				s
			})
			.collect(),
	)
}

// pub fn blocksgo(
// 	b: BufReader<File>,
// 	// blocks: Vec<u8>,
// )-> Vec<u8>
// {
// 	let mut buf = vec![];

// 	if let Ok(len) = b.buffer().read_until(b'\n\n', &mut buf) {

// 	}
// }

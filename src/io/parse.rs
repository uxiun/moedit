use std::{
	fs::File,
	io::{BufRead, BufReader},
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
fn blocks(b: BufReader<File>) -> Vec<Vec<String>> {
	let ls = linechunks("", b);
	ls.into_iter().filter(|v| v.len() > 0).collect()
}

pub fn active_blocks<'a>(active_block_marker_suffix: &'a str, b: BufReader<File>) -> Vec<String> {
	blocks(b)
		.into_iter()
		.filter_map(|ls| {
			// let mut s =
			ls.into_iter()
				.fold(String::new(), |s, l| s + l.as_str())
				.strip_suffix(active_block_marker_suffix)
				.map(|stripped| stripped.to_string())
		})
		.collect()
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

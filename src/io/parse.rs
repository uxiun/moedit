use std::{
	collections::{HashMap, HashSet},
	fs::File,
	io::{BufRead, BufReader},
	str::pattern::Pattern,
};

use crate::{
	io::file::{default_replaceallmap, default_replacetomap},
	reveal,
	util::func::{partition_toggle_tuple, split_partition},
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

pub fn blocks_emptyline2(b: BufReader<File>) -> Vec<Vec<String>> {
	blocks(b)
		.split(|lines| lines.len() == 0)
		.map(|lss| lss.into_iter().flatten().map(|s| s.clone()).collect())
		.collect()
}

pub fn active_inactive_blocks<'a, 's, 'j>(
	active_block_marker_suffix: &'a str,
	b: BufReader<File>,
	ascii_quote_len: usize,
	replacetomap: &'j ReplaceToMap<'s>,
	replaceallmap: &'j ReplaceToMap<'s>,
) -> (Vec<String>, Vec<String>) {
	let (actives, inactives): (Vec<_>, Vec<_>) = blocks_emptyline2(b).into_iter().partition(|ls| {
		ls.last()
			.map(|s| active_block_marker_suffix.is_suffix_of(&s))
			.unwrap_or(false)
	});

	(
		actives
			.into_iter()
			.map(|ls| {
				let mut s = magic_format_cozo_script(
					ls.into_iter()
						.intersperse("\n".to_string())
						.collect::<String>()
						.as_str(),
					ascii_quote_len,
					replacetomap,
					replaceallmap,
				);

				// .into_iter()
				// .fold(String::new(), |s, l| s + l.as_str() + "\n");
				// s.pop();

				s.strip_suffix(active_block_marker_suffix)
					.map(|s| s.to_string())
					.unwrap_or(s)
			})
			.collect(),
		inactives
			.into_iter()
			.map(|ls| {
				// magic_format_cozo_script(ls.into_iter().collect::<String>().as_str(), ascii_quote_len)
				ls.into_iter().intersperse("\n".to_string()).collect()
			})
			.collect(),
	)
}

fn magic_format_cozo_script<'a, 's, 'j>(
	lines: &'a str,
	ascii_quote_len: usize,
	replacetomap: &'j ReplaceToMap<'s>,
	replaceallmap: &'j ReplaceToMap<'s>,
) -> String {
	let mut quote_split = lines.split("\"").map(|s| s.split("'"));

	// let strings =

	let mut is_system_ops = false;

	quote_split
		.enumerate()
		.map(|(i, s)| {
			if i % 2 == 0 {
				s.enumerate()
					.map(|(j, s)| {
						if j % 2 == 0 {
							let replaced = replace_map_all(s, replaceallmap);

							if is_system_ops {
								replaced
							} else {
								let (is_system, s) = auto_quote(&replaced, ascii_quote_len, replacetomap);

								if j == 0 {
									is_system_ops = is_system;
								}
								s
							}
						} else {
							format!("'{}'", s.to_string())
						}
					})
					.collect::<String>()
			} else {
				let s: String = s
					.enumerate()
					.map(|(j, s)| {
						if j % 2 == 0 {
							s.to_string()
						} else {
							format!("'{s}'")
						}
					})
					.collect();
				format!("\"{s}\"")
			}
		})
		.collect()
	// .fold(String::new(), |s, d| s + d.as_str())
}

pub type ReplaceToMap<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn find_strip_prefix<'a, 's, 'd: 'a, 'k: 'a, 'j, I>(
	strs: I,
	tostrip: &'k str,
	replacetomap: &'j ReplaceToMap<'s>,
) -> Option<(&'a str, &'a str)>
where
	I: IntoIterator<Item = &'d str>,
{
	strs.into_iter().find_map(|key| {
		replacetomap
			.get(key)
			.map(|set| {
				set
					.into_iter()
					.find_map(|s| tostrip.strip_prefix(s).map(|stripped| (key, stripped)))
			})
			.flatten()
			.or(tostrip.strip_prefix(key).map(|stripped| (key, stripped)))
	})
}

fn replace_map_all<'a, 'j, 's>(s: &'a str, replacetomap: &'j ReplaceToMap<'s>) -> String {
	replacetomap
		.into_iter()
		.fold(s.to_string(), |s, (to, set)| {
			set.into_iter().fold(s, |s, from| s.replace(from, to))
		})
}

fn find_strip_prefixtest() {
	let replacetomap = &default_replacetomap();
	let ascstr = "<- [
		[";

	let res = find_strip_prefix(["{", "[", "<-"], ascstr, replacetomap);
	reveal!(res);
	assert!(res.is_some());
}

pub fn auto_quote<'a, 's, 'j>(
	s: &'a str,
	ascii_quote_len: usize,
	replacetomap: &'j ReplaceToMap<'s>,
) -> (bool, String) {
	// println!("auto_quote({})", s);

	let pt = partition_toggle_tuple(s.chars(), &mut |c| c.is_ascii());

	let mut is_system_ops = false;

	let (bools, ss): (Vec<_>, Vec<_>) = pt
		.into_iter()
		.map(|(non, ascii)| {
			// reveal!(non);
			// reveal!(ascii);

			// let open_separated: Vec<Vec<_>> = ascii.into_iter().fold(String::new(), |s, c| s+ c).split('{')
			// 	.map(|s| s.split('[').collect() )
			// 	.collect();

			let raw: String = [non.iter(), ascii.iter()].into_iter().flatten().collect();

			if is_system_ops {
				(true, raw)
			} else {
				let nons = non.iter().collect::<Vec<_>>();

				let mut asc = ascii.iter();

				let leftwhites_asc = asc
					.take_while(|c| c.is_whitespace() && ("\n".chars().all(|k| k != **c)))
					.collect::<Vec<_>>();

				let s = if leftwhites_asc.len() == 0 {
					let mut wordsuffix_asc = ascii
						.iter()
						.take_while(|c| c.is_ascii_alphanumeric() || ("_".contains(|x| x == **c)))
						.collect::<Vec<_>>();

					// reveal!(wordsuffix_asc);

					let (consumed, suffixright) = ascii.split_at(wordsuffix_asc.len());

					let mut afterword_whites = suffixright
						.into_iter()
						.take_while(|c| c.is_ascii_whitespace() && (**c != '\n'))
						.collect::<Vec<_>>();

					let (_, ascs) = ascii.split_at(wordsuffix_asc.len() + afterword_whites.len());

					if let Some(ascc) = ascs.into_iter().next() {
						let ascstr = ascs.into_iter().collect::<String>();

						// println!("ascstr=");
						// println!("{ascstr}");

						if "::".is_prefix_of(ascstr.as_str().trim_start()) {
							(true, raw)
						} else {
							(
								false,
								if let Some((mark, stripped)) =
									find_strip_prefix(["{", "[", "<-"], &ascstr, replacetomap)
								{
									let mut ascsiter = ascs.into_iter();
									ascsiter.next();
									let s: String = vec![nons, wordsuffix_asc, afterword_whites]
										.into_iter()
										.flatten()
										.collect();

									// println!("{s}++{mark}++");

									let (_, st) = auto_quote(stripped, ascii_quote_len, replacetomap);
									s + mark + st.as_str()
								} else if let Some((mark, stripped)) =
									find_strip_prefix([":"], &ascstr, replacetomap)
								{
									let comma_i = stripped
										.chars()
										.enumerate()
										.find(|(_, c)| ",\n".chars().any(|e| e == *c))
										.map(|(i, c)| i + 1) // charsを基準にfindしたので+1
										.unwrap_or(stripped.len());

									let (raw, rem) = stripped.split_at(comma_i);

									// println!("++{mark}++{raw}++");

									let rrem = if rem.len() > 0 { &rem[1..] } else { rem };
									let (_, st) = auto_quote(rrem, ascii_quote_len, replacetomap);

									[nons, wordsuffix_asc, afterword_whites]
										.into_iter()
										.flatten()
										.collect::<String>() + mark
										+ raw + st.as_str()
								} else {
									let (nonasc, mut ascrem): (String, String) = if nons.len() > 0 {
										let nonasc_word: String = format!(
											"\"{}\"",
											[nons, wordsuffix_asc]
												.into_iter()
												.flatten()
												.collect::<String>() // .fold(String::new(), |s, c| format!("{s}{c}"))
										);

										afterword_whites.extend(ascs.iter());
										(nonasc_word, afterword_whites.into_iter().collect())
									} else {
										(String::new(), ascii.into_iter().collect())
									};

									// reveal!(nonasc);
									// reveal!(ascrem);

									// let ascrems = *reveal!(
									// 	auto_quote_ascii(&ascrem, ascii_quote_len).as_str()
									// );

									nonasc + ascrem.as_str()
								},
							)
						}
					} else {
						(
							false,
							if nons.len() > 0 {
								let nonasc_word: String = format!(
									"\"{}\"",
									[nons, wordsuffix_asc]
										.into_iter()
										.flatten()
										.collect::<String>() // .fold(String::new(), |s, c| format!("{s}{c}"))
								);
								nonasc_word
							} else {
								if wordsuffix_asc
									.iter()
									.next()
									.map(|c| c.is_ascii_alphabetic())
									.unwrap_or(false) && wordsuffix_asc.len() >= ascii_quote_len
								{
									format!(
										"\"{}\"",
										wordsuffix_asc.into_iter().collect::<String>() // .fold(String::new(), |s, c| format!("{s}{c}"))
									)
								} else {
									raw
								}
							},
						)
					}
				} else {
					let (_, ascs) = ascii.split_at(leftwhites_asc.len());
					let nonsstr = nons.iter().map(|c| *c).collect::<String>();
					let leftwhites_string: String = leftwhites_asc.into_iter().collect();

					if let Some(ascc) = ascs.into_iter().next() {
						let ascstr = ascs.into_iter().collect::<String>();

						// println!("ascstr=");
						// println!("{ascstr}");

						if ascstr.strip_prefix("::").is_some() {
							(true, raw)
						} else {
							(
								false,
								if let Some((mark, stripped)) =
									find_strip_prefix(["{", "[", "<-"], &ascstr, replacetomap)
								{
									let mut ascsiter = ascs.into_iter();
									ascsiter.next();
									let s: String = nons.into_iter().collect();

									// println!("{s}++{mark}++");

									let (_, st) = auto_quote(stripped, ascii_quote_len, replacetomap);

									s + leftwhites_string.as_str() + mark + st.as_str()
								} else if let Some((mark, stripped)) =
									find_strip_prefix([":"], &ascstr, replacetomap)
								{
									let comma_i = stripped
										.chars()
										.enumerate()
										.find(|(_, c)| ",\n".chars().any(|e| e == *c))
										.map(|(i, c)| i + 1)
										.unwrap_or(stripped.len());

									let (raw, rem) = stripped.split_at(comma_i);

									// println!("++{mark}++{raw}++");

									let (_, st) = auto_quote(rem, ascii_quote_len, replacetomap);
									nonsstr + leftwhites_string.as_str() + mark + raw + st.as_str()
								} else {
									let nonasc = if nons.len() > 0 {
										let nonasc_word: String = format!("\"{}\"", nonsstr);

										nonasc_word
									} else {
										String::new()
									};

									// reveal!(nonasc);

									// let ascrems = *reveal!(
									// 	auto_quote_ascii(&ascrem, ascii_quote_len).as_str()
									// );

									nonasc + leftwhites_string.as_str() + ascstr.as_str()
								},
							)
						}
					} else {
						(
							false,
							if nons.len() > 0 && !is_system_ops {
								let nonasc_word: String = format!("\"{}\"", nonsstr);
								nonasc_word + leftwhites_string.as_str()
							} else {
								raw
							},
						)
					}
				};

				if s.0 {
					is_system_ops = true;
				}

				s
			}
		})
		.unzip();

	// (bools.into_iter().next().unwrap_or(false), ss.concat())

	// println!("{:?}", &bools);

	(bools.into_iter().any(|d| d), ss.concat())

	// .fold(String::new(), |s, f| s + f.as_str())
}

fn auto_quote_ascii<'a, 's>(asciis: &'s Vec<&'a char>, ascii_quote_len: usize) -> String {
	split_partition(asciis, &mut |c| "\n".contains(***c))
		.into_iter()
		.map(|mut line| {
			// let mut pt =

			let mut toggled = partition_toggle_tuple(line, &mut |c| c.is_whitespace());

			// let mut sss = vec![];

			toggled
				.into_iter()
				.map(|(black, white)| {
					let mut toggled = partition_toggle_tuple(black, &mut |c| c.is_ascii_alphanumeric());

					// let bl: String = ac.map(|[other, alphanumeric]| {
					// let mut ss = vec![];

					let s: String = toggled
						.into_iter()
						.map(|(other, alphanumeric)| {
							let al: String = alphanumeric.iter().map(|c| ***c).collect();
							let so: String = other.into_iter().map(|c| **c).collect();
							let s = so
								+ if alphanumeric.len() > ascii_quote_len
									&& alphanumeric
										.iter()
										.next()
										.map(|head| head.is_ascii_alphabetic())
										.unwrap_or(false)
								{
									format!("\"{}\"", al)
								} else {
									al
								}
								.as_str();
							s
						})
						.collect();

					let wh: String = white.into_iter().map(|c| **c).collect();

					s + wh.as_str()
				})
				.collect()

			// sss.concat()
			// 	+ ac
			// 		.remainder()
			// 		.into_iter()
			// 		.map(|v| v.into_iter().map(|c| ***c).collect())
			// 		.collect::<Vec<String>>()
			// 		.concat()
			// 		.as_str()
		})
		.intersperse("\n".to_string())
		.collect::<Vec<_>>()
		.concat()
}

#[test]
fn quote_escape_test() {
	let t = "
	r1[] <- [[1, 'a'], [2, 'b']]
	r2[] . [[2, 'B'], [3, 'C']]

	?[l1, l2] :- r1[a, l1],
					 r2[b, l2]";

	let d = magic_format_cozo_script(t, 3, &default_replacetomap(), &default_replaceallmap());

	println!("{d}");
}

#[test]
fn auto_quote_test() {
	let text = "?[lang, word] := *挨拶[lang, こんにちは]";
	let tex = "
	:create 挨拶2 {
		lang: String,
		=>
		言葉: String
	}
	挨拶2 <- [
		[日本語, こんにちは ],
		[\"en\", \"hello\"],
	];
	";

	let opt = "
	::columns 挨拶
	;";

	let h = default_replacetomap();
	let s = auto_quote(opt, 3, &h);

	reveal!(s);
	// assert_eq!(text, s.as_str());
}

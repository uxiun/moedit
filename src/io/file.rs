use cozo::{new_cozo_sqlite, Db, NamedRows, ScriptMutability, Storage};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use std::{
	borrow::Cow,
	collections::{BTreeMap, HashMap, HashSet},
	fs::{File, OpenOptions},
	io::{BufReader, BufWriter, Bytes, Write},
	iter::zip,
	path::{Path, PathBuf},
	str::from_utf8,
	time::Duration,
};

use crate::{data::table::NamedRowsWrap, reveal};

use super::parse::{active_inactive_blocks, blocks, ReplaceToMap};

pub fn main() {}

pub fn run_cozopath_watchpath() {
	let mut args = std::env::args();

	let dbpath = args
		.nth(1)
		.expect("Argument 1 (cozo db path) needs to be a path");

	let path = args
		.nth(0)
		.expect("Argument 2 (watch path) needs to be a path");

	println!("Watching {path}");

	if let Err(error) = watch_trigger_with_db(path, dbpath) {
		println!("Error: {error:?}");
	}
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
	let (tx, rx) = std::sync::mpsc::channel();

	// Automatically select the best implementation for your platform.
	// You can also access each implementation directly e.g. INotifyWatcher.
	let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

	// Add a path to be watched. All files and directories at that path and
	// below will be monitored for changes.
	watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

	for res in rx {
		match res {
			Ok(event) => println!("Change: {event:?}"),
			Err(error) => println!("Error: {error:?}"),
		}
	}

	Ok(())
}

fn watch_trigger_with_db<P: AsRef<Path>>(path: P, dbpath: P) -> notify::Result<()> {
	let db = new_cozo_sqlite(dbpath).expect("won't fail create sqlite storage");

	let (tx, rx) = std::sync::mpsc::channel();

	let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;

	debouncer
		.watcher()
		.watch(path.as_ref(), RecursiveMode::Recursive)?;

	let mut count = 0;
	let mut last_executed_count = 0;
	let mut lastables: Vec<String> = vec![];

	for res in rx {
		match res {
			Ok(event) => {
				println!("watch[{count}] event[{}]", &event.len());
				count += 1;
				let scriptables = run_cozo_script_blocks(&db, event);

				// if zip(lastables.iter(), tables.iter()).any(|(a, b)| a != b) {
				// 	last_executed_count = count;
				// 	lastables = vec![];

				// 	tables.into_iter().for_each(|t| {
				// 		println!("{}", t.as_str());
				// 		lastables.push(t);
				// 	})

				// }

				let (scripts, tables): (Vec<_>, Vec<_>) = scriptables.into_iter().unzip();

				scripts.iter().for_each(|s| {
					println!(">>> {s}");
				});

				tables.into_iter().enumerate().for_each(|(i, t)| {
					let mut ls = scripts[i].lines();
					println!(
						"↓ {}{}",
						ls.next().unwrap_or(""),
						if ls.next().is_some() { " ..." } else { "" }
					);
					println!("{}", t);
				})
			}
			Err(error) => println!("Error: {error:?}"),
		}
	}

	Ok(())
}

type ScannedFileMap = HashMap<PathBuf, (Vec<String>, Vec<String>)>;

pub fn default_replacetomap<'a>() -> ReplaceToMap<'a> {
	HashMap::from_iter(
		[("{", vec!["《"]), ("}", vec!["》"])]
			.into_iter()
			.map(|(k, v)| (k, v.into_iter().collect())),
	)
}

pub fn default_replaceallmap<'a>() -> ReplaceToMap<'a> {
	HashMap::from_iter(
		[
			(":=", vec![":-"]),
			("<-", vec![".=", "←"]),
			(" <- ", vec![" . "]),
			("?[", vec!["？["]),
		]
		.into_iter()
		.map(|(k, v)| (k, v.into_iter().collect())),
	)
}

fn run_cozo_script_blocks<'a>(
	dbref: &'a Db<impl Storage<'a>>,
	events: Vec<DebouncedEvent>,
) -> Vec<(String, String)> {
	let suffix = ";";
	let ascii_quote_len = 3;
	let exedb = true;
	let replacetomap = default_replacetomap();

	let filenames = get_filenames(events);
	let mut scannedfiles = vec![];
	let mut scannedmap: ScannedFileMap = HashMap::new();

	let namedrows: Vec<(String, NamedRows)> = filenames
		.into_iter()
		.map(|path| {
			reveal!(path);
			readfile(&path)
				.map(|b| {
					let bs = active_inactive_blocks(
						suffix,
						b,
						ascii_quote_len,
						&replacetomap,
						&default_replaceallmap(),
					);

					if bs.0.len() > 0 {
						scannedfiles.push(path.clone());
						scannedmap.insert(path, bs.clone());
					}

					bs.0.into_iter().filter_map(|block| {
						// println!(">>> {block}");

						if exedb {
							dbref
								.run_script(&block, BTreeMap::new(), ScriptMutability::Mutable)
								.ok()
								.map(|x| (block, x))
						} else {
							None
						}
					})
				})
				.into_iter()
				.flatten()
		})
		.flatten()
		.collect();

	namedrows
		.into_iter()
		.map(|(script, row)| (script, NamedRowsWrap(row).into()))
		.collect()

	// reveal!(scannedmap);

	// 最後の文字が一番下の塊の最後にくっついてくる
	// scannedfile_inactivate( scannedmap);
}

fn get_filenames<'a>(events: Vec<DebouncedEvent>) -> HashSet<PathBuf> {
	let mut hs = HashSet::new();
	let mut is = vec![];

	events.iter().enumerate().for_each(|(i, e)| {
		let s = e.path.as_path().to_string_lossy();
		if !hs.contains(&s) {
			hs.insert(s);
			is.push(i);
		}
	});

	HashSet::from_iter(events.into_iter().enumerate().filter_map(|(i, e)| {
		if is.contains(&i) {
			Some(e.path)
		} else {
			None
		}
	}))
}

fn readfile<P: AsRef<Path>>(path: P) -> std::io::Result<BufReader<File>> {
	let f = File::open(path)?;
	let b = std::io::BufReader::new(f);
	Ok(b)
}

fn scannedfile_inactivate<'a>(scannedmap: ScannedFileMap) -> std::io::Result<()> {
	scannedmap
		.into_iter()
		.for_each(|(path, (active, mut inactive))| {
			inactive.extend(
				// active
				// 	.into_iter()
				// 	.map(|s| s.strip_suffix(suffix).map(|s| s.to_string()).unwrap_or(s)),
				active.into_iter(),
			);

			let u = inactive
				.into_iter()
				.map(|s| s.as_bytes().to_owned())
				.intersperse_with(|| vec![b'\n', b'\n'])
				.flatten()
				.collect::<Vec<u8>>();

			reveal!(from_utf8(&u));

			let f = OpenOptions::new()
				.write(true)
				.open(path)
				.expect("failed to open file");
			let mut b = BufWriter::new(f);
			if let Err(e) = b.write_all(&u) {
				println!("write error: {e}");
			}
			b.flush();
		});

	Ok(())
}

fn remove_active_suffix<'a, P: AsRef<Path>>(suffix: &'a str, path: P) -> std::io::Result<()> {
	let f = File::open(&path)?;
	let reader = BufReader::new(f);
	let towrite = blocks(reader)
		.into_iter()
		.map(|ls| {
			let mut s = ls
				.into_iter()
				.fold(String::new(), |s, l| s + l.as_str() + "\n");
			s.pop();
			s.strip_suffix(suffix)
				.map(|stripped| stripped.to_string())
				.unwrap_or(s)
			// .as_bytes()
		})
		.fold(vec![], |v, s| {
			[v, s.into_bytes(), vec![b'\n']]
				.into_iter()
				.flatten()
				.collect()
		});

	let f = OpenOptions::new().write(true).open(path)?;
	let mut b = BufWriter::new(f);
	b.write_all(&towrite)?;
	b.flush()?;

	Ok(())
}

use cozo::{new_cozo_sqlite, Db, NamedRows, ScriptMutability, Storage};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use std::{
	collections::{BTreeMap, HashSet},
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
	time::Duration,
};

use super::parse::active_blocks;

pub fn main() {}

pub fn run_cozopath_watchpath() {
	let mut args = std::env::args().into_iter();

	let dbpath = args
		.next()
		.expect("Argument 1 (cozo db path) needs to be a path");

	let path = args
		.next()
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

	for res in rx {
		match res {
			Ok(event) => run_cozo_script_blocks(&db, event),
			Err(error) => println!("Error: {error:?}"),
		}
	}

	Ok(())
}

fn run_cozo_script_blocks<'a>(dbref: &'a Db<impl Storage<'a>>, events: Vec<DebouncedEvent>) {
	let filenames = get_filenames(events);
	let namedrows: Vec<NamedRows> = filenames
		.into_iter()
		.map(|path| {
			readfile(path)
				.map(|b| {
					active_blocks(";", b).into_iter().filter_map(|block| {
						dbref
							.run_script("", BTreeMap::new(), ScriptMutability::Mutable)
							.ok()
					})
				})
				.into_iter()
				.flatten()
		})
		.flatten()
		.collect();

	for row in namedrows {
		println!("{:?}", row);
	}
}

fn get_filenames(events: Vec<DebouncedEvent>) -> HashSet<PathBuf> {
	let mut filenames = HashSet::from_iter(events.into_iter().map(|e| e.path));
	filenames
}

fn readfile<P: AsRef<Path>>(path: P) -> std::io::Result<BufReader<File>> {
	let f = File::open(path)?;
	let b = std::io::BufReader::new(f);
	Ok(b)
}

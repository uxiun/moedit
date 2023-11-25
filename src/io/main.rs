use std::io::{stdin, stdout, Write};

use termion::{clear, cursor, raw::IntoRawMode, screen::AlternateScreen};

fn readlines_until<'a>(stoppers: &'a Vec<&'a str>) -> (Option<&'a str>, Vec<String>) {
	let mut v = vec![];
	// let stop = readlines_until_go(stoppers, &mut v);
	let mut stop = None;
	loop {
		let mut s = String::new();
		if let Ok(u) = stdin().read_line(&mut s) {
			if let Some(found) = stoppers.into_iter().find(|d| *d == &s) {
				stop = Some(*found);
				break;
			} else {
				v.push(s);
			}
		} else {
			break;
		}
	}
	(stop, v)
}

// fn readlines_until_go<'a,'s>(
// 	stoppers:  impl IntoIterator<Item = &'a str>,
// 	v: &'s mut Vec<String>
// )-> Option<&'a str>
// {
// 	let mut s = String::new();
// 	if let Ok(u) = stdin().read_line(&mut s) {
// 		if let Some(stop) = (&stoppers).into_iter().find(|d| *d == &s) {
// 			Some(stop)
// 		} else {
// 			v.push(s);
// 			readlines_until_go(stoppers, v)
// 		}
// 	} else {
// 		None
// 	}

// }

fn usetermion() {

	// let stdin = stdin();
	// let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
	// write!(stdout, "{}", clear::All);
	// write!(stdout, "{}", cursor::Goto(1,1));
	// write!(stdout, "Hello, world!");

	// stdout.flush().unwrap();

	// let mut handle = stdin.lock();
}

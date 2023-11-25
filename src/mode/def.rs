use std::{collections::HashMap, fmt::format};

use termion::event::Key;

use crate::mode::key::key_to_string;

use super::key::ModifierKey;

pub struct Modef<T> {
	pub insert: ModeMap<T>,
	pub other: HashMap<String, ModeMap<T>>,
}

type ModeMap<T> = HashMap<String, Opes<T>>;

struct KeysForInsertMode {
	head: ModifierKey,
	keys: Vec<Key>,
}

struct NonemptyKeys {
	head: Key,
	keys: Vec<Key>,
}

struct InsertOpes<T> {
	chain: Vec<Ope<T>>,
	commit: CommitOpe<T>,
}

pub struct Opes<T> {
	chain: Vec<Ope<T>>,
	commit: CommitOpe<T>,
}

enum Ope<T> {
	NotInsertMode(String),
	Cursor(CursorPosition),
	Scroll(LineCount),
	Mut(Mutation<T>),
}

enum CommitOpe<T> {
	ToggleInsertMode,
	Ope(Ope<T>),
}

#[derive(Debug, Clone, Copy)]
enum LineCount {
	Relative(i32),
	Absolute(u32),
}

type CursorPosition = usize;

enum Mutation<T> {
	Del(usize, usize),
	Insert(usize, T),
	Complex(Vec<Mutation<T>>),
}

impl ToString for NonemptyKeys {
	fn to_string(&self) -> String {
		let mut s = format!(
			"{}{}",
			key_to_string(self.head),
			self
				.keys
				.iter()
				.fold(String::new(), |s, d| format!("{}{} ", s, key_to_string(*d)))
		);
		s.pop();
		s
	}
}

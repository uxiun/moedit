use termion::event::Key;

#[derive(Debug, Clone, Copy, Hash)]
pub enum ModifierKey {
	Ctrl(char),
	Alt(char),
	Arrow(ArrowKey),
	PageUp,
	PageDown,
	Insert,
	Home,
	Delete,
	Backspace,
	Tab,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum ArrowKey {
	Up,
	Down,
	Left,
	Right,
}

impl ToString for ModifierKey {
	fn to_string(&self) -> String {
		match self {
			ModifierKey::Alt(c) => key_to_string(Key::Alt(*c)),
			ModifierKey::Ctrl(c) => key_to_string(Key::Ctrl(*c)),
			ModifierKey::Arrow(a) => a.to_string(),
			ModifierKey::Backspace => "backspace".to_string(),
			ModifierKey::Delete => "delete".to_string(),
			ModifierKey::Insert => "insert".to_string(),
			ModifierKey::Home => "home".to_string(),
			ModifierKey::Tab => "tab".to_string(),
			ModifierKey::PageDown => "pagedown".to_string(),
			ModifierKey::PageUp => "pageup".to_string(),
		}
	}
}

impl ToString for ArrowKey {
	fn to_string(&self) -> String {
		match self {
			ArrowKey::Down => "down",
			ArrowKey::Left => "left",
			ArrowKey::Right => "right",
			ArrowKey::Up => "up",
		}
		.to_string()
	}
}

pub fn key_to_string(key: Key) -> String {
	match key {
		Key::Alt(c) => format!("alt+{}", c),
		Key::Ctrl(c) => format!("ctrl+{}", c),
		Key::F(u) => format!("f{}", u),
		Key::Char(c) => c.to_string(),
		Key::BackTab => "backtab".to_string(),
		Key::Backspace => "backspace".to_string(),
		Key::Delete => "delete".to_string(),
		Key::Down => "down".to_string(),
		Key::End => "end".to_string(),
		Key::Esc => "esc".to_string(),
		Key::Home => "home".to_string(),
		Key::Insert => "insert".to_string(),
		Key::Left => "left".to_string(),
		Key::Null => "null".to_string(),
		Key::PageDown => "pagedown".to_string(),
		Key::PageUp => "pageup".to_string(),
		Key::Right => "right".to_string(),
		Key::Up => "up".to_string(),
		Key::__IsNotComplete => "__IsNotComplete".to_string(),
	}
}

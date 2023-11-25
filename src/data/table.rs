use std::str::from_utf8;

use cozo::{DataValue, NamedRows, Validity};
use tabled::{builder::Builder, Tabled};

#[derive(Debug, Tabled)]
struct DatavalueWrap(DataValue);

impl Into<String> for DatavalueWrap {
	fn into(self) -> String {
		match self.0 {
			DataValue::Bool(d) => d.to_string(),
			DataValue::Bot => "Bot".to_string(),
			DataValue::Bytes(d) => format!("Bytes({})", from_utf8(&d).unwrap_or("from_utf8 error")),
			DataValue::Json(d) => "Json".to_string(),
			DataValue::List(d) => format!("[{}]", values_tostring(d)),
			DataValue::Null => "Null".to_string(),
			DataValue::Num(d) => d.to_string(),
			DataValue::Regex(d) => format!("/{}/", d.0),
			DataValue::Str(d) => format!("\"{}\"", d.to_string()),
			DataValue::Uuid(d) => format!("Uuid({})", d.0),
			DataValue::Validity(d) => ValidityWrap(d).into(),
			DataValue::Vec(d) => VectorWrap(d).into(),
			DataValue::Set(d) => {
				let v: Vec<DataValue> = d.into_iter().collect();
				format!("{{{}}}", values_tostring(v))
			}
		}
	}
}

fn values_tostring(values: Vec<DataValue>) -> String {
	format!(
		"{}",
		values
			.into_iter()
			.fold(String::new(), |s, d| s + d.to_string().as_str())
	)
}

struct ValidityWrap(Validity);

impl Into<String> for ValidityWrap {
	fn into(self) -> String {
		format!(
			"Validity {{
	is_assert: {}
	timestamp: {}
}}",
			self.0.is_assert.0, self.0.timestamp.0 .0
		)
	}
}

struct VectorWrap(cozo::Vector);

impl Into<String> for VectorWrap {
	fn into(self) -> String {
		let mut s = match self.0 {
			cozo::Vector::F32(d) => d
				.into_iter()
				.fold(String::new(), |s, d| s + d.to_string().as_str() + "\n  "),
			cozo::Vector::F64(d) => d
				.into_iter()
				.fold(String::new(), |s, d| s + d.to_string().as_str() + "\n  "),
		};
		let (s, _) = s.split_at(s.len() - 3);
		format!("< {}>", s)
	}
}

#[derive(Debug)]
pub struct NamedRowsWrap(pub NamedRows);

impl Into<String> for NamedRowsWrap {
	fn into(self) -> String {
		let mut builder = Builder::default();
		builder.set_header(self.0.headers);

		self.0.rows.into_iter().for_each(|data| {
			builder.push_record(data.into_iter().map(|d| d.to_string()));
		});

		let table = builder.build();
		table.to_string()
	}
}

// pub fn pretty_datavalues(i: Vec<DataValue>, indent: u8)-> String
// {
// 	i.into_iter().fold(String::new(), |s, d| {
// 		s + "\n" + "  ".repeat(indent as usize).as_str()
// 		+
// 	})
// }

// impl<'a,D,> pretty::Pretty<'a, D> for DatavalueWrap {
// 	fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {

// 	}
// }

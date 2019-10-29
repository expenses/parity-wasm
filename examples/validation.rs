extern crate parity_wasm;

use parity_wasm::elements::{Validator, ValueType, Error};
use std::env;

struct NoFloatValidator;

impl Validator for NoFloatValidator {
	fn validate_value(&self, value: ValueType) -> Result<(), Error> {
		match value {
			ValueType::F32 | ValueType::F64 => Err(Error::Other("No f32/f64s allowed!")),
			_ => Ok(())
		}
	}
}

fn main() {
	let args = env::args().collect::<Vec<_>>();
	if args.len() != 2 {
		println!("Usage: {} <wasm file>", args[0]);
		return;
	}

	if let Err(error) = parity_wasm::deserialize_file(&args[1], &NoFloatValidator) {
		println!("Error: {}", error);
	} else {
		println!("File cleared validation");
	}
}

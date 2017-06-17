use std::io::Read;
use std::io;

fn main () {
	
}

fn parse_pactl_info<R: Read>(mut buffer: R) -> Result<String, io::Error> {
	let mut string = String::new();
	let result = buffer.read_to_string(&mut string);

	match result {
		Err(e) => Err(e),
		Ok(_) => Ok(string)
	}
}

#[cfg(test)]
mod test {
	use parse_pactl_info;
	#[test]
	fn test () {
		const LONG_STRING: &'static str = include_str!("samples/pactl_info.txt");
		println!("{:?}", parse_pactl_info(LONG_STRING.as_bytes()));
	}
}

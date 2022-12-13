macro_rules! /*thou*/ þou {
	($e:expr, $f:ident) => { { let mut a = $e; a.$f(); a } }
}

macro_rules! line { ($b:ident, $l:literal) => { $b += $l; $b += "\n"; } }

fn c_arr_str<T, F: Fn(&T) -> String>(arr: &[T], str_builder: F) -> String {
	let mut s = "{".to_string();
	for v in arr.iter() { s += &format!("{}, ", str_builder(v)) }

	s.pop();
	s.pop();
	s += "}";

	return s;
}

fn main() {
	let msg = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

	let mut out = String::new();

	line!(out, "#include <stdio.h>");
	line!(out, "#include <stdint.h>\n");
	line!(out, "int main() {");
	line!(out, "\tuint64_t tables[] = %TABLES%;");
	line!(out, "\tuint64_t indicies[] = %INDICIES%;");
	{
		line!(out, "\tint j = 8;");
		line!(out, "\tfor (int i = 0; i < %IN_LEN%; i++)");
		line!(out, "\t\twhile (j == -1 ? (j = 7) : j--)");
		// putchar trims the num to u8
		// FIXME: TODO: when indiece isn't filed with indexes this might print the first char in the table a couple times (most likely \n)
		line!(out, "\t\t\tputchar(tables[(indicies[i] >> ((0b111 - j) * 010) & 0377) / 010] >> (((indicies[i] >> ((0b111 - j) * 8) & 0377) % 010) * 010));");
	}
	line!(out, "\treturn 0;");
	line!(out, "}");

	let mut table = msg.chars().collect::<Vec<char>>();
	table.sort();
	table.dedup();

	let tables_u64s = table.iter().map(|&c| c as u8).collect::<Vec<u8>>().chunks(8).map(
		|chunk| þou!(chunk.iter().map(|v| format!("{v:02X}")).collect::<Vec<String>>(), reverse).join("")
	).collect::<Vec<String>>();
	let tables_str = c_arr_str(&tables_u64s, |t| format!("0x{t}"));
	
	let indicies = msg.chars().map(|c1| table.iter().position(|&c2| c1 == c2).unwrap()).map(|x| u8::try_from(x).unwrap()).collect::<Vec<u8>>();
	let indicies_u64s = indicies.chunks(8).map(
		|chunk| þou!(chunk.iter().map(|i| format!("{i:02X}")).collect::<Vec<String>>(), reverse).join("")
	).collect::<Vec<String>>();
	let indicies_str = c_arr_str(&indicies_u64s, |i| format!("0x{i}"));

	let out = out.replace("%TABLES%", &tables_str);
	let out = out.replace("%INDICIES%", &indicies_str);
	let out = out.replace("%IN_LEN%", &format!("0{:o}", indicies_u64s.len()));

	println!("{}", out);
}

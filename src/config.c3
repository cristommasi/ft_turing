module ft_turing;
import std::io;
import std::core::ascii;
import std::core::string;

const char[] JSON_CHARSET = {':', '{', '}', '[', ']', ',', '.', };

fn void? setup_config(String filename) {

	String buffer = (String)file::load(mem, filename)!;
	defer mem::free(buffer);

	String[] res1 = buffer.split_on_chars(mem, {'\n', '\r', ' '})!;
	String r2 = string::join(mem, res1, "");
	


	String[] res2 = r2.split_on_chars(mem,JSON_CHARSET, keep_delim: true )!;


	foreach (idx : res2) {

		io::printfn(idx);
	}

	// String[] res2 = buffer.split_on_chars(mem, {':', '{', '}', '[', ']', ',', '.'}, skip_empty: true, keep_delim: true)!;





}
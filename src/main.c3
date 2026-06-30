module ft_turing;
import std::io;
import std::core::ascii;




fn int main(String[] args)
{
	if (args.len < 2) {
		io::printfn("ERROR: Wrong amount of arguments.");
		return 1;
	}
	if (catch err = setup_config(args[1])) {
		io::printfn("ERROR: %s", err);
		return 1;
	}


	

	return 0;
}

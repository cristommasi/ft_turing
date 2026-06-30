module ft_turing::utils;

<*
 Split a string into parts, optionally keeping the delimiter characters as
 separate elements.
 e.g "a|b|c" split with "|" and keep_delim = true yields { "a", "|", "b", "|", "c" }
 e.g "a|b|c" split with "|" and keep_delim = false yields { "a", "b", "c" }
 @param [in] self
 @param [in] delimiters
 @param max : "Max number of split points, 0 means no limit, defaults to 0"
 @param skip_empty : "True to skip empty (non-delimiter) elements"
 @param keep_delim : "True to keep the delimiter characters as separate elements"
 @param [&inout] allocator : "The allocator to use for the String[]"
 @require delimiters.len > 0 : "The delimiter must be at least 1 character long"
 @ensure return.len > 0 || skip_empty
*>
fn String[]? String.split_on_chars(self, Allocator allocator, char[] delimiters, sz max = 0, bool skip_empty = false, bool keep_delim = false)
{
    if (delimiters.len < 1) return NOT_FOUND~;
    sz capacity = 16;
    sz i = 0;
    String* holder = alloc::alloc_array(allocator, String, capacity);
    sz splits = 0;
    bool no_more = false;
    while (!no_more)
    {
        sz? index = (max != 0 && splits == max) ? NOT_FOUND~ : self.index_of_chars(delimiters);
        String res @noinit;
        String delim @noinit;
        bool had_delim = false;
        if (try index)
        {
            res = self[:index];
            delim = self[index..index];
            self = self[index + 1..];
            had_delim = true;
            splits++;
        }
        else
        {
            res = self;
            no_more = true;
        }
        if (res.len || !skip_empty)
        {
            if (i == capacity)
            {
                capacity *= 2;
                holder = alloc::realloc(allocator, holder, String::size * capacity);
            }
            holder[i++] = res;
        }
        if (had_delim && keep_delim)
        {
            if (i == capacity)
            {
                capacity *= 2;
                holder = alloc::realloc(allocator, holder, String::size * capacity);
            }
            holder[i++] = delim;
        }
    }
    return holder[:i];
}
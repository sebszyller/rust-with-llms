# Overview of interactions with ChatGPT

These are not *super detailed* unless it calls for it.
Provided text is not verbatim because that would be too long, and no screenshots either unless needed.

It's divided into three parts:
1. Part I:
2. Part II (Coming soon!)
3. Part III (Coming soon!)

## Part I

1.1. **Question:** How to select from and run multiple `main` files in rust?

**Answer:** ✅ Use `--bin`

1.2. **Question:** I'm trying to learn rust programming language? What do you recommend as starting points?

**Answer:** ✅ List of books, official documentation, advent of code, YouTube.

**Notes:**
Afterwards, I asked the model for some actual technical topics, and not a list of resource, and I got the regular spiel about rust's features.
I knew about all of these already but I think it's a good list for someone who didn't.

1.3. **Question:** How to parse command line arguments?

**Answer:** ✅/❌ Got some examples with `std::env`.

**Notes:**
I asked for some more powerful alternatives with reasons why.
ChatGPT told me about `clap` and with some further probing, it explained to me why it was a good choice.
Unfortunately, all code examples for `clap` were for some old version.
While they did work for that version (2.33), I want *fresh* documentation for the latest(-ish) versions of libraries (as of writing this the latest version of `clap` is 4.4.11).
ChatGPT told me that its training data cut-off was **January 2022** but the internet claims it is **April 2023**.
Go figure.
In the end, I just had to use the docs to get an arg parser going.

1.4. **Question:** How to use <insert basic feature X>, e.g. defining functions, importing from other files, conditionals, pattern matching, error handling...?

**Answer:** ✅/❌ Code examples with explanations.

**Notes:** Sometimes I asked it something that can't be done (AFAIK), like importing multiple modules on the same line `mod mod1, mod2, mod3;` but ChatGPT was convinced it was possible.
It would give me code that either doesn't work or does something totally different.
Nudging it to fix its mistake didn't work most of the time, and I don't think spending time to get it to generate the correct answer is a) worth it and b) representative of me learning after I already googled the answer in 10s.
![Failed attempt at importing many modules on a single line](screenshots/modules.webp)

1.5. **Question:** Explain in detail how borrow checking and lifetimes work in rust.
**Answer:** ✅ Got decently detailed answers -- comparable to an introductory blogpost.
No complaints there.

1.6. **Question:** What kind of pointers are there in rust and how do you use them?
**Answer:** ✅ Got decently detailed answers -- comparable to an introductory blogpost.
No complaints there.

1.7. **Question:** How to read a text file in rust?
**Answer:** ✅ I got some code examples with explanations.
**Notes:** What I got was workable.
I still googled around because I wanted to check what are the alternatives and what are the most rust-idiomatic ways.
ChatGPT wasn't very good at capturing that.

1.8. **Question:** What does `?` do in rust?
**Answer:** ✅ I got high-level explanation on unwrapping `Options` and `Results`.
**Notes:** It's fine for now.
My hunch is that it won't be when I dig deeper.

1.9. **Question:** What's the best collection for keeping a growing number of elements?
**Answer:** ✅ Appaz `Vec` with some explanations how toy you use it.
Notably, some examples with `map` and `collect`.

1.10. **Question:** What's the syntax for closures and how to use them? Are there any tricky things to look out for when using closures?
**Answer:** ✅ High-level explanation.
According to ChatGPT capture mode, borrow checker and lifetimes, mutability, return type inference, size and cloning, named closures are important considerations.
**Notes:** I haven't used rust enough to know if it makes sense.
Intuitively, I can see why some of these could be tricky, some not really.

1.11. **Question:** How to map an `Iterator` of `Results`? + a bunch of follow up questions regarding `map`, `filter`, `filter_map`, differences between `?` and `unwrap`.
**Answer:** ❌ ChatGPT just tells me to `unwrap` results when mapping an iterator.
**Notes:** This will result in `!panic` if something fails for an individual element.
I wanted to do it gracefully but `?` is weird to use.
I googled a bit and I've found good explanations of what the difference between `?` and `unwrap` is.
Then, I found out about `filter_map`.
Now, knowing what I don't know, I figured out how to formulate the question to ChatGPT, and it told me to use `filter_map`.
Quite backwards.

1.12. **Question:** Various questions regarding string slicing, finding occurrences, `Vec` and `HashMap`, closures and comparators.
**Answer:** ✅ For the most part, I think it was quite good at telling my how to do index and condition -based operations.
I did some additional searches when I wanted to look at the documentation.

X. **Question:** how do you tell?
**Answer:** can't tell
**Notes:** none

## Part II (WIP)

2.1. **Question:** How to provide inline type hints?
**Answer:** ✅ `::<T>`.

2.2. **Question:** How to use `struct`?
**Answer:** ✅ Got a good example with some explanations.

2.3. **Question:** A bunch of questions regaridng collections, e.g. `zip`, `sum`.
**Answer:** ✅ I think overall good examples of working with collections but often I'd look up more info in the documentation.

2.4. **Question:** How to use regex? How to parse strings to numbers?
**Answer:** ✅/❌ Got fairly basic info without really explaining what is going on.
**Notes:** Searching around for examples and using the docs was much better than what ChatGPT gave me.

X. **Question:** how do you tell?
**Answer:** can't tell
**Notes:** none

## Part III

Coming soon!

# Rustle

A couple of small command line programs to help solve [Wordle](https://www.powerlanguage.co.uk/wordle/) puzzles.

Note IMHO the whole point of Wordle is to solve it without assistance, instead I have done this
as a programing exercise for the purpose of learning about the [Rust](https://www.rust-lang.org/) programming language.

## Filter

Filters a dictionary of words for those that match the specified pattern.

Usage:

```shell
filter <guess> <outcome> [words]
```

Arguments:

* `guess` - the guess that you made
* `outcome` - the outcome of the guess. Exact matches are represented by `!`, letters that appear elsewhere are represented by `?` and letters that do not appear are represented by `.`
* `words` - the path to the dictionary file to use, defaults to `dict.txt`. Use `-` to read from standard input

Examples:

```shell
filter bound '.!!!!'

found
hound
lound
mound
pound
round
sound
wound
```

```shell
filter bound '.!!!!' | filter farms '...?.' -

mound
```

## Rank

Scores a dictionary of guess words according to how well they filter a dictionary of candidate answers.

Usage:

```shell
filter <answer-words> <guess-words>
```

Arguments:

* `answer-words` - the path to the dictionary of possible answer words. Use `-` to read from standard input
* `guess-words` - the path to the dictionary of guess words to score

Note it is often beneficial to restrict guesses to possible answers ("hard" mode), in which case the
`guess-words` dictionary would be the same as the `answer-words`.

Output:

Outputs CSV data with the following headings:

* `WORD` - the guess word in question
* `COUNT` - the number of answers this word has been tested against (size of `answer-words` dictionary)
* `MEDIAN` - the median **SCORE** of using this word (typical scenario)
* `MAX` - the maximum **SCORE** of using this word (worst case scenario)
* `SUM` - the sum of all **SCORE**s (divide this by `COUNT` to get the average)

where **SCORE** is the number of possible answer words that would be left if this word was chosen (the _lower_ the better).

## Build and test

The repository is built by running

```shell
cargo build
```

and tested by running

```shell
cargo test
```

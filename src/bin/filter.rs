use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use structopt::StructOpt;

/// Filters a dictionary of words for those that match the specified pattern.
#[derive(StructOpt)]
struct Cli {
    /// The guess that you made
    guess: String,
    /// The outcome that you got, in the form `.?!..`
    outcome: String,
    /// The path to the dictionary file to use
    #[structopt(default_value = "dict.txt", parse(from_os_str))]
    words: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    if args.words.to_str() == Some("-") {
        return print_compatible_words(std::io::stdin().lock(), &args.guess, &args.outcome);
    } else {
        return print_compatible_words(
            BufReader::new(File::open(args.words)?),
            &args.guess,
            &args.outcome,
        );
    }
}

fn print_compatible_words<T: BufRead>(reader: T, guess: &str, outcome: &str) -> io::Result<()> {
    for line in reader.lines() {
        // Check exact matches
        let l = line?;

        if is_compatible(&l, guess, outcome) {
            println!("{}", l);
        }
    }
    Ok(())
}

fn is_compatible(line: &str, guess: &str, outcome: &str) -> bool {
    let mut exact_matches_ok = true;
    let mut must_not_exist = Vec::with_capacity(outcome.len());
    let mut must_exist = Vec::with_capacity(outcome.len());
    let mut remainder = Vec::with_capacity(outcome.len());

    // Check exact matches, building up a list of what's left
    for i in 0..outcome.len() {
        match outcome.as_bytes()[i] {
            b'!' => {
                // char must exist at this location
                exact_matches_ok &= guess.as_bytes()[i] == line.as_bytes()[i];
            }
            b'?' => {
                // char must exist at a different location
                exact_matches_ok &= guess.as_bytes()[i] != line.as_bytes()[i];
                must_exist.push(guess.as_bytes()[i]);
                remainder.push(line.as_bytes()[i]);
            }
            _ => {
                // char must NOT exist, at this location or anywhere in the remainder
                exact_matches_ok &= guess.as_bytes()[i] != line.as_bytes()[i];
                must_not_exist.push(guess.as_bytes()[i]);
                remainder.push(line.as_bytes()[i]);
            }
        }
    }
    if !exact_matches_ok {
        return false;
    }

    // Check must_not_exist
    for m in must_not_exist {
        for r in &remainder {
            if m == *r {
                return false;
            }
        }
    }

    // Check must_exist
    'outer: for m in must_exist {
        for (i, r) in remainder.iter().enumerate() {
            if m == *r {
                remainder.swap_remove(i);
                continue 'outer;
            }
        }
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_correct_match() {
        assert_eq!(is_compatible("apple", "apple", "!!!!!"), true);
    }

    #[test]
    fn all_correct_nomatch() {
        assert_eq!(is_compatible("apace", "apple", "!!!!!"), false);
    }

    #[test]
    fn some_correct_match() {
        assert_eq!(is_compatible("apace", "apple", "!!--!"), true);
    }

    #[test]
    fn some_correct_not_enough_match() {
        assert_eq!(is_compatible("arace", "apple", "!!--!"), false);
    }

    #[test]
    fn some_correct_too_many_match_same_place() {
        assert_eq!(is_compatible("apple", "apple", "!!--!"), false);
    }

    #[test]
    fn some_correct_too_many_match_different_place() {
        assert_eq!(is_compatible("aplpe", "apple", "!!--!"), false);
    }

    #[test]
    fn misplaced_match() {
        assert_eq!(is_compatible("aplpe", "apple", "!!??!"), true);
    }

    #[test]
    fn misplaced_too_many_match_same_place() {
        assert_eq!(is_compatible("apple", "apple", "!!??!"), false);
    }

    #[test]
    fn misplaced_too_few_match() {
        assert_eq!(is_compatible("aplie", "apple", "!!??!"), false);
    }
}

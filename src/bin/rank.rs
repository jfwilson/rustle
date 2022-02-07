use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use structopt::StructOpt;

/// Scores a dictionary of guess words according to how well they filter a dictionary of candidate answers
#[derive(StructOpt)]
struct Cli {
    /// The path to the dictionary of possible answer words
    #[structopt(default_value = "dict-small.txt", parse(from_os_str))]
    answer_words: std::path::PathBuf,
    /// The path to the dictionary of guess words to score
    #[structopt(default_value = "dict-small.txt", parse(from_os_str))]
    guess_words: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let answers = load_answers(args.answer_words)?;
    println!("WORD,COUNT,MEDIAN,MAX,SUM,IS_CANDIDATE");
    for guess_word in BufReader::new(File::open(args.guess_words)?).lines() {
        let guess_ref = &guess_word?;
        let mut scores = determine_score(guess_ref, &answers);
        let is_candidate = scores[242] > 0;
        scores.sort_unstable();
        let max = scores[242];
        let count: u32 = scores.iter().sum();
        let mut count_until_median = count >> 1;
        let mut median = 0u32;
        let sum = scores.iter().fold(0u32, |acc, s| {
            if count_until_median > 0 {
                median = *s;
                count_until_median = count_until_median.saturating_sub(*s);
            };
            acc + s * s
        });
        println!(
            "{},{},{},{},{},{}",
            guess_ref, count, median, max, sum, is_candidate
        );
    }
    Ok(())
}

fn load_answers(path: std::path::PathBuf) -> io::Result<Vec<String>> {
    if path.to_str() == Some("-") {
        return std::io::stdin().lock().lines().collect();
    } else {
        return BufReader::new(File::open(path)?).lines().collect();
    }
}

fn determine_score(guess: &str, answers: &Vec<String>) -> [u32; 243] {
    let mut outcomes = [0u32; 243];
    for answer in answers {
        // Determine outcome pattern
        let pattern = determine_outcome(guess.as_bytes(), answer.as_bytes());
        // Increment counter for this outcome
        outcomes[pattern] += 1;
    }
    return outcomes;
}

fn determine_outcome(guess: &[u8], answer: &[u8]) -> usize {
    let both_match: [bool; 5] = [
        guess[0] == answer[0],
        guess[1] == answer[1],
        guess[2] == answer[2],
        guess[3] == answer[3],
        guess[4] == answer[4],
    ];
    let mut answer_visited: [bool; 5] = both_match;
    let mut pattern: usize =
        if both_match[0] { 2 }
        else if !answer_visited[1] && guess[0] == answer[1] { answer_visited[1] = true; 1 }
        else if !answer_visited[2] && guess[0] == answer[2] { answer_visited[2] = true; 1 }
        else if !answer_visited[3] && guess[0] == answer[3] { answer_visited[3] = true; 1 }
        else if !answer_visited[4] && guess[0] == answer[4] { answer_visited[4] = true; 1 }
        else { 0 };
    pattern +=
        if both_match[1] { 2 * 3 }
        else if !answer_visited[0] && guess[1] == answer[0] { answer_visited[0] = true; 1 * 3 }
        else if !answer_visited[2] && guess[1] == answer[2] { answer_visited[2] = true; 1 * 3 }
        else if !answer_visited[3] && guess[1] == answer[3] { answer_visited[3] = true; 1 * 3 }
        else if !answer_visited[4] && guess[1] == answer[4] { answer_visited[4] = true; 1 * 3 }
        else { 0 };
    pattern +=
        if both_match[2] { 2 * 9 }
        else if !answer_visited[0] && guess[2] == answer[0] { answer_visited[0] = true; 1 * 9 }
        else if !answer_visited[1] && guess[2] == answer[1] { answer_visited[1] = true; 1 * 9 }
        else if !answer_visited[3] && guess[2] == answer[3] { answer_visited[3] = true; 1 * 9 }
        else if !answer_visited[4] && guess[2] == answer[4] { answer_visited[4] = true; 1 * 9 }
        else { 0 };
    pattern +=
        if both_match[3] { 2 * 27 }
        else if !answer_visited[0] && guess[3] == answer[0] { answer_visited[0] = true; 1 * 27 }
        else if !answer_visited[1] && guess[3] == answer[1] { answer_visited[1] = true; 1 * 27 }
        else if !answer_visited[2] && guess[3] == answer[2] { answer_visited[2] = true; 1 * 27 }
        else if !answer_visited[4] && guess[3] == answer[4] { answer_visited[4] = true; 1 * 27 }
        else { 0 };
    pattern +=
        if both_match[4] { 2 * 81 }
        else if !answer_visited[0] && guess[4] == answer[0] { answer_visited[0] = true; 1 * 81 }
        else if !answer_visited[1] && guess[4] == answer[1] { answer_visited[1] = true; 1 * 81 }
        else if !answer_visited[2] && guess[4] == answer[2] { answer_visited[2] = true; 1 * 81 }
        else if !answer_visited[3] && guess[4] == answer[3] { answer_visited[3] = true; 1 * 81 }
        else { 0 };
    return pattern;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp(guess: &str, answer: &str) -> [u8; 5] {
        let mut outcome = determine_outcome(guess.as_bytes(), answer.as_bytes());
        let symbols: [u8; 3] = [b'.', b'?', b'!'];
        return [
            symbols[outcome % 3],
            symbols[{ outcome /= 3; outcome % 3 }],
            symbols[{ outcome /= 3; outcome % 3 }],
            symbols[{ outcome /= 3; outcome % 3 }],
            symbols[{ outcome /= 3; outcome % 3 }],
        ];
    }

    #[test]
    fn check_determine_outcome() {
        assert_eq!(std::str::from_utf8(&cmp("apple", "apple")).unwrap(), "!!!!!");
        assert_eq!(std::str::from_utf8(&cmp("apple", "shape")).unwrap(), "??..!");
        assert_eq!(std::str::from_utf8(&cmp("shape", "apple")).unwrap(), "..??!");
        assert_eq!(std::str::from_utf8(&cmp("apple", "plump")).unwrap(), ".???.");
        assert_eq!(std::str::from_utf8(&cmp("plump", "apple")).unwrap(), "??..?");
    }
}

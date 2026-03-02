fn count_matches(words: &Vec<&str>, target: &str, count: &mut usize) {

    *count = 0;
    for i in 0..words.len() {
        if words[i] == target {
            *count += 1; 
        }
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {

    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count = 0;

    for i in 0..words.len() {

        let mut count = 0;

        count_matches(&words, words[i], &mut count);

        if count > max_count {
            max_count = count;
            max_word = words[i];
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {

    let text =
        "the quick brown fox jumps over the lazy dog the quick brown fox";

    let (word, count) = most_frequent_word(text);

    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

// Define the character set to encode (excluding - _ . ! ~ * ' ( ))
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'+')
    .add(b',')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

fn encode_uri_component(s: String) -> String {
    utf8_percent_encode(&s, FRAGMENT).to_string()
}

pub fn file_path_to_uri(file_path: &str) -> String {
    let remove_file_type = Regex::new("\\.md").expect("Valid Regex").replace_all(file_path, "").to_string();
    let replace_space = remove_file_type.replace(' ', "-");
    encode_uri_component(replace_space)
}

pub fn calculate_reading_time(text: &str) -> String {
    let words_per_minute = 200; // Average reading speed
    let word_count = text.split_whitespace().count();
    let reading_time_minutes = word_count / words_per_minute;

    let minutes = if word_count % words_per_minute > 0 {
        reading_time_minutes + 1 // Round up if there are extra words
    } else {
        reading_time_minutes
    };

    format!("{minutes} mins")
}

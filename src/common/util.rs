use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

// Define the character set to encode (excluding - _ . ! ~ * ' ( ))
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
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
    encode_uri_component(remove_file_type)
}

pub fn pig_latin(string: &str) -> String {
  let vowels = ['a', 'e', 'i', 'o', 'u'];
  let mut rem = &string[..];
  let mut result = String::new();
  while rem.len() > 0 {
    let first_char = rem.chars().next().unwrap();
    if first_char.is_alphabetic() {
      let (word, rest) = split_at_occurence(rem, |c: char| !c.is_alphabetic());
      rem = rest;

      if vowels.contains(&first_char) {
        result.push_str(word);
        result.push_str("-hay");
      } else {
        let second_char_index = first_char.len_utf8();
        result.push_str(&word[second_char_index..]);
        result.push_str("-");
        result.push(first_char);
        result.push_str("ay");
      }
    } else {
      let (whitespace, rest) = split_at_occurence(rem, char::is_alphabetic);
      rem = rest;
      result.push_str(whitespace);
    }
  }

  result
}

fn split_at_occurence<P: Fn(char) -> bool>(string: &str, p: P) -> (&str, &str) {
  match string.find(p) {
    Some(i) => string.split_at(i),
    None => (string, ""),
  }
}

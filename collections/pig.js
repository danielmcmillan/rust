const vowels = ['a', 'e', 'i', 'o', 'u'];

function splitAt(string, pattern) {
  const match = string.search(pattern);
  if (match < 0) {
    return [string, ''];
  } else {
    return [string.substring(0, match), string.substring(match)];
  }
}

function pigLatin(string) {
  let rem = string;
  let result = '';
  while (rem.length > 0) {
    if (/\s/.test(rem[0])) {
      // Next char is whitespace, include part up to the next word unchanged
      let whitespace;
      [whitespace, rem] = splitAt(rem, /[^\s]/);
      result += whitespace;
    } else {
      // Next char is word
      let word;
      [word, rem] = splitAt(rem, /\s/);

      if (vowels.includes(word[0])) {
        result += `${word}-hay`;
      } else {
        result += `${word.substring(1)}-${word[0]}ay`;
      }
    }
  }
  return result;
}

console.log([
  pigLatin('hello नमस्ते')
]);

fn capitalized(target: String) -> String {
  let mut target_chars = target.chars();
  match target_chars.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().chain(target_chars).collect(),
  }
}

pub fn verse(number: u32) -> String {
  let b = |x| match x {
    0 => "no more bottles of beer".to_string(),
    1 => "1 bottle of beer".to_string(),
    _ => format!("{} bottles of beer", x),
  };
  let mut result = format!("{} on the wall, {}.\n", capitalized(b(number)), b(number));

  result.push_str(
    &match number {
      0 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
      1 => "Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
      _ => format!("Take one down and pass it around, {} on the wall.\n", b(number - 1)),
    }
  );
  result
}

pub fn sing(start: u32, end: u32) -> String {
  (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}

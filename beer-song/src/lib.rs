pub fn verse(n: u32) -> String {
    let mut str: String = String::new();
    let m: String = n.to_string();
    if n == 0 {
      str += "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
    } else {
      str += &m;
      str += &get_bottle(n > 1);
      str += " of beer on the wall, ";
      str += &m;
      str += &get_bottle(n > 1);
      str += " of beer.\nTake ";
      if n > 1 {
        str += "one";
      } else {
        str += "it";
      }
      str += " down and pass it around, ";
      if n == 1 {
        str += "no more bottles";
      } else {
        str += &(n - 1).to_string();
        str += &get_bottle(n - 2 > 0);
      }
      str += " of beer on the wall.\n";
    }
    str
}

fn get_bottle(x: bool) -> String {
  if x { String::from(" bottles") } else { String::from(" bottle") }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut str: String = String::new();
    for n in (end..(start + 1)).rev() {
      str += &verse(n);
      if n != end {
        str += "\n";
      }
    }
    str
}

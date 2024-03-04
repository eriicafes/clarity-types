#![allow(dead_code)]

pub fn capitalize(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

pub fn to_pascal_case(s: &str) -> String {
  s.split("-").map(capitalize).collect::<String>()
}

pub fn to_js_key(s: &str) -> String {
  if s.contains("-") {
    format!("\"{s}\"")
  } else {
    s.to_string()
  }
}

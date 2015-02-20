#![feature(collections)]
extern crate collections;
use collections::BTreeSet;

trait Digitize {
  fn digits(self) -> Vec<usize>;
}

impl Digitize for usize {
  fn digits(self) -> Vec<usize> {
    let mut x = self.clone();
    let mut xs = vec![];
    if self == 0 {
      return vec![0];
    }
    while x > 0 {
      xs.push(x % 10);
      x /= 10;
    }
    xs.reverse();
    xs
  }
}

fn is_happy(n: usize) -> bool {
  let mut c = n.clone();
  let mut s: BTreeSet<usize> = BTreeSet::new();
  while c > 1 {
    c = c.digits().iter().fold(0, |a, &b| a + b * b);
    if s.contains(&c) {
      return false;
    }
    s.insert(c);
  }
  return true;
}

fn main() {
  let mut found = 0;
  let mut n = 1;
  while found < 8 {
    if is_happy(n) {
      println!("{}", n);
      found += 1;
    }
    n = n + 1;
  }
}

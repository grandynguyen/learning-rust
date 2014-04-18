trait Digitize {
  fn digits(self) -> Vec<int>;
}

impl Digitize for int {
  fn digits(self) -> Vec<int> {
    let mut x = self.clone();
    let mut vec = Vec::new();
    while x != 0 {
      vec.push(x % 10);
      x = x / 10;
    };
    return vec;
  }
}

fn happy(n: int) -> bool {
  let mut curr = n.clone();
  let mut s: Vec<int> = Vec::new();
  let mut acc;

  while curr > 1 {
    acc = 0;
    for x in curr.digits().iter() {
      acc = *x * *x + acc;
    }
    curr = acc;
    
    if s.contains(&acc) {
      return false;
    } else {
      s.push(acc);
    }

  }
  return true;
}

fn main() {
  let mut found = 0;
  let mut n = 1;

  while found < 8 {
    if happy(n) {
      println!("{}", n);
      found += 1;
    }
    n = n + 1;
  }
}

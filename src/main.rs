// use std::collections::HashMap;

fn foo(s: &mut String) {
  // s.push_str(" You are batman.");
  *s = String::from("I am a batman. You are superman.");
}

// fn main() {
//   let mut s1 = String::from("I am a superman.");
//   println!("{s1}");
//   foo(&mut s1);    // 注意这里传的是字符串的可变引用 &mut s1
//   println!("{s1}");
// }
//

// fn main() {
//   let mut a: u32 = 10;
//   let b = &a;
//   println!("{}", b);
//   let c = &mut a;
//   println!("{}", c);
//   a = 11;
//   println!("{}", a);
//   let d = &mut a;
//   println!("{}", d);
// }

// 命名结构体
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct Class {
  serial_number: u32,
  grade_number: u32,
  entry_year: String,
  members: Vec<User>,  
}

fn main() {
  let active = true;
  let username = String::from("someusername123");
  let email = String::from("someone@example.com");
  let user1 = User {
      active,
      username,
      email,
      sign_in_count: 1,
  };
  let user2 = User {
      email: String::from("another@example.com"),
      ..user1
  };
  // 会发生所有权的部分转移
  // let email = user1.email;
  let email = &user2.email;
  println!("{email}");
  println!("{}", user2.email);

  // 元组结构体
  struct Color(i32, i32, i32);
  let mut black = Color(0, 0, 0);
  black.0 = 1;
  let x = black.0;
  println!("{}", x);
  println!("{}", black.0);

  // 单元结构体, 只有类型，没有字段
  struct Unit;
  // 可以初始化为变量
  let unit = Unit;
}

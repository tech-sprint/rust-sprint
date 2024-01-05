fn main() {
  let s: u32 = 33;
  println!("{s}");
  let i: i32 = -12;
  println!("{}", i);

  let s1 = "s1: # sdfsdf \\ #";
  println!("{}", s1);
  let s2 = String::from("s2: sdfsdf \\");
  println!("{}", s2);
  let s3 = r"s3: # \\ \n";
  println!("{}", s3);
  let s4 = r#""s4: # " sdfsdlfjsf"#;
  println!("{}", s4);
  let s5 = r####"s5: """"" ### sdflkjklsdf"####;
  println!("{}", s5);


  // 字节串的类型是字节的数组，而不是字符串了
  let bytestring: &[u8; 21] = b"this is a byte string";
  println!("A byte string: {:?}", bytestring);
  // 可以看看下面这串打印出什么
  let escaped = b"\x52\x75\x73\x74 as bytes";
  println!("Some escaped bytes: {:?}", escaped);
  // 字节串与原始字面量结合使用
  let raw_bytestring = br"\u{211D} is not escaped here";
  println!("{:?}", raw_bytestring);

  // 数组
  let arr: [i32; 4] = [1, 2, 3, 4];
  let arr2 = [1, 2, 3, 4];
  println!("{}, {}", arr[1], arr2[1]);

  // 动态数组 vec
  let mut v1: Vec<_> = Vec::new();
  v1.push("q");
  v1.push("w");
  println!("v1: {}", v1[1]);
  // 不用 mut 声明，将不允许 push
  let mut v2 = vec!["e", "r"];
  v2.push("t");
  println!("v2: {}", v2[2]);

  // let mut hash_map1 = HashMap::new();
  // hash_map1.insert("a", v1[0]);
  // hash_map1.insert("b", v1[1]);
  // println!("h1: {}", hash_map1.get("b"));

  // 元组, 用 '.' 访问元素，定长，类型不定。数组, 定长类型一致
  // 元组可以用作 函数的返回值
  // 没有元素的元组，退化成 (), 叫做 unit 类型，这个类型只有一个值 ()
  // 函数没有返回值的时候，默认返回值是 unit
  // 元组像是简化了的 struct。确实有元组结构体这个类型
  let tup: (i32, u8) = (120, 2);
  println!("{}", tup.0);
  // 枚举中的选项，叫做此枚举的变体(variants),
  // 变体可以理解为，此枚举的一个个实例，比如下面这个枚举，只有这两个实例。
  // 学术上，通常把枚举叫作和类型（sum type），把结构体叫作积类型（product type）。
  // enum IpAddrKind {
  //   V4,
  //   V6,
  // }
  // let four = IpAddrKind::V4;
  // let six = IpAddrKind::V6;
}

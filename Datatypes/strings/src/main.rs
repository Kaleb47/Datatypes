fn main() {
  // Mutable string
  // let mut s = String::new();

  let data = "initial contents";

  let s = data.to_string();

  // The above is equivalent to
  // let s = "initial contents".to_string();

  // Also
  // let s = String::from("initial contents");

  // UTF-8 Encoded
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


    let mut s3 = String::from("lu");
    s3.push('l');
    combine_strings_with_plus();
    iterating_over_strings();
}
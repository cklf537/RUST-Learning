fn main() {
    

    let mut s = String::from("hello world");

    let _word = first_word(&mut s); // word will get the value 5
    
    // println!("{:?}", word);
    // println!("{:?}", s);
    // print!("{:?}", &s);

    // s.clear(); // this empties the String, making it equal to ""
    
    // print!("{:?}", &word.len());
    // print!("{}", _word);
    // let some_otherstring: &str = &_word;

    print!("{:?}", one_wordfun(&s));


    let arr = [1,2,3,4,5,6];
    
    print!("{:?}", &arr[1..2]);
}

fn first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn one_wordfun(s: &str) -> &str{
    &s[0..5]
}
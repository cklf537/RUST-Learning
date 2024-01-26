
fn main() {

    let number = 10; 
    let number1 = 20;

    let flt_number = 12.01;

    // return_num(&number);

    print!("{}", return_num(&number, &number1));

    print!("Convert float to int {}", flt_number as i32);

    let mut greet = String::from(" Hello");

    println!("{}", append_string(&mut greet));

}

fn return_num(v: &i32, v1: &i32) -> i32 {
    v + v1
}


fn append_string(some_string: &mut String) -> String {
    some_string.push_str(" Chandra Shekar");
    some_string.to_string()
}

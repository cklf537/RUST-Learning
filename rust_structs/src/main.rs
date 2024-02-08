fn main() {

    //=========== Creating Structs ===============

    // WHAT ARE STRUCTS?
    // Structs are similar to Tuple Typs.
    // Structs can be created using keyword 'struct'.
    // Structs can hold Multiple releated values. (Ex: bool, u32, i64, String).
    // Structs propertsis are clearly defined with appropriate 'data types'.
    // Structs can be used by creating on instance of Struct model (Ex: reffer User struct model & user instance).
    // Structs uses (.)dot notation to access perticular value.
    // Struct instance values can be modified if the instance is mutable(mut - keyword).
    // Entire Struct instance is mutable, individual value in struct instance can not be made as mutabel.

    // User struct model

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Person Struct Model

    struct Person {
        first_name: String,
        last_name: String,
    }

    // Create user instance with 'User' struct model

    let user = User {
        active: true,
        username: String::from("username"),
        email: String::from("someemail@info.com"),
        sign_in_count: 1,
    };

    // Create person instance with 'Person' struct model

    let person1 = Person {
        first_name: String::from("First Name"),
        last_name: String::from("Last Name"),
    };

    let mut heading = String::from("Access user details from user instance using (.)dot notation");
    println!("{:?}", print_heading(&mut heading));

    println!("{:?}, {:?}, {:?}, {:?}", user.active, user.username, user.email, user.sign_in_count);
    println!("{:?}, {:?}", person1.first_name, person1.last_name);

    // let mut anoter_heading = String::from("Another_heading");
    // println!("{:?}", print_heading(&mut anoter_heading));

}

fn print_heading(s: &mut String) -> &str {
    s.insert_str(0, "******* ");
    s.push_str(" *******");
    s
}
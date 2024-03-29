fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let mut s = String::from("hello");
    println!("The string created on heap: {}", s);
    s.push_str(" world!");

    println!("s:{}", s);

    //let s_temp = String::from("temp");

    // let s_temp1 = s_temp;
    // println!("{}, world!", s_temp);
    // this doesn't work because rust moved the value
    // and s_temp is not valid anymore

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //references
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s1's len:{}", len);

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User1 info: {}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);
    let user2 = build_user(String::from("hello@gmail.com"), String::from("user2"));

    println!("User2 info: {}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    println!("User3 info: {}, {}, {}, {}", user3.active, user3.username, user3.email, user3.sign_in_count);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
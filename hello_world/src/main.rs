fn main() {
    println!("\n\n\nWelcome to my program!\n");

    for i in 1..10 {
        foo_bar(i);
    }
}

fn foo_bar(num : i32) {
    match num {
        n if n % 3 == 0 && n % 5 == 0 => println!("foobar"),
        n if n % 3 == 0 => println!("foo"),
        n if n % 5 == 0 => println!("bar"),
        _ => println!("{}", num),
    }
}
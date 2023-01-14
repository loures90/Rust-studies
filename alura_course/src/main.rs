fn main() {
    println!("Hello, world!");
    conditions();
    repetitions();
    match_statement();
    ownership();
    patterning_matching();
}
// ! -> calling a Rust macro
// ! -> Without ! is calling a function
// Rust is an ahead-of-time compiled language, meaning you can compile a program and give the 
// executable to someone else, and they can run it even without having Rust installed
// rustc main.rs -> compile the source code and generate the executable

fn conditions() {
    let age: u8 = 17;
    let responsible_allwed=true; 
    let is_older: bool = age >= 18;

    if is_older {
        println!("Go to Party");
    } else if age >= 16 && responsible_allwed {
        println!("Go to Party with parents authorization");
    } else {
        println!("Can not Go to Party");
    }

    let condition = if age >= 18 { "older" } else { "younger" };
    println!("He is {} than 18!", condition);
}

fn repetitions(){
    let multi: u8 = 5;
    let mut count = 0;
    while count <=10 {
        println!("{} x {} = {}", count, multi, count*multi);
        count +=1;
    }

    let multi_b = 4;
    count = 0;
    loop {
        println!("{} x {} = {}", count, multi_b, count*multi_b);
        if count == 10 { break; }
        count +=1;
    }

    let multi_c = 6;
    for i in 0..=10 { // excluded
        println!("{} x {} = {}", i, multi_c, i*multi_c);
    }

    for i in 0..11 { // included
        println!("{} x {} = {}", i, multi_c, i*multi_c);
    }
}

fn match_statement() {
     let language = "PHP";
     let main_go = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Unknown"
     };
     println!("The main go of {} is {}.", language, main_go);
}

fn ownership(){
    let a_string = String::from("bla bla bla");
    println!("{}", a_string);

    still_ownership(a_string);

    //println!("{}", a_string);  Error!!
    // The function still_ownership "still ownership from a_string valriable."
    // The memory alocated in heap is cleaned when it loses the ownership.

    let b_string = String::from("bla bla bla");
    still_ownership_borrowing(&b_string); // b_string borrows the references to the function
                                          // So, b_string does not loose the ownership
    println!("{}", b_string);              // But it can not be mutated.


    let c_string = String::from("bla bla bla");
    still_ownership_borrowing_try_mutate(&c_string);
    println!("{}", c_string);

    let mut d_string = String::from("bla bla bla");
    still_ownership_borrowing_mutate(&mut d_string);
    println!("{}", d_string);
}

fn still_ownership(string: String) {
    println!("{}", string);
}

fn still_ownership_borrowing(string: &String) { // theis function borrow the reference of a variable.
    println!("{}", string);
}

fn still_ownership_borrowing_try_mutate(string: &String) { // Returns an erro because the variable is imutable
 //   string.push_str("It is an error");  // because variable and param are imutable by default
    println!("{}", string);
}

fn still_ownership_borrowing_mutate(string: &mut String) { // Returns correct because variable and param are explicty mutable
    string.push_str("It is ok!"); 
    println!("{}", string);
}

fn patterning_matching(){
    for x in 1..=20 {
        println!("{}, {}", x, match x {
            1 => "a little",
            2 | 3 => "almost there",
            4..=10 => "It is almost there",
            _ if x %2 == 0 => "Perfect",
            _ => "too much",
        });
    } 
}
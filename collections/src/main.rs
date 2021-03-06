fn main() {
    // creates empty vector, specifying the type to store
    let v: Vec<i32> = Vec::new();

    // More commonly we just insert values and the type is inferred
    // But how does it know it's not a u8? 
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("vector is {:?}", v);
    println!("vector part 3 is {:?}", &v[2]); // panic on bad index
    println!("yes, vector part 3 is {:?}", v.get(2)); // handle out-of-bounds 

    for i in &mut v {
        *i += 50;
    }

    //new string
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string(); // you can also use this on the literal

    // the above is equivalent to
    let mut s = String::from("initial contents");

    let s2 = " and more";
    s.push_str(s2); // append data to string w/out taking ownership

    println!("s2 is {}", s2);

    //append just one char (Must use single quote ' ' not double " ")
    s.push('!');

    // Concatenate strings
    let t1 = String::from("Hey");
    let t2 = String::from(", what's up?");
    let t3 = t1 + &t2; // here t1 is moved and falls out of scope
    // the first thing to be added is always a string
    // the rest are borrows or raw data

    println!("t3 is {}", t3);

    // We can concatanate a lot more with the format! macro
    let f1 = String::from("tic");
    let f2 = String::from("tac");
    let f3 = String::from("toe");

    let f = format!("{}-{}-{}", f1, f2, f3);

    println!("f is {}", f);

    // try concatenating data
    let d1 = "Hey";
    let d2 = ", how's it going in there?";
    let d3 = d1.to_string() + &d2; // first thing has to be sent to string
    
    println!("{}", d3);

    // using ranges to create string slices is dangerous
    // because out of boundary errors can cause panic at runtime
    let slice = &d2[0..4]; // &f2 here would compile but panic
    println!("{}", slice);

    //iterating over strings two ways
    for c in "coop".chars() {
        println!("{}", c);
    };
    for c in "coop".bytes() {
        println!("{}", c);
    };



}

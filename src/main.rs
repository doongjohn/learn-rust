#![allow(dead_code)]
#![allow(unused_variables)]

// Crust of Rust (in-depth explanation of rust language)
// https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa

// TODO: learn this
// - Fn, FnMut, FnOnce
// - dyn
// - move

// Heap memory
// Box, Rc, Arc, are smart pointers
// - Box: deallocation happens when it goes out of scope
// - Rc: referance counted pointer
// - Arc: atomic referance counted pointer

fn main() {
    // module (namespace)
    {
        // `use` is equivalent to c++ `usingnamespace`
        use greetings::greetings_inner::*;
        use greetings::*;

        hello();
        hello_world();
        // say_hello(); // <-- error: only visiable in greetings module
    }

    // Copy types: types that implement copy trait are copied on assignment
    // https://dhghomon.github.io/easy_rust/Chapter_19.html
    {
        let a = 10;
        let b = a;

        println!("a = {}", a); // <-- a is still alive because it is never moved
        println!("b = {}", b);
    }

    // other types get moved on assignment
    {
        let a = "hi".to_string();
        let b = a;

        // println!("a = {}", a); // <-- error: value of a is moved to b
        println!("b = {}", b);
    }

    // debug print
    {
        let v = vec![0, 1, 2, 3, 4];
        println!("{:?}", v);
        println!("{:#?}", v);
        //        ^^^^^ --> pretty print
    }

    // option type
    {
        let number = Some(10);

        // The `if let` construct reads: "if `let` destructures `number` into
        // `Some(i)`, evaluate the block (`{}`).
        // basically unwrapping Option
        if let Some(value) = number {
            println!("unwrapped value = {}", value);
        }
    }

    // taking ownership and take it back
    {
        let mut hello: String = "hello".to_string();
        println!("{}", hello);

        hello = append(hello, " and good bye"); // <-- this takes hello's ownership
                                                //                                             but it returns it back

        // append(hello, " and good bye"); // <-- this will invalidate the variable `hello`
        // //                                     because it takes its ownership

        println!("{}", hello);
    }
    // taking ownership and using variable shadowing
    {
        let hello: String = "hello".to_string();
        println!("{}", hello);
        let hello = append(hello, " and good bye");
        println!("{}", hello);
    }
    // borrowing
    {
        let mut hello: String = "hello".to_string();
        println!("{}", hello);
        append_borrow(&mut hello, " and good bye");
        println!("{}", hello);
    }

    // closure
    // https://doc.rust-lang.org/rust-by-example/fn/closures.html
    {
        let mut closures: Vec<Box<dyn Fn() -> i32>> = Vec::new();
        for i in 0..10 {
            closures.push(Box::new(move || -> i32 { i + 1 }));
        }

        for i in 0..10 {
            print!("{} ", closures[i]());
        }
        println!("");
    }

    // trait object (dynamic dispatch)
    // trait is a compile time construct (has no size at runtime)
    {
        let person = Person {
            name: "John".to_string(),
        };
        let dog = Dog {
            name: "Puppy".to_string(),
        };

        let mut animal: &dyn Animal;
        //               ^^^ --> trait object must use dyn keyword

        animal = &person;
        println!("{}", animal.name());

        animal = &dog;
        println!("{}", animal.name());
    }

    // ref vs &
    // https://users.rust-lang.org/t/ref-keyword-versus/18818/5
}

mod greetings {
    pub fn hello() {
        greetings_inner::say_hello();
    }

    pub mod greetings_inner {
        pub fn hello_world() {
            crate::greetings::hello();
            println!("hello world");
        }

        // this function is only visiable in `greetings` module
        pub(in crate::greetings) fn say_hello() {
            println!("hello");
        }
    }
}

// trait method can be called with dot syntax
// similar to UFCS (Universal function call syntax)
trait Animal {
    fn name(&self) -> &String;
}

struct Person {
    name: String,
}

impl Animal for Person {
    fn name(&self) -> &String {
        &self.name
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &String {
        &self.name
    }
}

fn say_name(a: &impl Animal) {
    println!("{}", a.name());
}

// parameter without borrowing will take its ownership
// more about mut parameter: https://www.snoyman.com/blog/2020/05/no-mutable-parameters-in-rust/
fn append<'a>(mut target: String, source: &str) -> String {
    target.push_str(source);
    target
}

// this instead use borrowing
fn append_borrow<'a>(target: &mut String, source: &str) {
    (*target).push_str(source);
}

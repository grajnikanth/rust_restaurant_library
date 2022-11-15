// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// lib.rs - this file is the crate root

// Example of Restaurant example for defining modules and organizing functionality
// Note that below we have modules nested inside of modules
// note the use of "mod" keyword to represent "module"
// note this first module front_of_house is private but the siblings have access to private modules
// siblings still have access to it without declaring it to be public
// Sibling in this case would be the function eat_at_restaurant
mod front_of_house {
    // You can make a module public with pub keyword
    // Making module public doesn't automatically make the functions inside
    // automatically public
    //Making module public only lets code in its ancestor module refer to it
    pub mod hosting {
        //default functions are private in modules. Child can access parent's function
        //even if they are private. Parent cannot access Child's functions which
        // are private
        // functions are part of the module tree structures in Rust
        // Path in RUST crate -> modules -> functions, variables etc
        
        pub fn add_to_waitlist() {}
        //child components can access private parent componenets
        // parent cannot access private child components
        // child components are defined in the context of parent so they have access to
        // the parent variables
        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// below function is defined at the root of this crate
// Path syntax to access functions in modules
// since we made this function below public, this function is available for access as part of library API to other code
pub fn eat_at_restaurant() {
    //absolute path - using crate as the root module
    // crate keyword in this file means this lib.rs file or the root of this crate, represented by code in this file
    // use of absolute paths is recommended
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path - Relative to this code location
    // since eat_at_restaurant function is a sibling of the module front_of_house we can use relative path as shown below,
    // that is we can call the front_of_house module (almost as if it is a directory in a file structure),
    // then hosting is like the next directory in the file structure analogy until we reach the function add_to_waitlist
    // the "::" can be thought of as "/" in a directory file structure system
    front_of_house::hosting::add_to_waitlist();

    // accessing structs
    //see module back_of_house for struct Breakfast used here
    //order a breakfast in summer with Rye toast
    // notice the syntax how to call the function summer inside sruct. This syntax is similar
    // to the syntax of calling a function inside a module
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //we can change the toast if we would like becasue the toast in struct Breakfast is public
    meal.toast = String::from("Wheat");
    println!("meal.toast = {}", meal.toast);

    // since seasonal_fruit in Breakfast struct is private, we cannot access it 
    // from this function. Below code will give error if uncommented
    // meal.seasonal_fruit = String::from("blueberries");

    // accessing enums from other modules
    // Appetizer enum from back_of_house module
    // since Appetizer enum is public can be accessed as shown below
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


}

//parent function to be accessed inside fix_inco.. function below
fn customer_order() {

}

mod back_of_house {
    fn fix_incorrect_order() {
        //cook_order function defined below is a sibling function of fix_incorrect_order, can be accessed directly
        // in this function. Since it is a sibling, we can access here inside the function
        cook_order();

        
        //use super to go back up out of the back_of_house module to access a function in the
        // parent crate
        // customer_order() function is in the root of this library and is allowed to 
        // be accessed using the super keyword. It appears super here means the roof
        // of the crate
        super::customer_order();
    }

    fn cook_order() {}

    // struct private vs public how it works
    // Structs can be made public as shown below
    // each field of struct needs to be declared public if you need them to be
    // otherwise they are private by default
    // we can think of struct as being enclosed by {} almost like a module type so
    // maybe that's why Rust decided to make struct fields private be default
    pub struct Breakfast {
        // This pub and private models the behaviour of say customer can choose
        // toast type but fruit is chosen by the restaurant based on season
        // so toast should be public to be selected by customer but not seasonal_fruit
        // as customer cannot chose that anyway
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        //we need this below function to create a Breakfast struct from other modules. If we do not
        // have this kind of public function, we cannot create a Breakfast struct because one of its
        // field is private and we cannot access it directly. But below code works as
        // it accesses the fields from with in the struct
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    //Enum public vs private
    // if enum is public, then all variants of the enum are public as well.
    // The way enum works it does not make sense to have individual private and public variants
    // as enum variants represent different states of the enum
    pub enum Appetizer {
        Soup,
        Salad
    }
}


// "use" keyword to bring functions and modules into scope
//"use" keyword to bring modules and functions into the current scope once and then use
// the module functions directly without having to place the whole path all the time
// below hosting and serving are now in the scope
// absolute path and "use"
// note scope is brought only to the parent not the underlying function. This is done to keep
// the code readable
use crate::front_of_house::hosting;

// syntax to use relative path to bring to scope using "use" keyword
// relative path using "self"
use self::front_of_house::serving;

pub fn testing_use_keyword() {
    hosting::add_to_waitlist();
    //"use" keyword brought module functions into scope to use here
    // serving module is in the scope right now
    serving::take_order();

}

// convention to bring structs from other modules is by calling the path all the way to
// the struct rather than stopping at the parent. HashMap is a struct.
// standard library std comes shipped with Rust so we need not add this as dependency in
// Cargo.toml file. It is available directly with the use keyword
use std::collections::HashMap;

fn main() {
    // can use the HashMap struct 
    let mut map = HashMap::new();
    map.insert(1,2);

}

//Bringing two module with same name functions into scope
// below code is commented to prevent errors. But the code is to illustrate
// the point that "Result" is available from both fmt and io parents
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {

// }

// fn function2() -> io::Result {

// }

// "as" keyword to rename a function locally
use std::fmt::Result;
// rename the result function to IoResult locally to avoid name collusions
use std::io::Result as IoResult;


//re-exporting a imported variable in our scope
// since below was already defined else where the below code gives error but
// here use of "pub" keyword makes the hosting variable available to other code
// using our library. 
// pub use crate::front_of_house::hosting;

//use of nested paths to bring multiple variables from one library
// uput common path in the front and put all variations of the path in curly brackets
// use the below in lieu of bringing each variable into scope on separate lines.
use std::{cmp::Ordering, io};

//merging common paths and use of self
// self below is = std::io below
use std::io::{self, Write};

//glob operator - brings all public items in the path to scope
// glob operator = * - represents all items
use std::collections::*;
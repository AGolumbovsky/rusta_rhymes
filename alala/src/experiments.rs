/*
#![allow(dead_code)]
#![allow(unused_variable)]
*/

use std::mem;
struct Point {
    x: f64,
    y: f64
}

pub fn experimental_hello() {
    println!("This hello is experimental, accept at your own risk");
}

pub fn one_more() {
    println!("this is one more function that doesn't do much");
    for i in 1..11 {
        if i%2 != 0 { continue; }
        println!("i equols {}", i);
    }
    for (pos, i) in (30..41).enumerate() {
        println!("some shit and {}", i);
    }
}

pub fn match_stuff() {
    let country_code = 4454;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        1...999 => "one of many",
        _ => "fuck off"
    };
    println!("the country with code {} is {}", country_code, country);
}


/*

// engine x what is
// sebasmagri on github

// rocket stuff

*/

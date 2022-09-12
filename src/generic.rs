/*
 * File: generic.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::generic::struct_important_excerpt::ImportantExcerpt;

mod struct_important_excerpt;

pub(crate) fn generic() {
    call_generic_lifetime_0();
    call_generic_lifetime_1();
    //call_generic_lifetime_2();
    call_generic_lifetime_3();
    //call_generic_lifetime_4();
    call_generic_lifetime_5();
    generic_struct();
}

fn generic_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("ERROR: Could not find novel.");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn call_generic_lifetime_5() {
    let result = generic_lifetime_5();
    println!("The longest string is: {}", result);
}
// Returns owned value and transfers ownership -> valid

fn generic_lifetime_5() -> String {
    String::from("really long string")
}

/*
fn call_generic_lifetime_4() {
    let result = generic_lifetime_4();
    println!("The longest string is: {}", result);
}

// Cannot return reference to local variable `result` returns a reference to data
// owned by the current function
fn generic_lifetime_4<'a>() -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
 */

fn call_generic_lifetime_3() {
    let one = String::from("one");
    let result;
    {
        let two = String::from("two_two");
        result = generic_lifetime_3(one.as_str(), two.as_str());
    }
    println!("The longest string is: {}", result);
}

fn generic_lifetime_3<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}
// IN THIS CASE IT IS THE LIFETIME OF 'one' and 'two' -> 'two', but 'two' is no longer valid when
// 'result' is needed -> `two` does not live long enough
/*
fn call_generic_lifetime_2() {
    let one = String::from("one");
    let result;
    {
        let two = String::from("two_two");
        result = generic_lifetime_0(one.as_str(), two.as_str());
    }
    println!("The longest string is: {}", result);
}
 */
// IN THIS CASE IT IS THE LIFETIME OF 'one' and 'two' -> 'two'

fn call_generic_lifetime_1() {
    let one = String::from("one");
    {
        let two = String::from("two_two");
        let result = generic_lifetime_0(one.as_str(), two.as_str());
        println!("The longest string is: {}", result);
    }
}
/*
 * Always starts with "'" -> start with 'a to 'z
 * Creates relationship between all variables with the lifetime
 * Lifetime of return value will be the same as smallest lifetime
 * IN THIS CASE IT IS THE LIFETIME OF 'one' and 'two' -> 'one'
 * &i32        -> reference
 * &'a i32     -> reference with explicit lifetime
 * &'a mut i32 -> mut reference with explicit lifetime
 */

fn call_generic_lifetime_0() {
    let one = String::from("one");
    let two = String::from("two_two");
    let result = generic_lifetime_0(one.as_str(), two.as_str());
    println!("The longest string is: {}", result);
}

fn generic_lifetime_0<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

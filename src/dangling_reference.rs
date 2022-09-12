/*
 * File: dangling_reference.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn dangling_reference() {
    //dangling_reference_0();
    non_dangling_reference();
    //call_dangling_reference_1();
}

/*
fn call_dangling_reference_1() {
    let one = String::from("one");
    let two = String::from("two_two");
    let result = dangling_reference_1(one.as_str(), two.as_str());
    println!("The longest string is: {}", result);
}
// missing lifetime specifier - expected named lifetime parameter (-> &str)

fn dangling_reference_1(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
 */

fn non_dangling_reference() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}
// `x` does not live long enough
/*
fn dangling_reference_0() {
    let r;
    {
        let x = 5;
        r = &x
    }
    println!("r: {}", r);
}
 */

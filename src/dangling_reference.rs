/*
 * lifetimes is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/lifetimes/blob/main/LICENSE
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

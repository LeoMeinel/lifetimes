/*
 * File: combination.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::fmt::Display;

pub(crate) fn combination() {
    println!(
        "{}",
        longest_with_an_announcement("This is string x", "This is string y", "Important Message!")
    );
}
// We can not do automatic elision because of Rule 2. and 3.

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

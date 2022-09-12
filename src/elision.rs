/*
 * File: elision.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * 1. Each parameter that is a reference gets its own lifetime parameter
 * 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output
 * lifetime parameters.
 * 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self the
 * lifetime of self is assigned to all output lifetime parameters.
 * -> meaning that if there are multiple input lifetimes and none of them is &self -> no elision
 */

use crate::elision::struct_important_excerpt::ImportantExcerpt;

mod struct_important_excerpt;

pub(crate) fn elision() {
    //no_elision_first_word();
    println!(
        "The first word is: {}",
        elision_first_word("This is a sentence")
    );
    elision_struct();
}

fn elision_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("ERROR: Could not find novel.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.return_part("This is an announcement"));
}

fn elision_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
// Explicit lifetimes given in parameter types where they could be elided
/*
fn no_elision_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
 */

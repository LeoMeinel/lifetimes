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

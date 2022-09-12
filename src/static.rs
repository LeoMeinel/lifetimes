/*
 * File: static.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * static means that the lifetime will be as long as the program runs.
 * All string literals have a static lifetime.
 */

pub(crate) fn r#static() {
    println!("The static string literal is: {}", static_str());
}

fn static_str() -> &'static str {
    let s: &'static str = "I have a static lifetime.";
    s
}

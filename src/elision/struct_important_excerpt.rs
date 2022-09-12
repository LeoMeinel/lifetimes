/*
 * File: struct_important_excerpt.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) struct ImportantExcerpt<'a> {
    pub(crate) part: &'a str,
}
// lifetimes are a part of traits -> in impl you need to use them as well
// output lifetimes are same as &self because of Rule 3.

impl<'a> ImportantExcerpt<'a> {
    pub(crate) fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

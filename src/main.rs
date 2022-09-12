/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::combination::combination;
use crate::dangling_reference::dangling_reference;
use crate::elision::elision;
use crate::generic::generic;
use crate::r#static::r#static;

mod combination;
mod dangling_reference;
mod elision;
mod generic;
mod r#static;

fn main() {
    dangling_reference();
    generic();
    r#static();
    elision();
    combination();
}

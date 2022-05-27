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

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
 * along with this program. If not, see https://github.com/LeoMeinel/lifetimes/blob/main/LICENSE
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

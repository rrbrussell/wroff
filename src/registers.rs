/*
Copyright (C) 2024 Robert R. Russell

This program is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; version 2.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this
program; if not, write to the Free Software Foundation, Inc., 51 Franklin Street,
Fifth Floor, Boston, MA 02110-1301, USA.
*/

//! Documentation for wroff(lib)/registers

use std::collections::HashMap;

/// The list of builtin registers.
static BUILTIN_REGISTER_NAMES: [&str; 41] = [
	"%", "c.", "ct", "dl", "dn", "dw", "dy", "hp", "ln", "mo", "nl", "sb", "st", "yr", ".$", ".A",
	".H", ".L", ".P", ".T", ".V", ".a", ".c", ".d", ".f", ".h", ".i", ".j", ".k", ".l", ".n", ".o",
	".p", ".s", ".t", ".u", ".v", ".w", ".x", ".y", ".z",
];

/// Custom enum for register names.
#[derive(Eq, Hash, PartialEq)]
pub enum RegisterName<'a> {
	Builtin(&'a str),
	User(&'a str),
}

/// Storage for all of the registers
pub struct RegisterStorage<'a> {
	storage: HashMap<RegisterName<'a>, u32>,
}

/// Stuff that RegisterStorage implements.
impl RegisterStorage<'_> {
	/// Create and initalize a new allocated area for the registers.
	pub fn new() -> Self {
		let mut storage: HashMap<RegisterName<'_>, u32> =
			HashMap::with_capacity(BUILTIN_REGISTER_NAMES.len());
		for i in BUILTIN_REGISTER_NAMES {
			storage.insert(RegisterName::Builtin(i), 0u32);
		}
		return RegisterStorage { storage: storage };
	}
}

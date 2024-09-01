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

//! Documentation for wroff(lib)

#![cfg_attr(debug_assertions, allow(dead_code))]

/// Information about registers.
pub mod registers;

/// The package version
static VERSION: &str = env!("CARGO_PKG_VERSION");
/// The package name
static NAME: &str = env!("CARGO_PKG_NAME");

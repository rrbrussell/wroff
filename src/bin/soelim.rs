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

//! A replacement for the soelim program from either heirloom troff or GNU roff.
//!
//! This program is designed as a general replacement for both of those soilem
//! programs. It doesn't implement every command line option and operating mode
//! of the original programs.

use clap::Parser;
use std::ffi::OsString;

/// This is holds information collected from the command line.
#[derive(Parser)]
#[command(name = "soilem")]
#[command(version = "0.0.1")]
#[command(about = "Source request eliminator", long_about = None)]
struct CliArguments {
	/// Add a directory to the include path for sourced files.
	/// The current directory is appended to this list be default.
	/// If you want it higher in the search list pass '-I .' in the appropriate
	/// position in the list.
	#[arg(short = 'I', value_name = "DIR")]
	input_directories: Vec<OsString>,
	/// Raw mode does not modify the file name and line number of input file.
	/// This does not make the error handling easier.
	#[arg(short, help = "Do not update file names and line numbers.")]
	raw: bool,
	/// Tex mode uses Tex comments to track input file name changes and line
	/// number updates.
	#[arg(
		short,
		help = "Use Tex comments to track input files and line numbers."
	)]
	tex: bool,
	/// Do not require spaces between the .so request and the file name.
	#[arg(short)]
	concated: bool,
}

/// Our main function
fn main() {
	let _cli = CliArguments::parse();
	println!("Hello, world!");
}

// Copyright (c) 2018 Lawrence Dark
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `tgcli` runtime
use clap::App;
use error::Result;
use std::io::{self, Write};

/// CLI Runtime
pub fn run() -> Result<i32> {
    let _matches = App::new(env!("CARGO_PKG_NAME"))
                      .version(env!("CARGO_PKG_VERSION"))
                      .author(env!("CARGO_PKG_AUTHORS"))
                      .about("Controls tonal glyph tracker from the command")
                      .get_matches();
    writeln!(io::stdout(), "Hello, Rustaceans!")?;
    Ok(0)
}
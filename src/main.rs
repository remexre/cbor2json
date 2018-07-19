extern crate failure;
extern crate serde_cbor;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

use failure::Error;
use serde_cbor::Value;
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();

    let func = if options.reverse {
        json2cbor
    } else {
        cbor2json
    };

    if let Err(err) = func(stdin(), stdout()) {
        eprintln!("{}", err);
        exit(1)
    }
}

fn cbor2json(read: impl Read, mut write: impl Write) -> Result<(), Error> {
    let value: Value = serde_cbor::from_reader(read)?;
    serde_json::to_writer(&mut write, &value)?;
    Ok(())
}

fn json2cbor(read: impl Read, mut write: impl Write) -> Result<(), Error> {
    let value: Value = serde_json::from_reader(read)?;
    serde_cbor::to_writer(&mut write, &value)?;
    Ok(())
}

#[derive(StructOpt)]
#[structopt(raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
struct Options {
    /// Convert JSON to CBOR instead.
    #[structopt(short = "r")]
    pub reverse: bool,
}

use core::panic;
use std::{
    io::{IsTerminal, Read},
    process::exit,
};

use clap::{CommandFactory, Parser};

/// Simple program for generating QR codes
#[derive(clap::Parser)]
struct Args {
    /// Url or other data
    data: Option<String>,

    /// File to write qrcd to
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut data;
    if let Some(d) = args.data {
        // create qrcd from argument
        data = d
    } else if !std::io::stdin().is_terminal() {
        // create qrcd from pipe
        data = String::new();
        _ = std::io::stdin().read_to_string(&mut data).unwrap();
    } else {
        // print help if no arguments or pipe
        Args::command().print_help().unwrap();
        exit(0);
    }

    // start converting data into qrcode
    let qr_code = match qrcode::QrCode::new(data) {
        Ok(qr_code) => qr_code,
        Err(err) => panic!("{err}"),
    };
    let mut renderer = qr_code.render();
    let text = renderer.dark_color("\x1b[;40m  \x1b[m");

    // write to file or print to terminal
    if let Some(path) = args.path {
        let image = qr_code.render::<image::Luma<u8>>().build();

        // print to terminal if writing to file fails
        if let Err(err) = image.save(path) {
            println!("{err}");

            // make qrcode red on fail
            println!("{}", text.light_color("\x1b[;41m  \x1b[m").build());
        }
    } else {
        println!("{}", text.light_color("\x1b[;47m  \x1b[m").build());
    }
}

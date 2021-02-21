
use std::io::{self, Cursor};
use std::env;
use libflate::zlib::{Decoder, Encoder};

fn main() -> io::Result<()> {

    let args : Vec<String> = env::args().collect();
    let mut in_stream = io::stdin();

    match args.len() == 2 && args[1] == "-d" {
        true => {
            let mut decoder = Decoder::new(in_stream).unwrap();
            io::copy(&mut decoder, &mut io::stdout())?;
        },
        false => {
            let mut encoder = Encoder::new(Vec::new()).unwrap();
            io::copy(&mut in_stream, &mut encoder).unwrap();

            match encoder.finish().into_result() {
                Ok(v) => {
                    io::copy(&mut Cursor::new(v), &mut io::stdout())?;
                },
                Err(e) => println!("encode error: {}", e)
            }
        }
    }

    Ok(())
}

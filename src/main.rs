use std::io::{self, Write};
use smartcard::logic::Context;
use smartcard::logic::reader::Reader;
use smartcard::parameters::{ShareMode, Protocol};
use smartcard::errors::*;

use std::sync::Arc;

fn get_context() -> Result<Context> {

    // First we create the resource manager context. I think of it as 'the driver'.
    let context = Context::establish_context_auto()?;

    Ok(context)
}

fn get_reader(context: &Arc<Context>) -> Result<Reader> {

    // The context allows to list all available card readers.
    let mut readers = context.list_readers()?;

    println!("{} readers found:", readers.len());
    for r in readers.iter() {
        println!("reader name: {}", r.get_name());
    }

    // Let's get the first reader.
    // TODO: select the reader when there are more than one.
    let reader = readers.pop().ok_or(format!("no readers found"))?;

    Ok(reader)
}

fn str_to_apdu(input: &str) -> Result<Vec<u8>> {
    if input.len() % 2 != 0 {
        return Err("Input string length must be even.".into());
    }

    let mut result = Vec::new();

    for i in 0..input.len() / 2 {
        let byte_str = &input[i * 2..(i * 2) + 2];
        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
            result.push(byte);
        } else {
            return Err("invalid APDU".into());
        }
    }

    Ok(result)
}

fn main() {

    // we use an Arc<Context> so that even if we
    // drop(context) the context still exists while the card is alive.
    let context = match get_context() {
        Ok(context) => Arc::new(context),
        Err(e) => { println!("Get context error: {}.", e.to_string()); return; }
    };

    let reader = match get_reader(&context) {
        Ok(reader) => reader,
        Err(e) => { println!("Get reader error: {}.", e.to_string()); return; }
    };

    // From the reader, we can connect to its smartcard this way.
    let card = match reader.connect_to(context, ShareMode::Auto, Protocol::Auto) {
        Ok(card) => card,
        Err(e) => { println!("Get card error: {}.", e.to_string()); return; }
    };

    loop {
        let mut input = String::new();
        print!(">> ");
        let _ = io::stdout().flush();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd: &str = &input.trim();
                if cmd == "quit" {
                    break;
                } else if cmd == "help" {
                    println!("TODO: print usage.");
                } else {
                    let apdu = str_to_apdu(&cmd.split(' ').collect::<Vec<_>>().join("")).unwrap();

                    println!("CMD: {:02X?}", apdu);

                    // 256 is the maximum size of the expected response.
                    match card.send_raw_command(&apdu, 256) {
                        Ok(answer) => {
                            println!("RES: {:02X?}", answer);
                        }
                        Err(e) => {
                            println!("ERR: {}.", e.to_string());
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

// extern crate smartcard;

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
    let reader = readers.pop().ok_or(format!("no readers found"))?;

    Ok(reader)
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

    // Now that we have a card available, we can send commands to it.
    // select app on my card
    let cmd_vec = vec![0x00, 0xA4, 0x04, 0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00];

    // 256 is the maximum size of the expected answer
    match card.send_raw_command(&cmd_vec, 256) {
        Ok(answer) => {
            println!("Answer: {:?}", answer);
        }
        Err(e) => {
            println!("Command error: {}.", e.to_string());
        }
    }
}

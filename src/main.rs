// extern crate smartcard;

use smartcard::logic::Context;
use smartcard::parameters::{ShareMode, Protocol};
use smartcard::errors::*;

use std::sync::Arc;

fn run() -> Result<()> {
    //First we create the resource manager context. I think of it as 'the driver'.
    let context = Arc::new(Context::establish_context_auto()?);

    //The context allows to list all available card readers.
    let mut readers = context.list_readers()?;

    println!("{} readers found:", readers.len());
    for r in readers.iter() {
        println!("- {}", r.get_name());
    }

    //Let's get the first reader.
    let reader = readers.pop().ok_or(format!("no readers found"))?;

    //From the reader, we can connect to its smartcard this way.
    let card = reader.connect_to(context, ShareMode::Auto, Protocol::Auto)?;
    //we use an Arc<Context> so that even if we
    //drop(context)
    //the context still exists while the card is alive

    //Now that we have a card available, we can send commands to it.
    //select app on my card
    let cmd_vec = vec![0x00, 0xA4, 0x04, 0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00];
    let answer = card.send_raw_command(&cmd_vec, 256)?; //256 is the maximum size of the expected answer

    println!("Answer: {:?}", answer);//I get 0x90 0x00, perfect!
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => println!("An error occured: {}.", e.to_string())
    }
}

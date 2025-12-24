use hagja::*;
// use std::fs::File;
// use std::sync::{Arc, Mutex};

fn main() {
    let id = "Examples/Main";
    // let file = File::create("./log.txt").ok().map(|f| Arc::new(Mutex::new(f)));
    let logger: Hagja = Hagja::new(id, LogLevel::Trace, false, None);

    set_default_logger(logger).expect("Error, unable to set default logger.");

    info!("Starting...");
    debug!("Executing...");
    warn!("Unable to get specific resource.");
    error!("Cannot initialize.");
    fatal!("Corruption detected, exiting.");
    trace!("Cleaning resource cache 4."); 
}
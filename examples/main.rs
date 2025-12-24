use hagja::*;

fn main() {
    let id = "Examples/Main";
    let logger: Hagja = Hagja::new(id, LogLevel::Trace);

    set_default_logger(logger).expect("Error, unable to set default logger.");

    info!("Starting...");
    debug!("Executing...");
    warn!("Unable to get specific resource.");
    error!("Cannot initialize.");
    fatal!("Corruption detected, exiting.");
    trace!("Cleaning resource cache 3."); 
}
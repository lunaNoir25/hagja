use hagja::Hagja;

fn main() {
    let id = "Examples/Main";
    let logger: Hagja = Hagja::new(id);

    logger.info("Starting...");
    logger.debug("Executing...");
    logger.warn("Unable to get specific resource.");
    logger.error("Cannot intialize.");
    logger.fatal("Corruption detected, exiting.");
    logger.trace("Cleaning resource cache 3.");
}
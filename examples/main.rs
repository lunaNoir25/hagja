use hagja::Hagja;

fn main() {
    let id = "Examples/Main";
    let logger: Hagja = Hagja::new(id);

    logger.info("Starting...");
}
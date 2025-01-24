#[cfg(not(feature = "offline"))]
mod fetcher;
fn main() {
    #[cfg(not(feature = "offline"))]
    {
        fetcher::main();
    }
}

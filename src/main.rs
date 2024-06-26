// remove this later!
#![allow(dead_code)]

mod move_generation;
mod nnue;
mod search;
mod uci;
mod util_macros;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        if arg == "bench" {
            search::bench::run_bench();
            return;
        }
    }

    let mut uci_handler = uci::uci_handler::UciHandler::new();

    loop {
        uci_handler.respond();
    }
}

use std::process;

fn main() {
    if !dmt::run_dmt() {
        process::exit(1);
    }
}

use std::process;

fn main() {
    if !dmt_cli::run_dmt() {
        process::exit(1);
    }
}

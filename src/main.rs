mod domain;

use domain::blockchain::{Blockchain};
use domain::cli::{CLI};

fn main() {
    let mut cli = CLI::new();
    cli.run();
}

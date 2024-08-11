mod domain;

use domain::blockchain::{Blockchain};
use domain::cli::{CLI};

fn main() {
    let bc = Blockchain::new().expect("Failed to create blockchain");
    let mut cli = CLI::new(bc);
    cli.run();
}

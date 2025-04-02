mod database;
mod ui;
mod util;

use ui::cli::start_cli;

fn main() {
    let result = start_cli();
    println!("{result:?}");
}

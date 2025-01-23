use crate::args::parse_args;

mod args;
mod commands;
mod daemon;
mod weather;

mod display;

fn main() {
    parse_args();
}

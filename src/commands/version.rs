use crate::print_info;

pub fn print_version() {
    print_info!("You are running v{} of nwsd", env!("CARGO_PKG_VERSION"));
}

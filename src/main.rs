extern crate nix;

use nix::sys::socket;
use nix::sys::socket::AddressFamily;
use nix::sys::socket::SockType;

fn main() {
    socket::socket(AddressFamily::Packet, SockType::Raw);
    println!("Hello, world!");
}

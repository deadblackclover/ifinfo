extern crate interfaces2 as interfaces;

use interfaces::Address;
use interfaces::Interface;
use interfaces::Kind;

fn main() {
    let ifaces = Interface::get_all().unwrap();
    for iface in &ifaces {
        let indent = " ".repeat(8);
        print!("{}:", iface.name);
        if iface.is_up() {
            print!(" <UP>");
        } else {
            print!(" <DOWN>");
        }

        println!("  mtu {}", iface.get_mtu().unwrap());

        for addr in &iface.addresses {
            match addr.kind {
                Kind::Packet => {}
                Kind::Ipv4 => {
                    print!("{}", indent);
                    print!("inet ");
                    pr(*addr);
                }
                Kind::Ipv6 => {
                    print!("{}", indent);
                    print!("inet6 ");
                    pr(*addr);
                }
                Kind::Link => {}
                _ => {}
            }
        }

        print!("{}", indent);
        println!("ether {}", iface.hardware_addr().unwrap().as_string());

        println!(" ");
    }
}

fn pr(addr: Address) {
    if addr.addr.is_some() {
        print!("{}", addr.addr.unwrap().ip());
    }
    if addr.mask.is_some() {
        print!("  netmask {}", addr.mask.unwrap().ip());
    }
    if addr.hop.is_some() {
        print!("  broadcast {}", addr.hop.unwrap());
    }
    println!("");
}

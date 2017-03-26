#![feature(ip_addr, raw)]

extern crate pnet;

use std::net::{IpAddr, Ipv4Addr};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::transport_channel;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

#[allow(dead_code)]
struct IcmpPacket {
    typ: u8,
    code: u8,
    checksum: u16,
    roh: u32,
}

impl IcmpPacket {
    #[allow(dead_code)]
    fn new(typ: u8, code: u8, roh: u32) -> Option<Self> {
        let mut checksum = 0u16;
        checksum += typ as u16;
        checksum += code as u16;
        checksum += roh as u16;
        checksum += (roh << 8) as u16;
        checksum += (roh << 16) as u16;
        checksum += (roh << 24) as u16;

        let pac = IcmpPacket {
        typ: typ,
        code: code,
        checksum: checksum,
        roh: roh,
        };

        Some(pac)
    }
}

impl Packet for IcmpPacket {
    fn packet(&self) -> &[u8] {
        unsafe {
        ::std::mem::transmute(::std::raw::Slice {
            data: self as *const _ as *const u8,
            len: ::std::mem::size_of::<Self>(),
        })
        }
    }

    fn payload(&self) -> &[u8] {
        self.packet()  // TODO: Add the data section.
    }
}

fn main() {
    let icmp = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, _) = match transport_channel(2048, icmp) {
        Ok((tx, rx)) => (tx, rx),
        Err(err) => panic!("{:?}", err),
    };

    let packet = IcmpPacket {
        typ: 8,
        code: 0,
        checksum: 0xdcdc,
        roh: 0x0100221b,
    };

    let ip_address = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));

    match tx.send_to(packet, ip_address) {
        Ok(_) => println!("Sent"),
        Err(e) => println!("{}", e),
    }
}

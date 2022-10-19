use clap::{value_parser, Arg, Command};
use pcap::Capture;
use std::path::Path;
use std::collections::HashMap;

use std::mem;

mod bookmanager;
mod enums;
mod messages;
mod utils;

use bookmanager::{BookManager, Order, OrderManager};

use messages::{Message, PacketHeader, Body};
use utils::{
    as_u16, PACKET_HEADER_LENGTH, PACKET_HEADER_OFFSET, UDP_DEST_PORT, UDP_DEST_PORT_OFFSET,
};

fn main() {
    let matches = Command::new("bookbuilder")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("PCAP FILE")
                .help("Sets a pcap file path as input.")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("symbol")
                .short('s')
                .long("symbol")
                .value_name("SYMBOL")
                .help("Sets stock symbol.")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("book_depth")
                .short('d')
                .long("book-depth")
                .value_name("DEPTH")
                .help("Sets book depth.")
                .value_parser(value_parser!(u32))
                .num_args(1)
                .default_value("10"),
        )
        .arg(
            Arg::new("sequence_number")
                .short('n')
                .long("sequence-number")
                .value_name("SEQNUM")
                .help("Sets sequence number to stop.")
                .value_parser(value_parser!(u64))
                .num_args(1)
                .required(true),
        )
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();
    let symbol = format!("{:8}", matches.get_one::<String>("symbol").unwrap()).to_uppercase();
    let depth = matches.get_one::<u32>("book_depth").unwrap();
    let seqnum = matches.get_one::<u64>("sequence_number").unwrap();
    let mut listing = HashMap::<u16, String>::new();
    let mut order_manager = OrderManager::new();
    let mut book_manager = BookManager::new();

    let mut capture = Capture::from_file(Path::new(file)).unwrap();

    while let Ok(packet) = capture.next_packet() {
        let udp_dest_port = as_u16(&packet.data[UDP_DEST_PORT_OFFSET..UDP_DEST_PORT_OFFSET + 2]);
        if udp_dest_port == UDP_DEST_PORT {
            let packet_header = PacketHeader::new(&packet.data[PACKET_HEADER_OFFSET..]);
            if packet_header.sequence_number > *seqnum {
                break;
            }
            let mut offset = PACKET_HEADER_OFFSET + PACKET_HEADER_LENGTH;

            //println!("------------------");
            //println!("{:?}", packet.data);
            let mut msg_count = packet_header.message_count;
            //println!("[{}] count: {}", packet_header.sequence_number, msg_count);

            while msg_count > 0 {
                let msg = Message::new(&packet.data[offset..]);
                //println!("{:?}", &msg);
                if msg.body != Body::None {
                    let order = Order::new(&msg, &mut listing);
                    order_manager.process(&order, &mut book_manager);
                    //println!("{:?}", book_manager);
                }
                msg_count -= 1;
                offset += msg.length as usize + mem::size_of_val(&msg.length);
            }
        }
    }

    let associate_stock_locate = listing.iter().find_map(|(key, val)| if *val == *symbol { Some(key.clone()) } else { None });

    if let Some(sl) = associate_stock_locate {
        println!("Book (depth: {}) : {}", depth, symbol);
        book_manager.display_book(sl, *depth as usize);
    }
}

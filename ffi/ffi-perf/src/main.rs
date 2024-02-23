use std::{net::{IpAddr, Ipv4Addr}, str::FromStr, collections::HashMap};


#[derive(Debug, Clone, Hash)]
pub struct PacketInfo<'a> {
    pub src: IpAddr,
    pub dst: IpAddr,
    pub protocol: u16,
    pub sport: Option<u16>,
    pub dport: Option<u16>,
    pub remain: &'a [u8]
}


fn main() {
    // Since this is a test example, use unwrap directly
    let remain = b"jankincai";
    let pktinfo = PacketInfo {
        src: IpAddr::V4(Ipv4Addr::from_str("192.168.12.123").unwrap()),
        dst: IpAddr::V4(Ipv4Addr::from_str("192.168.12.124").unwrap()),
        protocol: 6,
        sport: Some(4567),
        dport: Some(8888),
        remain,
    };
    let pktinfo_ptr = Box::into_raw(Box::new(pktinfo));

    let mut hashmap: HashMap<u16, PacketInfo> = HashMap::new();

    let t1 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    for _i in 0..=1000000 {
        let value = unsafe { &mut *pktinfo_ptr  };
        // hashmap.insert(1, value.clone());
    }
    let t2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    println!("time(s): {}", t2 - t1);

    let t1 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    for _i in 0..=1000000 {
        let value = unsafe { std::ptr::read(pktinfo_ptr) };
        // hashmap.insert(1, value);
    }
    let t2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    println!("time(s): {}", t2 - t1);

    let t1 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    for _i in 0..=1000000 {
        let value = unsafe { &mut *pktinfo_ptr  }.clone();
        // hashmap.insert(1, value);
    }
    let t2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    println!("time(s): {}", t2 - t1);

    // time(s): 0.010022163391113281
    // time(s): 0.011680364608764648
    // time(s): 0.024011850357055664
}

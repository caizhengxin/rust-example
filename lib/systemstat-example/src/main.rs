use systemstat::{Platform, System};


fn main() {
    let sys = System::new();

    match sys.networks() {
        Ok(netifs) => {
            for iface in netifs.values() {
                // println!("name: {:?} {:?}", iface, sys.network_stats(&iface.name));

                match sys.network_stats(&iface.name) {
                    Ok(network_stats) => {
                        println!("rx_bytes: {} tx_bytes: {} {:?}", network_stats.rx_bytes.as_u64() / 1024, network_stats.tx_bytes.as_u64() / 1024, network_stats);
                    },
                    Err(_) => {},
                }    
            }
        }
        Err(_) => {},
    }
}

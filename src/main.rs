//use std::os::unix::net::UnixDatagram;
use std::net::Ipv4Addr;
use tokio::net::UnixDatagram;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Ip {
    ip: String
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let path = "/root/test.sock";
    let client = reqwest::Client::new();


    let socket = match UnixDatagram::bind(path) {
        Ok(dg) => dg,
        Err(_) => {
            std::fs::remove_file(path)?;
            UnixDatagram::bind(path)?
        },
    };
    
    loop {
        let mut buf = [0; 100];
        let count = socket.recv_from(&mut buf).await?.0;
    
        if count != 23 {
            continue
        }

        // only allow connect logging and IPv4 type 
        if buf[0] == 0x2 || buf[1] != 0x1 {
            continue
        }

        let ip_bytes: [u8; 4] = buf[5..9].try_into().unwrap();
        let ip: Ipv4Addr = Ipv4Addr::from(ip_bytes);

        let params = Ip { ip: ip.to_string() };

        match client.post("http://localhost:5000/api/v1/ips")
            .json(&params)
            .send()
            .await {
                Ok(_) => println!("Successfully whitelisted {:?}.", ip),
                Err(_) => println!("There was an error whitelisting {:?}.", ip),    
            };
 
    }
}

//use std::os::unix::net::UnixDatagram;
use std::net::Ipv4Addr;
use tokio::net::UnixDatagram;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Command {
    #[structopt(default_value = "logger.sock")]
    sockpath: String,
    #[structopt(default_value = "http://localhost:3000")]
    api_url: String,
    #[structopt(default_value = "")]
    api_key: String,
}

#[derive(Serialize, Deserialize)]
struct Ip {
    ip: String
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let opts = Command::from_args();
    let client = reqwest::Client::new();

    let socket = match UnixDatagram::bind(&opts.sockpath) {
        Ok(dg) => dg,
        Err(_) => {
            std::fs::remove_file(&opts.sockpath)?;
            UnixDatagram::bind(&opts.sockpath)?
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

        match client.post(format!("{}/api/v1/whitelist", opts.api_url))
            .json(&params)
            .bearer_auth(&opts.api_key)
            .send()
            .await {
                Ok(response) => println!("Successfully whitelisted {:?}. Status code {:?}", ip, response.status()),
                Err(_) => println!("There was an error whitelisting {:?}.", ip),    
            };
 
    }
}

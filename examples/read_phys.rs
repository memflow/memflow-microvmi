use std::env;
use std::time::Instant;

use log::{info, Level};

use memflow::*;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(Level::Debug.to_level_filter())
        .init()
        .unwrap();

    let args: Vec<String> = env::args().collect();
    let conn_args = if args.len() > 1 {
        ConnectorArgs::parse(&args[1]).expect("unable to parse arguments")
    } else {
        ConnectorArgs::new()
    };

    // TODO: parse command-line args
    let mut conn = memflow_microvmi::create_connector(&conn_args)
        .expect("unable to initialize memflow_microvmi");

    let mut mem = vec![0; 8];
    conn.phys_read_raw_into(Address::from(0x1000).into(), &mut mem)
        .unwrap();
    info!("Received memory: {:?}", mem);

    let start = Instant::now();
    let mut counter = 0;
    loop {
        let mut buf = vec![0; 0x1000];
        conn.phys_read_raw_into(Address::from(0x1000).into(), &mut buf)
            .unwrap();

        counter += 1;
        if (counter % 10000) == 0 {
            let elapsed = start.elapsed().as_millis() as f64;
            if elapsed > 0.0 {
                info!("{} reads/sec", (f64::from(counter)) / elapsed * 1000.0);
                info!("{} ms/read", elapsed / (f64::from(counter)));
            }
        }
    }
}

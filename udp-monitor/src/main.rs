use std::net::Ipv4Addr;

use anyhow::Context as _;
use aya::{
    maps::HashMap,
    programs::{Xdp, XdpFlags},
};
use clap::Parser;
#[rustfmt::skip]
use log::{debug, warn};
use std::io::{self, Write};

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {ret}");
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/udp-monitor"
    )))?;
    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {e}");
    }
    let Opt { iface } = opt;
    let program: &mut Xdp = ebpf.program_mut("udp_monitor").unwrap().try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    let mut blocklist: HashMap<_, u32, u32> =
        HashMap::try_from(ebpf.map_mut("BLOCKLIST").unwrap())?;

    loop {
        print!("Enter an IP address: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        let Ok(ip_address) = input.parse::<Ipv4Addr>() else {
            continue;
        };
        let block_addr: u32 = ip_address.into();
        if blocklist.get(&block_addr, 0).is_ok() {
            println!("Removing IP address: {ip_address}");
            blocklist.remove(&block_addr)?;
        } else {
            println!("Adding IP address: {ip_address}");
            blocklist.insert(block_addr, 0, 0)?;
        }
    }
}

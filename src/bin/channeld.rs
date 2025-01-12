// LNP Node: node running lightning network protocol and generalized lightning
// channels.
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![recursion_limit = "256"]
// Coding conventions
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    dead_code,
    missing_docs
)]

//! Main executable for channeld: lightning node channel operations microservice

#[macro_use]
extern crate log;

use clap::Clap;
#[cfg(feature = "rgb")]
use std::convert::TryInto;

use lnp_node::channeld::{self, Opts};
use lnp_node::{Config, LogStyle};

fn main() {
    println!("channeld: lightning channel microservice");

    let mut opts = Opts::parse();
    trace!("Command-line arguments: {:?}", &opts);
    opts.process();
    trace!("Processed arguments: {:?}", &opts);

    let config: Config = opts.shared.clone().into();
    trace!("Daemon configuration: {:?}", &config);
    debug!("MSG RPC socket {}", &config.msg_endpoint);
    debug!("CTL RPC socket {}", &config.ctl_endpoint);

    #[cfg(feature = "rgb")]
    let rgb20_socket_addr = opts
        .rgb_opts
        .rgb20_socket
        .try_into()
        .expect("RPC socket must be a valid ZMQ local file socket");

    let node_id = opts.key_opts.local_node().node_id();
    info!("{}: {}", "Local node id".ended(), node_id.addr());

    /*
    use self::internal::ResultExt;
    let (config_from_file, _) =
        internal::Config::custom_args_and_optional_files(std::iter::empty::<
            &str,
        >())
        .unwrap_or_exit();
     */

    debug!("Starting runtime ...");
    channeld::run(
        config,
        opts.key_opts.local_node(),
        opts.channel_id,
        opts.shared.chain,
        #[cfg(feature = "rgb")]
        rgb20_socket_addr,
    )
    .expect("Error running channeld runtime");

    unreachable!()
}

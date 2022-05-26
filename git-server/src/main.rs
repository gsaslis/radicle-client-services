use std::path::PathBuf;
use std::{net, process};

use radicle_git_server as server;

use argh::FromArgs;

use shared::LogFmt;

/// Radicle Git Server.
#[derive(FromArgs)]
pub struct Options {
    /// listen on the following address for HTTP connections (default: 0.0.0.0:8778)
    #[argh(option, default = "std::net::SocketAddr::from(([0, 0, 0, 0], 8778))")]
    pub listen: net::SocketAddr,

    /// radicle root path, for key and git storage
    #[argh(option)]
    pub root: Option<PathBuf>,

    /// radicle encrypted key passphrase
    #[argh(option)]
    pub passphrase: Option<String>,

    /// TLS certificate path
    #[argh(option)]
    pub tls_cert: Option<PathBuf>,

    /// TLS key path
    #[argh(option)]
    pub tls_key: Option<PathBuf>,

    /// either "plain" or "gcp" (gcp available only when compiled-in)
    #[argh(option, default = "LogFmt::Plain")]
    pub log_format: LogFmt,

    /// service 'git-receive-pack' operations, eg. resulting from a `git push` (default: false)
    #[argh(switch)]
    pub git_receive_pack: bool,

    /// certificate nonce seed used to enable `push --signed`
    #[argh(option)]
    pub cert_nonce_seed: Option<String>,

    /// allow unauthorized keys, ignores gpg certificate verification
    #[argh(switch)]
    pub allow_unauthorized_keys: bool,
}

impl Options {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

impl From<Options> for server::Options {
    fn from(other: Options) -> Self {
        Self {
            root: other.root,
            passphrase: other.passphrase,
            tls_cert: other.tls_cert,
            tls_key: other.tls_key,
            listen: other.listen,
            git_receive_pack: other.git_receive_pack,
            cert_nonce_seed: other.cert_nonce_seed,
            allow_unauthorized_keys: other.allow_unauthorized_keys,
        }
    }
}

#[tokio::main]
async fn main() {
    let options = Options::from_env();

    shared::init_logger(options.log_format);
    tracing::info!("version {}-{}", env!("CARGO_PKG_VERSION"), env!("GIT_HEAD"));

    match server::run(options.into()).await {
        Ok(()) => {}
        Err(err) => {
            tracing::error!("Fatal: {:#}", err);
            process::exit(1);
        }
    }
}

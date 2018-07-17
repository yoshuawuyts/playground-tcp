#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
  #[structopt(flatten)]
  pub port: clap_port_flag::Port,
}

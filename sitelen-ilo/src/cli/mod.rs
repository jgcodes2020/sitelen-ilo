use std::sync::OnceLock;

#[derive(clap::Parser)]
#[command(name = "o-ilo")]
#[command(about = "Runs a sitelen ilo program.")]
pub struct Cli {
    pub input: String,
    #[arg(long = "log", help = "Sets the log level.", default_value_t = log::LevelFilter::Error)]
    pub log_level: log::LevelFilter,
}

impl Cli {
    /// Performs any initial setup that is independent of the compilation process.
    pub fn preinit_all(&self) {
        self.preinit_logger();
    }

    /// Sets up `env_logger` using the log level.
    fn preinit_logger(&self) {
        static LOGGER_INST: OnceLock<env_logger::Logger> = OnceLock::new();

        let logger = LOGGER_INST.get_or_init(|| {
            env_logger::builder()
                .parse_default_env()
                .filter_level(self.log_level)
                .build()
        });

        log::set_logger(logger).unwrap();
    }
}

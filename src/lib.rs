use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn logger(log_file_path: &str, level: LevelFilter) {

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("app::requests", LevelFilter::Info));

    if level.eq(&LevelFilter::Debug) || level.eq(&LevelFilter::Trace) {
        let stdout = ConsoleAppender::builder().build();

        log4rs::init_config(config.appender(Appender::builder().build("stdout", Box::new(stdout))).build(Root::builder().appender("stdout").build(level)).unwrap()).unwrap();
    } else {
        log4rs::init_config(config.build(Root::builder().appender("stdout").build(LevelFilter::Off)).unwrap()).unwrap();
    }





}

fn select_level(level: &str) -> LevelFilter {
    match level {
        "debug" => {LevelFilter::Debug}
        "info" => {LevelFilter::Info}
        "trace" => {LevelFilter::Trace}
        _ => {LevelFilter::Off}
    }
}
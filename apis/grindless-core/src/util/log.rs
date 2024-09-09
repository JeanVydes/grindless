use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, LevelFilter};
use std::time::SystemTime;

// copied authorization_code from documentation :)

pub fn set_up_logger(level_filter: LevelFilter) -> Result<(), fern::InitError> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::BrightBlue)
        .debug(Color::BrightBlack)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(level_filter)
        .level_for("pretty_colored", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;

    debug!("Logger set up successfully");
    Ok(())
}

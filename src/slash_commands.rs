use serenity_commands::{Command, CommandData, CommandOption};

#[derive(Debug, CommandData)]
pub enum Commands {
    /// Ping the bot.
    Ping,

    /// Echo a message.
    Echo {
        /// The message to echo.
        message: String,
    },

    /// Perform math operations.
    Math(MathCommand),

    /// one or two numbers.
    OneOrTwo(OneOrTwo),

    /// Miscaellaneaous commands.
    Misc(MiscCommands),
}

#[derive(Debug, Command)]
enum MathCommand {
    /// Add two numbers.
    Add {
        /// The first number.
        first: f64,

        /// The second number.
        second: f64,
    },

    /// Subtract two numbers.
    Subtract(SubtractCommandOption),
}

#[derive(Debug, CommandOption)]
struct SubtractCommandOption {
    /// The first number.
    first: f64,

    /// The second number.
    second: f64,
}

#[derive(Debug, Command)]
enum MiscCommands {
    /// Get the current time.
    Time,

    /// one or two numbers... inside misc!
    OneOrTwo(OneOrTwo),
    // /// deeper misc commands
    // Deeper(DeeperMiscCommands), DOES NOT COMPILE! nesting 3 levels deep is not supported by the
    // discord API, and thus this crate prevents it.
}

#[derive(Debug, Command)]
enum DeeperMiscCommands {
    /// how??
    How,
}

// usable at the top level or as a subcommand!
#[derive(Debug, Command, CommandOption)]
struct OneOrTwo {
    /// The first number.
    first: f64,

    /// The second number, optional.
    second: Option<f64>,
}
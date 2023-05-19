pub use create::create_sponsor;
pub use delete::delete;
pub use get::get_sponsor;
pub use get_all::get_all;
pub use healthcheck::get_health;
pub use search::search;
pub use update::update;
pub use whoami::whoami;

pub mod settings;

mod healthcheck;
mod create;
mod search;
mod delete;
mod whoami;
mod get;
mod get_all;
mod update;

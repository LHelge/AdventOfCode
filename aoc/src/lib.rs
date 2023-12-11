mod error;
pub use error::AoCError;

mod input;
pub use input::get_input;

mod pairs;
pub use pairs::Pairable;

mod permute;
pub use permute::Permutable;

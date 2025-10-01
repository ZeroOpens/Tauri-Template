pub mod anime;
pub mod dog_api;

// 统一导出命令
pub use anime::{get_multiple_anime_quotes, get_random_anime_quote};
pub use dog_api::get_random_dog_image;

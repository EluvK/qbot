pub type UserId = u64;

mod gpt;
mod manager;
mod message;
mod model;
mod role;
mod session;

pub use manager::ChatManager;

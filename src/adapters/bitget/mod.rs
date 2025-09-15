pub mod adapter;
mod types;
#[cfg(test)]
mod tests;

pub use adapter::BitgetAdapter;
pub use types::BitgetTickerDto;
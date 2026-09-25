pub mod adapter;
#[cfg(test)]
mod tests;
mod types;

pub use adapter::BitgetAdapter;
pub use types::BitgetTickerDto;

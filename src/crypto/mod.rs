use decimal::d128;

pub use self::bitcoin::Bitcoin;

mod bitcoin;

pub trait Crypto {
    fn balance(address: &str) -> Result<d128, String>;
    fn exchange_rate() -> Result<d128, String>;
}

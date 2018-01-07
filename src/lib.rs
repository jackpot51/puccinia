extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
extern crate mime;
extern crate rand;
extern crate xml;

pub mod ofx;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod server;
mod router;

pub use router::FedoraRouter;

pub mod svc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

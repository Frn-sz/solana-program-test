mod constants;
mod entrypoint;
mod error;
mod instructions;
mod processor;
mod state;

#[cfg(test)]
mod test {
    use litesvm::LiteSVM;

    fn setup() -> LiteSVM {
        let mut svm = litesvm::LiteSVM::new();
        svm
    }

    fn test_create_ata() {
        let svm = setup();
    }
}

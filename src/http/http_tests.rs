
#[cfg(test)]
mod http_tests {

    use crate::http::Request;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::RequestGet;

    #[test]
    fn some_test() {
        let sut = RequestGet::TextFile;
        assert_eq!(RequestGet::TextFile, sut);
    }
}
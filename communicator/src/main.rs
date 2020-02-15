extern crate communicator;

fn main() {
    communicator::client::connect();
}

#[cfg(test)]
mod tests {
    use super::communicator::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}

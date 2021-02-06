pub mod codec;
mod proto;
pub mod server;

use proto::gorums::Metadata;

pub trait Message {
    type Item: prost::Message;

    fn get_metadata(&self) -> &Metadata;
    fn get_message(&self) -> &Vec<u8>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
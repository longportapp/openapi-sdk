#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(uint64, tag="1")]
    pub code: u64,
    #[prost(string, tag="2")]
    pub msg: ::prost::alloc::string::String,
}

pub enum S7Error {
    S7ClientError(s7_client::Error),
}

impl From<s7_client::Error> for S7Error {
    fn from(value: s7_client::Error) -> Self {
        S7Error::S7ClientError(value)
    }
}

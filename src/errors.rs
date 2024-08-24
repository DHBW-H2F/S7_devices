use custom_error::custom_error;

custom_error! {pub S7Error
    S7ClientError {err: s7_client::Error} = "Client error {err}",
    DeviceNotConnectedError = "The device is not connected",
    MismatchedRegisterLengthError = "The given register length does not match the selected register",
    RegisterDoesNotExistsError = "The selected register does not exist",
    InvalidRegisterValue = "The register value is invalid",
}

impl From<s7_client::Error> for S7Error {
    fn from(value: s7_client::Error) -> Self {
        S7Error::S7ClientError { err: value }
    }
}

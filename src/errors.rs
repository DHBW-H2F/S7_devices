#[derive(Debug)]
pub struct DeviceNotConnectedError;
#[derive(Debug)]
pub struct MismatchedRegisterLengthError;
#[derive(Debug)]
pub struct RegisterDoesNotExistsError;
#[derive(Debug)]
pub struct InvalidRegisterValue;

#[derive(Debug)]
pub enum S7Error {
    S7ClientError(s7_client::Error),
    DeviceNotConnectedError(DeviceNotConnectedError),
    MismatchedRegisterLengthError(MismatchedRegisterLengthError),
    RegisterDoesNotExistsError(RegisterDoesNotExistsError),
    InvalidRegisterValue(InvalidRegisterValue),
}

impl From<s7_client::Error> for S7Error {
    fn from(value: s7_client::Error) -> Self {
        S7Error::S7ClientError(value)
    }
}

impl From<DeviceNotConnectedError> for S7Error {
    fn from(value: DeviceNotConnectedError) -> Self {
        S7Error::DeviceNotConnectedError(value)
    }
}

impl From<MismatchedRegisterLengthError> for S7Error {
    fn from(value: MismatchedRegisterLengthError) -> Self {
        S7Error::MismatchedRegisterLengthError(value)
    }
}

impl From<RegisterDoesNotExistsError> for S7Error {
    fn from(value: RegisterDoesNotExistsError) -> Self {
        S7Error::RegisterDoesNotExistsError(value)
    }
}

impl From<InvalidRegisterValue> for S7Error {
    fn from(value: InvalidRegisterValue) -> Self {
        S7Error::InvalidRegisterValue(value)
    }
}

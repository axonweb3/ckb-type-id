use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    // Add customized errors here...
    InvalidTypeIDCellNum,
    TypeIDNotMatch,
    ArgsLengthNotEnough,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl From<crate::type_id::Error> for Error {
    fn from(err: crate::type_id::Error) -> Self {
        match err {
            crate::type_id::Error::Syscall(e) => e.into(),
            crate::type_id::Error::Native(e) => e.into(),
        }
    }
}

impl From<crate::type_id::TypeIDError> for Error {
    fn from(err: crate::type_id::TypeIDError) -> Self {
        match err {
            crate::type_id::TypeIDError::InvalidTypeIDCellNum => Error::InvalidTypeIDCellNum,
            crate::type_id::TypeIDError::TypeIDNotMatch => Error::TypeIDNotMatch,
            crate::type_id::TypeIDError::ArgsLengthNotEnough => Error::ArgsLengthNotEnough,
        }
    }
}

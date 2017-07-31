// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use ffi as glib_ffi;
use error::ErrorDomain;
use translate::*;
use std;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChecksumType {
    Md5,
    Sha1,
    Sha256,
    #[cfg(feature = "v2_36")]
    Sha512,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ChecksumType {
    type GlibType = ffi::GChecksumType;

    fn to_glib(&self) -> ffi::GChecksumType {
        match *self {
            ChecksumType::Md5 => ffi::G_CHECKSUM_MD5,
            ChecksumType::Sha1 => ffi::G_CHECKSUM_SHA1,
            ChecksumType::Sha256 => ffi::G_CHECKSUM_SHA256,
            #[cfg(feature = "v2_36")]
            ChecksumType::Sha512 => ffi::G_CHECKSUM_SHA512,
            ChecksumType::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GChecksumType> for ChecksumType {
    fn from_glib(value: ffi::GChecksumType) -> Self {
        match value as i32 {
            0 => ChecksumType::Md5,
            1 => ChecksumType::Sha1,
            2 => ChecksumType::Sha256,
            #[cfg(feature = "v2_36")]
            3 => ChecksumType::Sha512,
            value => ChecksumType::__Unknown(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum KeyFileError {
    UnknownEncoding,
    Parse,
    NotFound,
    KeyNotFound,
    GroupNotFound,
    InvalidValue,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for KeyFileError {
    type GlibType = ffi::GKeyFileError;

    fn to_glib(&self) -> ffi::GKeyFileError {
        match *self {
            KeyFileError::UnknownEncoding => ffi::G_KEY_FILE_ERROR_UNKNOWN_ENCODING,
            KeyFileError::Parse => ffi::G_KEY_FILE_ERROR_PARSE,
            KeyFileError::NotFound => ffi::G_KEY_FILE_ERROR_NOT_FOUND,
            KeyFileError::KeyNotFound => ffi::G_KEY_FILE_ERROR_KEY_NOT_FOUND,
            KeyFileError::GroupNotFound => ffi::G_KEY_FILE_ERROR_GROUP_NOT_FOUND,
            KeyFileError::InvalidValue => ffi::G_KEY_FILE_ERROR_INVALID_VALUE,
            KeyFileError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileError> for KeyFileError {
    fn from_glib(value: ffi::GKeyFileError) -> Self {
        match value as i32 {
            0 => KeyFileError::UnknownEncoding,
            1 => KeyFileError::Parse,
            2 => KeyFileError::NotFound,
            3 => KeyFileError::KeyNotFound,
            4 => KeyFileError::GroupNotFound,
            5 => KeyFileError::InvalidValue,
            value => KeyFileError::__Unknown(value),
        }
    }
}

impl ErrorDomain for KeyFileError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::g_key_file_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(KeyFileError::UnknownEncoding),
            1 => Some(KeyFileError::Parse),
            2 => Some(KeyFileError::NotFound),
            3 => Some(KeyFileError::KeyNotFound),
            4 => Some(KeyFileError::GroupNotFound),
            5 => Some(KeyFileError::InvalidValue),
            value => Some(KeyFileError::__Unknown(value)),
        }
    }
}


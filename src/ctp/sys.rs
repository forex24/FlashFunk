use crate::get_interface_path;
use core::fmt;
use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_void};

include!(concat!(env!("HOME"), "/.HFQ/ctp/bindings.rs"));

pub fn to_c_ptr(string: String) -> *const i8 {
    CString::new(string).unwrap().as_ptr()
}

pub fn to_c_string(string: String) -> CString {
    CString::new(string).unwrap()
}

pub fn to_i8_arrary(string: String) -> Vec<i8> {
    CString::new(string)
        .unwrap()
        .as_bytes()
        .into_iter()
        .map(|x| *x as i8)
        .collect()
}

pub fn to_c_str<'a>(string: String) -> &'a CStr {
    unsafe { CStr::from_ptr(to_c_string(string).as_ptr()) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisconnectionReason {
    ReadError = 0x1001,
    WriteError = 0x1002,
    HeartbeatTimeout = 0x2001,
    HeartbeatSendError = 0x2002,
    ErrorMessageReceived = 0x2003,
    Unknown = 0x0000,
}

impl std::convert::From<c_int> for DisconnectionReason {
    fn from(reason: c_int) -> DisconnectionReason {
        match reason {
            0x1001 => DisconnectionReason::ReadError,
            0x1002 => DisconnectionReason::WriteError,
            0x2001 => DisconnectionReason::HeartbeatTimeout,
            0x2002 => DisconnectionReason::HeartbeatSendError,
            0x2003 => DisconnectionReason::ErrorMessageReceived,
            _ => DisconnectionReason::Unknown,
        }
    }
}

#[must_use]
pub type RspResult = Result<(), RspError>;

#[derive(Clone, Debug, PartialEq)]
pub struct RspError {
    pub id: TThostFtdcErrorIDType,
    pub msg: String,
}

impl std::error::Error for RspError {}

impl fmt::Display for RspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.msg)
    }
}

pub fn result_to_string(rsp_result: &RspResult) -> String {
    match rsp_result {
        Ok(()) => "Ok(())".to_string(),
        Err(err) => format!("Err(RspError{{ id: {}, msg: {} }})", err.id, err.msg),
    }
}

pub fn get_rsp_info(rsp_info: *const CThostFtdcRspInfoField) -> RspResult {
    #[allow(unused_unsafe)] // for future "unsafe blocks in unsafe fn" feature
    match unsafe { rsp_info.as_ref() } {
        Some(info) => match info {
            CThostFtdcRspInfoField { ErrorID: 0, .. } => Ok(()),
            CThostFtdcRspInfoField {
                ErrorID: id,
                ErrorMsg: msg,
            } => Err(RspError {
                id: *id,
                msg: slice_to_string(msg),
            }),
        },
        None => Ok(()),
    }
}

pub fn zh_cstr_to_str(v: &[u8]) -> Cow<str> {
    let slice = v.split(|&c| c == 0u8).next().unwrap();
    if slice.is_ascii() {
        unsafe {
            return Cow::Borrowed::<str>(std::str::from_utf8_unchecked(slice));
        }
    }
    match GB18030.decode(slice, DecoderTrap::Replace) {
        Ok(s) => Cow::Owned(s),
        Err(e) => e,
    }
}

/// todo: 下面有问题描述
pub fn slice_to_string(v: &[i8]) -> String {
    let x: Vec<u8> = v.to_vec().iter().map(|x| *x as u8).collect();
    zh_cstr_to_str(x.as_slice()).to_string()
}

pub trait ToCSlice<T> {
    fn to_c_slice(&self) -> T;
}

macro_rules! impl_to_c_slice {
    ($len: expr) => (
        impl ToCSlice<[i8; $len]> for &str {
            fn to_c_slice(&self) -> [i8; $len] {
                let mut array = [0i8; $len];
                let mut i = 0;
                let size = array.len();
                for x in self.chars() {
                    if i >= size {
                        break;
                    }
                    array[i] = x as i8;
                    i += 1;
                }
                array
            }
        }
        impl ToCSlice<[i8; $len]> for String {
            fn to_c_slice(&self) -> [i8; $len] {
                let mut array = [0i8; $len];
                let mut i = 0;
                let size = array.len();
                for x in self.chars() {
                    if i >= size {
                        break;
                    }
                    array[i] = x as i8;
                    i += 1;
                }
                array
            }
        }
    );
    ($len:expr, $($len2:expr),+) => (
        impl_to_c_slice!($len);
        impl_to_c_slice!($($len2),+);
    )
}

impl_to_c_slice!(
    301, 11, 2561, 349, 100, 501, 129, 9, 31, 12, 17, 13, 51, 71, 25, 10, 513, 21, 256, 365, 36,
    15, 16, 20, 23, 5, 41, 257, 3, 2, 33, 1001, 201, 273, 65, 401, 261, 24, 61, 4, 81, 161, 2049,
    6, 22, 1025, 7, 101
);

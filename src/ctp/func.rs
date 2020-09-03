/// This module implement all the c functions to be called in rust
use crate::ctp::sys::*;
use crate::ctp::api::{DisconnectionReason, RspResult, result_to_string};
use crate::app::CtpbeeR;
use actix::Addr;
use failure::_core::ffi::c_void;


pub trait QuoteApi: Send {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_login(&mut self, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("用户登录 回调 ")
    }

    #[allow(unused_variables)]
    fn on_rsp_user_logout(&mut self, pUserLogout: *mut CThostFtdcUserLogoutField,
                          pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("用户登出 回调")
    }

    fn on_rsp_error(&mut self, pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        print!("错误回调信息");
    }

    fn on_rsp_sub_market_data(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
                              pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("订阅行情回调信息");
    }

    fn on_rsp_un_sub_market_data(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
                                 pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("取消订阅行情回报信息");
    }

    fn on_rsp_sub_for_quote_rsp(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
                                pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("此处为询价应答");
    }

    fn on_rsp_un_sub_for_quote_rsp(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
                                   pRspInfo: *mut CThostFtdcRspInfoField, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("此处为取消询价应答");
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        println!("行情到达");
    }

    fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {
        println!("询价通知");
    }

    fn on_heart_beaten(&mut self) {
        println!("心跳警告");
    }
    fn get_addr(&self) -> &Addr<CtpbeeR>;
}


unsafe fn get_quote_spi<'a>(spi: *mut c_void) -> &'a mut dyn QuoteApi {
    &mut **(spi as *mut *mut dyn QuoteApi)
}

// extern "C" {
//     #[link_name = "\u{1}??0QuoteSpi@@QEAA@PEAX@Z"]
//     pub fn QuoteSpi_QuoteSpi(this: *mut QuoteSpi, rust_object: *mut ::std::os::raw::c_void);
// }
//
// extern "C" {
//     #[link_name = "\u{1}??_DQuoteSpi@@QEAAXXZ"]
//     pub fn QuoteSpi_QuoteSpi_destructor(this: *mut QuoteSpi);
// }

// impl QuoteSpi {
//     #[inline]
//     pub unsafe fn new(rust_object: *mut ::std::os::raw::c_void) -> Self {
//         let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
//         QuoteSpi_QuoteSpi(__bindgen_tmp.as_mut_ptr(), rust_object);
//         __bindgen_tmp.assume_init()
//     }
//     #[inline]
//     pub unsafe fn destruct(&mut self) {
//         QuoteSpi_QuoteSpi_destructor(self)
//     }
// }

pub unsafe extern "C" fn QuoteSpi_OnFrontConnected(this: *mut ::std::os::raw::c_void) {
    println!("{}", "前置连接成功! ");
}


pub unsafe extern "C" fn QuoteSpi_OnFrontDisconnected(
    this: *mut ::std::os::raw::c_void,
    nReason: ::std::os::raw::c_int,
) {
    let x = get_quote_spi(this);
    x.on_front_disconnected(DisconnectionReason::from(nReason));
}


#[link_name = "\u{1}?OnHeartBeatWarning@QuoteSpi@@UEAAXH@Z"]
pub unsafe extern "C" fn QuoteSpi_OnHeartBeatWarning(
    this: *mut ::std::os::raw::c_void,
    nTimeLapse: ::std::os::raw::c_int,
) {
    let x = get_quote_spi(this);
}


#[link_name = "\u{1}?OnRspUserLogin@QuoteSpi@@UEAAXPEAUCThostFtdcRspUserLoginField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspUserLogin(
    this: *mut ::std::os::raw::c_void,
    pRspUserLogin: *mut CThostFtdcRspUserLoginField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}


#[link_name = "\u{1}?OnRspUserLogout@QuoteSpi@@UEAAXPEAUCThostFtdcUserLogoutField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspUserLogout(
    this: *mut ::std::os::raw::c_void,
    pUserLogout: *mut CThostFtdcUserLogoutField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}


#[link_name = "\u{1}?OnRspQryMulticastInstrument@QuoteSpi@@UEAAXPEAUCThostFtdcMulticastInstrumentField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspQryMulticastInstrument(
    this: *mut ::std::os::raw::c_void,
    pMulticastInstrument: *mut CThostFtdcMulticastInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    println!("查询多个合约的回报  此处还没实现方法噢")
}


#[link_name = "\u{1}?OnRspError@QuoteSpi@@UEAAXPEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspError(
    this: *mut ::std::os::raw::c_void,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}


#[link_name = "\u{1}?OnRspSubMarketData@QuoteSpi@@UEAAXPEAUCThostFtdcSpecificInstrumentField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspSubMarketData(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)
}


#[link_name = "\u{1}?OnRspUnSubMarketData@QuoteSpi@@UEAAXPEAUCThostFtdcSpecificInstrumentField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspUnSubMarketData(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_un_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}


#[link_name = "\u{1}?OnRspSubForQuoteRsp@QuoteSpi@@UEAAXPEAUCThostFtdcSpecificInstrumentField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspSubForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}


#[link_name = "\u{1}?OnRspUnSubForQuoteRsp@QuoteSpi@@UEAAXPEAUCThostFtdcSpecificInstrumentField@@PEAUCThostFtdcRspInfoField@@H_N@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRspUnSubForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: ::std::os::raw::c_int,
    bIsLast: bool,
) {
    let x = get_quote_spi(this);
    x.on_rsp_un_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}


#[link_name = "\u{1}?OnRtnDepthMarketData@QuoteSpi@@UEAAXPEAUCThostFtdcDepthMarketDataField@@@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRtnDepthMarketData(
    this: *mut ::std::os::raw::c_void,
    pDepthMarketData: *mut CThostFtdcDepthMarketDataField,
) {
    let x = get_quote_spi(this);
    x.on_rtn_depth_market_data(pDepthMarketData);
}


#[link_name = "\u{1}?OnRtnForQuoteRsp@QuoteSpi@@UEAAXPEAUCThostFtdcForQuoteRspField@@@Z"]
pub unsafe extern "C" fn QuoteSpi_OnRtnForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pForQuoteRsp: *mut CThostFtdcForQuoteRspField,
) {
    let x = get_quote_spi(this);
    x.on_rtn_for_quote_rsp(pForQuoteRsp);
}

extern "C" {
    pub fn QuoteSpi_Destructor(stub: *mut QuoteSpi);
}
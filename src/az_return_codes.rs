#![allow(unused)]

use azsys;

use std::fmt;

#[derive(PartialEq, Debug)]
pub enum AzReturnCode {
    AzResultCoreOk = azsys::az_result_core_AZ_OK as isize,
    AzResultCoreErrorCanceled = azsys::az_result_core_AZ_ERROR_CANCELED as isize,
    AzResultCoreErrorArg = azsys::az_result_core_AZ_ERROR_ARG as isize,
    AzResultCoreErrorNotEnoughSpace = azsys::az_result_core_AZ_ERROR_NOT_ENOUGH_SPACE as isize,
    AzResultCoreErrorNotImplemented = azsys::az_result_core_AZ_ERROR_NOT_IMPLEMENTED as isize,
    AzResultCoreErrorItemNotFound = azsys::az_result_core_AZ_ERROR_ITEM_NOT_FOUND as isize,
    AzResultCoreErrorUnexpectedChar = azsys::az_result_core_AZ_ERROR_UNEXPECTED_CHAR as isize,
    AzResultCoreErrorUnexpectedEnd = azsys::az_result_core_AZ_ERROR_UNEXPECTED_END as isize,
    AzResultCoreErrorNotSupported = azsys::az_result_core_AZ_ERROR_NOT_SUPPORTED as isize,
    AzResultCoreErrorDependencyNotProvided =
        azsys::az_result_core_AZ_ERROR_DEPENDENCY_NOT_PROVIDED as isize,
    AzResultCoreErrorOutOfMemory = azsys::az_result_core_AZ_ERROR_OUT_OF_MEMORY as isize,
    AzResultCoreErrorJsonInvalidState = azsys::az_result_core_AZ_ERROR_JSON_INVALID_STATE as isize,
    AzResultCoreErrorJsonNestingOverflow =
        azsys::az_result_core_AZ_ERROR_JSON_NESTING_OVERFLOW as isize,
    AzResultCoreErrorJsonReaderDone = azsys::az_result_core_AZ_ERROR_JSON_READER_DONE as isize,
    AzResultCoreErrorHttpInvalidState = azsys::az_result_core_AZ_ERROR_HTTP_INVALID_STATE as isize,
    AzResultCoreErrorHttpPipelineInvalidPolicy =
        azsys::az_result_core_AZ_ERROR_HTTP_PIPELINE_INVALID_POLICY as isize,
    AzResultCoreErrorHttpInvalidMethodVerb =
        azsys::az_result_core_AZ_ERROR_HTTP_INVALID_METHOD_VERB as isize,
    AzResultCoreErrorHttpAuthenticationFailed =
        azsys::az_result_core_AZ_ERROR_HTTP_AUTHENTICATION_FAILED as isize,
    AzResultCoreErrorHttpResponseOverflow =
        azsys::az_result_core_AZ_ERROR_HTTP_RESPONSE_OVERFLOW as isize,
    AzResultCoreErrorHttpResponseCouldntResolveHost =
        azsys::az_result_core_AZ_ERROR_HTTP_RESPONSE_COULDNT_RESOLVE_HOST as isize,
    AzResultCoreErrorHttpCorruptResponseHeader =
        azsys::az_result_core_AZ_ERROR_HTTP_CORRUPT_RESPONSE_HEADER as isize,
    AzResultCoreErrorHttpEndOfHeaders = azsys::az_result_core_AZ_ERROR_HTTP_END_OF_HEADERS as isize,
    AzResultCoreErrorHttpAdapter = azsys::az_result_core_AZ_ERROR_HTTP_ADAPTER as isize,
    AzResultIoTErrorTopicNoMatch = azsys::az_result_iot_AZ_ERROR_IOT_TOPIC_NO_MATCH as isize,
    AzResultIoTErrorEndOfProperties = azsys::az_result_iot_AZ_ERROR_IOT_END_OF_PROPERTIES as isize,
}

impl AzReturnCode {
    pub fn from_i32(value: i32) -> AzReturnCode {
        unsafe { std::mem::transmute(value) }
    }
}

impl fmt::Display for AzReturnCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AzReturnCode::AzResultCoreOk => write!(f, "AzResultCoreOk"),
            AzReturnCode::AzResultCoreErrorCanceled => write!(f, "AzResultCoreErrorCanceled"),
            AzReturnCode::AzResultCoreErrorArg => write!(f, "AzResultCoreErrorArg"),
            AzReturnCode::AzResultCoreErrorNotEnoughSpace => {
                write!(f, "AzResultCoreErrorNotEnoughSpace")
            }
            AzReturnCode::AzResultCoreErrorNotImplemented => {
                write!(f, "AzResultCoreErrorNotImplemented")
            }
            AzReturnCode::AzResultCoreErrorItemNotFound => {
                write!(f, "AzResultCoreErrorItemNotFound")
            }
            AzReturnCode::AzResultCoreErrorUnexpectedChar => {
                write!(f, "AzResultCoreErrorUnexpectedChar")
            }
            AzReturnCode::AzResultCoreErrorUnexpectedEnd => {
                write!(f, "AzResultCoreErrorUnexpectedEnd")
            }
            AzReturnCode::AzResultCoreErrorNotSupported => {
                write!(f, "AzResultCoreErrorNotSupported")
            }
            AzReturnCode::AzResultCoreErrorDependencyNotProvided => {
                write!(f, "AzResultCoreErrorDependencyNotProvided")
            }
            AzReturnCode::AzResultCoreErrorOutOfMemory => write!(f, "AzResultCoreErrorOutOfMemory"),
            AzReturnCode::AzResultCoreErrorJsonInvalidState => {
                write!(f, "AzResultCoreErrorJsonInvalidState")
            }
            AzReturnCode::AzResultCoreErrorJsonNestingOverflow => {
                write!(f, "AzResultCoreErrorJsonNestingOverflow")
            }
            AzReturnCode::AzResultCoreErrorJsonReaderDone => {
                write!(f, "AzResultCoreErrorJsonReaderDone")
            }
            AzReturnCode::AzResultCoreErrorHttpInvalidState => {
                write!(f, "AzResultCoreErrorHttpInvalidState")
            }
            AzReturnCode::AzResultCoreErrorHttpPipelineInvalidPolicy => {
                write!(f, "AzResultCoreErrorHttpPipelineInvalidPolicy")
            }
            AzReturnCode::AzResultCoreErrorHttpInvalidMethodVerb => {
                write!(f, "AzResultCoreErrorHttpInvalidMethodVerb")
            }
            AzReturnCode::AzResultCoreErrorHttpAuthenticationFailed => {
                write!(f, "AzResultCoreErrorHttpAuthenticationFailed")
            }
            AzReturnCode::AzResultCoreErrorHttpResponseOverflow => {
                write!(f, "AzResultCoreErrorHttpResponseOverflow")
            }
            AzReturnCode::AzResultCoreErrorHttpResponseCouldntResolveHost => {
                write!(f, "AzResultCoreErrorHttpResponseCouldntResolveHost")
            }
            AzReturnCode::AzResultCoreErrorHttpCorruptResponseHeader => {
                write!(f, "AzResultCoreErrorHttpCorruptResponseHeader")
            }
            AzReturnCode::AzResultCoreErrorHttpEndOfHeaders => {
                write!(f, "AzResultCoreErrorHttpEndOfHeaders")
            }
            AzReturnCode::AzResultCoreErrorHttpAdapter => write!(f, "AzResultCoreErrorHttpAdapter"),
            AzReturnCode::AzResultIoTErrorTopicNoMatch => write!(f, "AzResultIoTErrorTopicNoMatch"),
            AzReturnCode::AzResultIoTErrorEndOfProperties => {
                write!(f, "AzResultIoTErrorEndOfProperties")
            }
            _ => {
                let work: *const i32 =
                    unsafe { std::mem::transmute::<&AzReturnCode, *const i32>(self) };
                write!(
                    f,
                    "Unrecognized return code failue {}",
                    format!("{}", unsafe { *work as i32 })
                )
            }
        }
    }
}

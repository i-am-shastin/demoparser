use crate::first_pass::sendtables::Serializer;
use std::fmt;

pub const HEADER_ENDS_AT_BYTE: usize = 16;
pub const OUTER_BUF_DEFAULT_LEN: usize = 400_000;
pub const INNER_BUF_DEFAULT_LEN: usize = 8192 * 15;

#[derive(Debug, Clone)]
pub struct Class {
    pub class_id: i32,
    pub name: String,
    pub serializer: Serializer,
}

#[derive(Debug, PartialEq)]
pub enum DemoParserError {
    ClassMapperNotFoundFirstPass,
    FieldNoDecoder,
    OutOfBitsError,
    OutOfBytesError,
    FailedByteRead(String),
    UnknownPathOp,
    EntityNotFound,
    ClassNotFound,
    MalformedMessage,
    StringTableNotFound,
    Source1DemoError,
    DemoEndsEarly(String),
    UnknownFile,
    IncorrectMetaDataProp,
    UnknownPropName(String),
    GameEventListNotSet,
    PropTypeNotFound(String),
    GameEventUnknownId(String),
    UnknownPawnPrefix(String),
    UnknownEntityHandle(String),
    ClsIdOutOfBounds,
    UnknownGameEventVariant(String),
    FileNotFound(String),
    NoEvents,
    DecompressionFailure(String),
    NoSendTableMessage,
    UserIdNotFound,
    EventListFallbackNotFound(String),
    VoiceDataWriteError(String),
    UnknownDemoCmd(i32),
    IllegalPathOp,
    VectorResizeFailure,
    ImpossibleCmd,
    UnknownVoiceFormat,
    MalformedVoicePacket,
    CantCreateOpusDecoder,
    MultithreadingWasNotOk,
}

impl std::error::Error for DemoParserError {}

impl fmt::Display for DemoParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
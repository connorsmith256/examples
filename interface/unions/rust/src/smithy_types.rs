// This file is @generated by wasmcloud/weld-codegen 0.4.6.
// It is not intended for manual editing.
// namespace: org.wasmcloud.example.smithy_types

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DataType {
    /// n(10)
    TypeU8,

    /// n(11)
    TypeU16,

    /// n(12)
    TypeU32,

    /// n(13)
    TypeU64,

    /// n(20)
    TypeI8,

    /// n(21)
    TypeI16,

    /// n(22)
    TypeI32,

    /// n(23)
    TypeI64,
}

// Encode DataType as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_data_type<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &DataType,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    // encoding union DataType
    e.array(2)?;
    match val {
        DataType::TypeU8 => {
            e.u16(10)?;
            e.null()?;
        }
        DataType::TypeU16 => {
            e.u16(11)?;
            e.null()?;
        }
        DataType::TypeU32 => {
            e.u16(12)?;
            e.null()?;
        }
        DataType::TypeU64 => {
            e.u16(13)?;
            e.null()?;
        }
        DataType::TypeI8 => {
            e.u16(20)?;
            e.null()?;
        }
        DataType::TypeI16 => {
            e.u16(21)?;
            e.null()?;
        }
        DataType::TypeI32 => {
            e.u16(22)?;
            e.null()?;
        }
        DataType::TypeI64 => {
            e.u16(23)?;
            e.null()?;
        }
    }
    Ok(())
}

// Decode DataType from cbor input stream
#[doc(hidden)]
pub fn decode_data_type(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<DataType, RpcError> {
    let __result = {
        // decoding union DataType
        let len = d.fixed_array()?;
        if len != 2 {
            return Err(RpcError::Deser(
                "decoding union 'DataType': expected 2-array".to_string(),
            ));
        }
        match d.u16()? {
            10 => {
                d.null()?;
                DataType::TypeU8
            }

            11 => {
                d.null()?;
                DataType::TypeU16
            }

            12 => {
                d.null()?;
                DataType::TypeU32
            }

            13 => {
                d.null()?;
                DataType::TypeU64
            }

            20 => {
                d.null()?;
                DataType::TypeI8
            }

            21 => {
                d.null()?;
                DataType::TypeI16
            }

            22 => {
                d.null()?;
                DataType::TypeI32
            }

            23 => {
                d.null()?;
                DataType::TypeI64
            }

            n => {
                return Err(RpcError::Deser(format!("invalid field number for union 'org.wasmcloud.example.smithy_types#DataType':{}", n)));
            }
        }
    };
    Ok(__result)
}
pub type Fruit = String;

// Encode Fruit as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_fruit<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Fruit,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.str(val)?;
    Ok(())
}

// Decode Fruit from cbor input stream
#[doc(hidden)]
pub fn decode_fruit(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Fruit, RpcError> {
    let __result = { d.str()?.to_string() };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MoreThings {
    pub things: Things,
}

// Encode MoreThings as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_more_things<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &MoreThings,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("things")?;
    encode_things(e, &val.things)?;
    Ok(())
}

// Decode MoreThings from cbor input stream
#[doc(hidden)]
pub fn decode_more_things(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<MoreThings, RpcError> {
    let __result = {
        let mut things: Option<Things> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct MoreThings, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        things = Some(decode_things(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.example.smithy_types#Things': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "things" => {
                        things = Some(decode_things(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.example.smithy_types#Things': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        MoreThings {
            things: if let Some(__x) = things {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field MoreThings.things (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// simple type declaration
pub type MyDocument = wasmbus_rpc::common::Document;

// Encode MyDocument as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_my_document<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &MyDocument,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    wasmbus_rpc::common::encode_document(&mut e, val)?;
    Ok(())
}

// Decode MyDocument from cbor input stream
#[doc(hidden)]
pub fn decode_my_document(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<MyDocument, RpcError> {
    let __result = { wasmbus_rpc::common::decode_document(d)? };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Stuff {
    #[serde(default)]
    pub val: String,
}

// Encode Stuff as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_stuff<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Stuff,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("val")?;
    e.str(&val.val)?;
    Ok(())
}

// Decode Stuff from cbor input stream
#[doc(hidden)]
pub fn decode_stuff(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Stuff, RpcError> {
    let __result = {
        let mut val: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Stuff, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => val = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "val" => val = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Stuff {
            val: if let Some(__x) = val {
                __x
            } else {
                return Err(RpcError::Deser("missing field Stuff.val (#0)".to_string()));
            },
        }
    };
    Ok(__result)
}
/// struct containing many types
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Things {
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub blob: Vec<u8>,
    #[serde(default)]
    pub doc: wasmbus_rpc::common::Document,
    #[serde(rename = "float32")]
    #[serde(default)]
    pub float_32: f32,
    #[serde(rename = "float64")]
    #[serde(default)]
    pub float_64: f64,
    #[serde(rename = "int16")]
    #[serde(default)]
    pub int_16: i16,
    #[serde(rename = "int32")]
    #[serde(default)]
    pub int_32: i32,
    #[serde(rename = "int64")]
    #[serde(default)]
    pub int_64: i64,
    #[serde(rename = "int8")]
    #[serde(default)]
    pub int_8: i8,
    #[serde(default)]
    pub str: String,
    #[serde(rename = "uint16")]
    #[serde(default)]
    pub uint_16: u16,
    #[serde(rename = "uint32")]
    #[serde(default)]
    pub uint_32: u32,
    #[serde(rename = "uint64")]
    #[serde(default)]
    pub uint_64: u64,
    #[serde(rename = "uint8")]
    #[serde(default)]
    pub uint_8: u8,
}

// Encode Things as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_things<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Things,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(13)?;
    e.str("blob")?;
    e.bytes(&val.blob)?;
    e.str("doc")?;
    wasmbus_rpc::common::encode_document(&mut e, &val.doc)?;
    e.str("float32")?;
    e.f32(val.float_32)?;
    e.str("float64")?;
    e.f64(val.float_64)?;
    e.str("int16")?;
    e.i16(val.int_16)?;
    e.str("int32")?;
    e.i32(val.int_32)?;
    e.str("int64")?;
    e.i64(val.int_64)?;
    e.str("int8")?;
    e.i8(val.int_8)?;
    e.str("str")?;
    e.str(&val.str)?;
    e.str("uint16")?;
    e.u16(val.uint_16)?;
    e.str("uint32")?;
    e.u32(val.uint_32)?;
    e.str("uint64")?;
    e.u64(val.uint_64)?;
    e.str("uint8")?;
    e.u8(val.uint_8)?;
    Ok(())
}

// Decode Things from cbor input stream
#[doc(hidden)]
pub fn decode_things(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Things, RpcError> {
    let __result = {
        let mut blob: Option<Vec<u8>> = None;
        let mut doc: Option<wasmbus_rpc::common::Document> = None;
        let mut float_32: Option<f32> = None;
        let mut float_64: Option<f64> = None;
        let mut int_16: Option<i16> = None;
        let mut int_32: Option<i32> = None;
        let mut int_64: Option<i64> = None;
        let mut int_8: Option<i8> = None;
        let mut str: Option<String> = None;
        let mut uint_16: Option<u16> = None;
        let mut uint_32: Option<u32> = None;
        let mut uint_64: Option<u64> = None;
        let mut uint_8: Option<u8> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Things, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => blob = Some(d.bytes()?.to_vec()),
                    1 => doc = Some(wasmbus_rpc::common::decode_document(d)?),
                    2 => float_32 = Some(d.f32()?),
                    3 => float_64 = Some(d.f64()?),
                    4 => int_16 = Some(d.i16()?),
                    5 => int_32 = Some(d.i32()?),
                    6 => int_64 = Some(d.i64()?),
                    7 => int_8 = Some(d.i8()?),
                    8 => str = Some(d.str()?.to_string()),
                    9 => uint_16 = Some(d.u16()?),
                    10 => uint_32 = Some(d.u32()?),
                    11 => uint_64 = Some(d.u64()?),
                    12 => uint_8 = Some(d.u8()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "blob" => blob = Some(d.bytes()?.to_vec()),
                    "doc" => doc = Some(wasmbus_rpc::common::decode_document(d)?),
                    "float32" => float_32 = Some(d.f32()?),
                    "float64" => float_64 = Some(d.f64()?),
                    "int16" => int_16 = Some(d.i16()?),
                    "int32" => int_32 = Some(d.i32()?),
                    "int64" => int_64 = Some(d.i64()?),
                    "int8" => int_8 = Some(d.i8()?),
                    "str" => str = Some(d.str()?.to_string()),
                    "uint16" => uint_16 = Some(d.u16()?),
                    "uint32" => uint_32 = Some(d.u32()?),
                    "uint64" => uint_64 = Some(d.u64()?),
                    "uint8" => uint_8 = Some(d.u8()?),
                    _ => d.skip()?,
                }
            }
        }
        Things {
            blob: if let Some(__x) = blob {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.blob (#0)".to_string(),
                ));
            },

            doc: if let Some(__x) = doc {
                __x
            } else {
                return Err(RpcError::Deser("missing field Things.doc (#1)".to_string()));
            },

            float_32: if let Some(__x) = float_32 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.float_32 (#2)".to_string(),
                ));
            },

            float_64: if let Some(__x) = float_64 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.float_64 (#3)".to_string(),
                ));
            },

            int_16: if let Some(__x) = int_16 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.int_16 (#4)".to_string(),
                ));
            },

            int_32: if let Some(__x) = int_32 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.int_32 (#5)".to_string(),
                ));
            },

            int_64: if let Some(__x) = int_64 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.int_64 (#6)".to_string(),
                ));
            },

            int_8: if let Some(__x) = int_8 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.int_8 (#7)".to_string(),
                ));
            },

            str: if let Some(__x) = str {
                __x
            } else {
                return Err(RpcError::Deser("missing field Things.str (#8)".to_string()));
            },

            uint_16: if let Some(__x) = uint_16 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.uint_16 (#9)".to_string(),
                ));
            },

            uint_32: if let Some(__x) = uint_32 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.uint_32 (#10)".to_string(),
                ));
            },

            uint_64: if let Some(__x) = uint_64 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.uint_64 (#11)".to_string(),
                ));
            },

            uint_8: if let Some(__x) = uint_8 {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Things.uint_8 (#12)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// wasmbus.contractId: wasmcloud:example:smithy_types
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait SmithyTypes {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:smithy_types"
    }
    async fn send_stuff(&self, ctx: &Context, arg: &Stuff) -> RpcResult<Stuff>;
    async fn send_document(
        &self,
        ctx: &Context,
        arg: &wasmbus_rpc::common::Document,
    ) -> RpcResult<wasmbus_rpc::common::Document>;
    async fn send_doc_type(&self, ctx: &Context, arg: &MyDocument) -> RpcResult<MyDocument>;
    async fn send_things(&self, ctx: &Context, arg: &Things) -> RpcResult<Things>;
    async fn send_more_things(&self, ctx: &Context, arg: &MoreThings) -> RpcResult<MoreThings>;
}

/// SmithyTypesReceiver receives messages defined in the SmithyTypes service trait
#[doc(hidden)]
#[async_trait]
pub trait SmithyTypesReceiver: MessageDispatch + SmithyTypes {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "SendStuff" => {
                let value: Stuff = wasmbus_rpc::common::decode(&message.arg, &decode_stuff)
                    .map_err(|e| RpcError::Deser(format!("'Stuff': {}", e)))?;
                let resp = SmithyTypes::send_stuff(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_stuff(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "SmithyTypes.SendStuff",
                    arg: Cow::Owned(buf),
                })
            }
            "SendDocument" => {
                let value: wasmbus_rpc::common::Document = wasmbus_rpc::common::decode(
                    &message.arg,
                    &wasmbus_rpc::common::decode_document,
                )
                .map_err(|e| RpcError::Deser(format!("'Document': {}", e)))?;
                let resp = SmithyTypes::send_document(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                wasmbus_rpc::common::encode_document(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "SmithyTypes.SendDocument",
                    arg: Cow::Owned(buf),
                })
            }
            "SendDocType" => {
                let value: MyDocument =
                    wasmbus_rpc::common::decode(&message.arg, &decode_my_document)
                        .map_err(|e| RpcError::Deser(format!("'MyDocument': {}", e)))?;
                let resp = SmithyTypes::send_doc_type(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_my_document(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "SmithyTypes.SendDocType",
                    arg: Cow::Owned(buf),
                })
            }
            "SendThings" => {
                let value: Things = wasmbus_rpc::common::decode(&message.arg, &decode_things)
                    .map_err(|e| RpcError::Deser(format!("'Things': {}", e)))?;
                let resp = SmithyTypes::send_things(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_things(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "SmithyTypes.SendThings",
                    arg: Cow::Owned(buf),
                })
            }
            "SendMoreThings" => {
                let value: MoreThings =
                    wasmbus_rpc::common::decode(&message.arg, &decode_more_things)
                        .map_err(|e| RpcError::Deser(format!("'MoreThings': {}", e)))?;
                let resp = SmithyTypes::send_more_things(self, ctx, &value).await?;
                let mut e = wasmbus_rpc::cbor::vec_encoder(true);
                encode_more_things(&mut e, &resp)?;
                let buf = e.into_inner();
                Ok(Message {
                    method: "SmithyTypes.SendMoreThings",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "SmithyTypes::{}",
                message.method
            ))),
        }
    }
}

/// SmithyTypesSender sends messages to a SmithyTypes service
/// client for sending SmithyTypes messages
#[derive(Debug)]
pub struct SmithyTypesSender<T: Transport> {
    transport: T,
}

impl<T: Transport> SmithyTypesSender<T> {
    /// Constructs a SmithyTypesSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> SmithyTypesSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl SmithyTypesSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl SmithyTypesSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a SmithyTypes provider
    /// implementing the 'wasmcloud:example:smithy_types' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:smithy_types",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a SmithyTypes provider
    /// implementing the 'wasmcloud:example:smithy_types' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:smithy_types",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> SmithyTypes for SmithyTypesSender<T> {
    #[allow(unused)]
    async fn send_stuff(&self, ctx: &Context, arg: &Stuff) -> RpcResult<Stuff> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_stuff(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SmithyTypes.SendStuff",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Stuff = wasmbus_rpc::common::decode(&resp, &decode_stuff)
            .map_err(|e| RpcError::Deser(format!("'{}': Stuff", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn send_document(
        &self,
        ctx: &Context,
        arg: &wasmbus_rpc::common::Document,
    ) -> RpcResult<wasmbus_rpc::common::Document> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        wasmbus_rpc::common::encode_document(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SmithyTypes.SendDocument",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: wasmbus_rpc::common::Document =
            wasmbus_rpc::common::decode(&resp, &wasmbus_rpc::common::decode_document)
                .map_err(|e| RpcError::Deser(format!("'{}': Document", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn send_doc_type(&self, ctx: &Context, arg: &MyDocument) -> RpcResult<MyDocument> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_my_document(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SmithyTypes.SendDocType",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: MyDocument = wasmbus_rpc::common::decode(&resp, &decode_my_document)
            .map_err(|e| RpcError::Deser(format!("'{}': MyDocument", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn send_things(&self, ctx: &Context, arg: &Things) -> RpcResult<Things> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_things(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SmithyTypes.SendThings",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Things = wasmbus_rpc::common::decode(&resp, &decode_things)
            .map_err(|e| RpcError::Deser(format!("'{}': Things", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn send_more_things(&self, ctx: &Context, arg: &MoreThings) -> RpcResult<MoreThings> {
        let mut e = wasmbus_rpc::cbor::vec_encoder(true);
        encode_more_things(&mut e, arg)?;
        let buf = e.into_inner();
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SmithyTypes.SendMoreThings",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: MoreThings = wasmbus_rpc::common::decode(&resp, &decode_more_things)
            .map_err(|e| RpcError::Deser(format!("'{}': MoreThings", e)))?;
        Ok(value)
    }
}

use crate::utils::LabelMetadata;

use super::*;
pub use cml_core::{error::*, serialization::*};
use std::io::{Seek, SeekFrom};

impl cbor_event::se::Serialize for FilesDetails {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(3))?;
        serializer.write_text(&"src")?;
        self.src.serialize(serializer)?;
        serializer.write_text(&"name")?;
        self.name.serialize(serializer)?;
        serializer.write_text(&"mediaType")?;
        self.media_type.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for FilesDetails {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(3)?;
            let mut src = None;
            let mut name = None;
            let mut media_type = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::Text => match raw.text()?.as_str() {
                        "src" => {
                            if src.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "src".into(),
                                ))
                                .into());
                            }
                            src = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(ChunkableString::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("src"))?,
                            );
                        }
                        "name" => {
                            if name.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "name".into(),
                                ))
                                .into());
                            }
                            name = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(String64::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("name"))?,
                            );
                        }
                        "mediaType" => {
                            if media_type.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "mediaType".into(),
                                ))
                                .into());
                            }
                            media_type = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(String64::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("media_type"))?,
                            );
                        }
                        _unknown_key => {
                            // CIP-25 allows permissive parsing
                            read_len.read_elems(1)?;
                            // we still need to read the data to move on to the CBOR after it
                            let _other_metadatum =
                                cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    _other_type => {
                        // CIP-25 allows permissive parsing
                        read_len.read_elems(1)?;
                        // we still need to read the data to move on to the CBOR after it
                        let _other_key =
                            cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                        let _other_value =
                            cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                    }
                }
                read += 1;
            }
            let name = match name {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("name")))
                            .into(),
                    )
                }
            };
            let media_type =
                match media_type {
                    Some(x) => x,
                    None => {
                        return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(
                            String::from("mediaType"),
                        ))
                        .into())
                    }
                };
            let src = match src {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("src")))
                            .into(),
                    )
                }
            };
            ();
            Ok(Self {
                name,
                media_type,
                src,
            })
        })()
        .map_err(|e| e.annotate("FilesDetails"))
    }
}

impl cbor_event::se::Serialize for CIP25Metadata {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(721u64)?;
        self.key_721.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CIP25Metadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(1)?;
            let mut key_721 = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        721 => {
                            if key_721.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(721)).into());
                            }
                            key_721 = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(LabelMetadata::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("key_721"))?,
                            );
                        }
                        _unknown_key => {
                            // CIP-25 allows permissive parsing
                            read_len.read_elems(1)?;
                            // we still need to read the data to move on to the CBOR after it
                            let _other_metadatum =
                                cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            let key_721 = match key_721 {
                Some(x) => x,
                None => {
                    return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(721)).into())
                }
            };
            ();
            Ok(Self { key_721 })
        })()
        .map_err(|e| e.annotate("CIP25Metadata"))
    }
}

impl cbor_event::se::Serialize for MetadataDetails {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(
            2 + match &self.media_type {
                Some(_) => 1,
                None => 0,
            } + match &self.description {
                Some(_) => 1,
                None => 0,
            } + match &self.files {
                Some(_) => 1,
                None => 0,
            },
        ))?;
        serializer.write_text(&"name")?;
        self.name.serialize(serializer)?;
        if let Some(field) = &self.files {
            serializer.write_text(&"files")?;
            serializer.write_array(cbor_event::Len::Len(field.len() as u64))?;
            for element in field.iter() {
                element.serialize(serializer)?;
            }
        }
        serializer.write_text(&"image")?;
        self.image.serialize(serializer)?;
        if let Some(field) = &self.media_type {
            serializer.write_text(&"mediaType")?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.description {
            serializer.write_text(&"description")?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MetadataDetails {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(2)?;
            let mut name = None;
            let mut files = None;
            let mut image = None;
            let mut media_type = None;
            let mut description = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::Text => match raw.text()?.as_str() {
                        "name" => {
                            if name.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "name".into(),
                                ))
                                .into());
                            }
                            name = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(String64::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("name"))?,
                            );
                        }
                        "files" => {
                            if files.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "files".into(),
                                ))
                                .into());
                            }
                            files = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut files_arr = Vec::new();
                                    let len = raw.array()?;
                                    while match len {
                                        cbor_event::Len::Len(n) => files_arr.len() < n as usize,
                                        cbor_event::Len::Indefinite => true,
                                    } {
                                        if raw.cbor_type()? == CBORType::Special {
                                            assert_eq!(raw.special()?, CBORSpecial::Break);
                                            break;
                                        }
                                        files_arr.push(FilesDetails::deserialize(raw)?);
                                    }
                                    Ok(files_arr)
                                })()
                                .map_err(|e| e.annotate("files"))?,
                            );
                        }
                        "image" => {
                            if image.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "image".into(),
                                ))
                                .into());
                            }
                            image = Some(
                                (|| -> Result<_, DeserializeError> {
                                    Ok(ChunkableString::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("image"))?,
                            );
                        }
                        "mediaType" => {
                            if media_type.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "mediaType".into(),
                                ))
                                .into());
                            }
                            media_type = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(String64::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("media_type"))?,
                            );
                        }
                        "description" => {
                            if description.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "description".into(),
                                ))
                                .into());
                            }
                            description = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ChunkableString::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("description"))?,
                            );
                        }
                        _unknown_key => {
                            // CIP-25 allows permissive parsing
                            read_len.read_elems(1)?;
                            // we still need to read the data to move on to the CBOR after it
                            let _other_metadatum =
                                cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    _other_type => {
                        // CIP-25 allows permissive parsing
                        read_len.read_elems(1)?;
                        // we still need to read the data to move on to the CBOR after it
                        let _other_key =
                            cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                        let _other_value =
                            cml_core::metadata::TransactionMetadatum::deserialize(raw)?;
                    }
                }
                read += 1;
            }
            let name = match name {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("name")))
                            .into(),
                    )
                }
            };
            let image = match image {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("image")))
                            .into(),
                    )
                }
            };
            read_len.finish()?;
            Ok(Self {
                name,
                image,
                media_type,
                description,
                files,
            })
        })()
        .map_err(|e| e.annotate("MetadataDetails"))
    }
}

impl cbor_event::se::Serialize for String64 {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text(&self.0)
    }
}

impl Deserialize for String64 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let inner = raw.text()? as String;
        if inner.len() > 64 {
            return Err(DeserializeError::new(
                "String64",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(64),
                },
            ));
        }
        Ok(Self(inner))
    }
}

impl cbor_event::se::Serialize for ChunkableString {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ChunkableString::Single(string64) => string64.serialize(serializer),
            ChunkableString::Chunked(arr_string64) => {
                serializer.write_array(cbor_event::Len::Len(arr_string64.len() as u64))?;
                for element in arr_string64.iter() {
                    element.serialize(serializer)?;
                }
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for ChunkableString {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(String64::deserialize(raw)?)
            })(raw)
            {
                Ok(string64) => return Ok(Self::Single(string64)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut arr_string64_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => arr_string64_arr.len() < n as usize,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    arr_string64_arr.push(String64::deserialize(raw)?);
                }
                Ok(arr_string64_arr)
            })(raw)
            {
                Ok(arr_string64) => return Ok(Self::Chunked(arr_string64)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            Err(DeserializeError::new(
                "ChunkableString",
                DeserializeFailure::NoVariantMatched.into(),
            ))
        })()
        .map_err(|e| e.annotate("ChunkableString"))
    }
}

#[allow(dead_code)]
const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP: () = {
    use ::prost_wkt::typetag;
    #[typetag::serde(name = "type.googleapis.com/google.protobuf.Timestamp")]
    impl ::prost_wkt::MessageSerde for Timestamp {
        fn package_name(&self) -> &'static str { "google.protobuf" }
        fn message_name(&self) -> &'static str { "Timestamp" }
        fn type_url(&self) -> &'static str { "type.googleapis.com/google.protobuf.Timestamp" }
        fn new_instance(&self, data: Vec<u8>) -> ::std::result::Result<Box<dyn ::prost_wkt::MessageSerde>, ::prost::DecodeError> {
            let mut target = Self::default();
            ::prost::Message::merge(&mut target, data.as_slice())?;
            let erased: ::std::boxed::Box<dyn ::prost_wkt::MessageSerde> = ::std::boxed::Box::new(target);
            Ok(erased)
        }
        fn try_encoded(&self) -> ::std::result::Result<::std::vec::Vec<u8>, ::prost::EncodeError> {
            let mut buf = ::std::vec::Vec::new();
            buf.reserve(::prost::Message::encoded_len(self));
            ::prost::Message::encode(self, &mut buf)?;
            Ok(buf)
        }
    }
    ::prost_wkt::inventory::submit! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } }
};

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TIMESTAMP : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/google.protobuf.Timestamp")] impl :: prost_wkt :: MessageSerde for Timestamp { fn package_name (& self) -> & 'static str { "google.protobuf" } fn message_name (& self) -> & 'static str { "Timestamp" } fn type_url (& self) -> & 'static str { "type.googleapis.com/google.protobuf.Timestamp" } fn new_instance (& self , data : Vec < u8 >) -> :: std :: result :: Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : :: std :: boxed :: Box < dyn :: prost_wkt :: MessageSerde > = :: std :: boxed :: Box :: new (target) ; Ok (erased) } fn try_encoded (& self) -> :: std :: result :: Result < :: std :: vec :: Vec < u8 > , :: prost :: EncodeError > { let mut buf = :: std :: vec :: Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } :: prost_wkt :: inventory :: submit ! { :: prost_wkt :: MessageSerdeDecoderEntry { type_url : "type.googleapis.com/google.protobuf.Timestamp" , decoder : | buf : & [u8] | { let msg : Timestamp = :: prost :: Message :: decode (buf) ? ; Ok (:: std :: boxed :: Box :: new (msg)) } } } } ;

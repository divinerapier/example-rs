// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod testing {
    include!(concat!(env!("OUT_DIR"), "/testing/mod.rs"));

    #[cfg(feature = "prost-codec")]
    pub use self::grpc::testing::*;
}

pub mod example {
    include!(concat!(env!("OUT_DIR"), "/example/mod.rs"));
}

pub mod health {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/health/mod.rs"));

        #[cfg(feature = "prost-codec")]
        pub use self::grpc::health::v1::*;
    }
}

pub mod directory {
    include!(concat!(env!("OUT_DIR"), "/directory/mod.rs"));
    #[cfg(feature = "prost-codec")]
    pub use self::grpc::directory::*;
}

pub mod volume {
    include!(concat!(env!("OUT_DIR"), "/volume/mod.rs"));
    #[cfg(feature = "prost-codec")]
    pub use self::grpc::volume::*;
}

#[cfg(feature = "prost-codec")]
#[allow(clippy::large_enum_variant)]
pub mod help {

    use super::testing::*;

    // Wrapper functions for oneof fields.
    impl ClientArgs {
        pub fn get_mark(&self) -> &Mark {
            match &self.argtype {
                ::std::option::Option::Some(client_args::Argtype::Mark(v)) => v,
                _ => <Mark as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_setup(&self) -> &ClientConfig {
            match &self.argtype {
                ::std::option::Option::Some(client_args::Argtype::Setup(v)) => v,
                _ => <ClientConfig as ::protobuf::Message>::default_instance(),
            }
        }
    }
    impl ServerArgs {
        pub fn get_mark(&self) -> &Mark {
            match &self.argtype {
                ::std::option::Option::Some(server_args::Argtype::Mark(v)) => v,
                _ => <Mark as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_setup(&self) -> &ServerConfig {
            match &self.argtype {
                ::std::option::Option::Some(server_args::Argtype::Setup(v)) => v,
                _ => <ServerConfig as ::protobuf::Message>::default_instance(),
            }
        }
    }
    impl ChannelArg {
        pub fn get_str_value(&self) -> &str {
            match &self.value {
                ::std::option::Option::Some(channel_arg::Value::StrValue(v)) => v,
                _ => "",
            }
        }
        pub fn get_int_value(&self) -> i32 {
            match self.value {
                ::std::option::Option::Some(channel_arg::Value::IntValue(v)) => v,
                _ => 0,
            }
        }
        pub fn has_str_value(&self) -> bool {
            match self.value {
                ::std::option::Option::Some(channel_arg::Value::StrValue(_)) => true,
                _ => false,
            }
        }
        pub fn has_int_value(&self) -> bool {
            match self.value {
                ::std::option::Option::Some(channel_arg::Value::IntValue(_)) => true,
                _ => false,
            }
        }
    }
    impl LoadParams {
        pub fn get_closed_loop(&self) -> &ClosedLoopParams {
            match &self.load {
                ::std::option::Option::Some(load_params::Load::ClosedLoop(v)) => v,
                _ => <ClosedLoopParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_poisson(&self) -> &PoissonParams {
            match &self.load {
                ::std::option::Option::Some(load_params::Load::Poisson(v)) => v,
                _ => <PoissonParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_poisson(&self) -> bool {
            match self.load {
                ::std::option::Option::Some(load_params::Load::Poisson(_)) => true,
                _ => false,
            }
        }
    }
    impl PayloadConfig {
        pub fn get_simple_params(&self) -> &SimpleProtoParams {
            match &self.payload {
                ::std::option::Option::Some(payload_config::Payload::SimpleParams(v)) => v,
                _ => <SimpleProtoParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_bytebuf_params(&self) -> &ByteBufferParams {
            match &self.payload {
                ::std::option::Option::Some(payload_config::Payload::BytebufParams(v)) => v,
                _ => <ByteBufferParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_bytebuf_params(&self) -> bool {
            match self.payload {
                ::std::option::Option::Some(payload_config::Payload::BytebufParams(_)) => true,
                _ => false,
            }
        }
    }
}

pub mod util;

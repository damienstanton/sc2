use prost;
use prost_derive;

pub mod Request {
    include!(concat!(env!("OUT_DIR"), "/sc2api_protocol.rs"));
}

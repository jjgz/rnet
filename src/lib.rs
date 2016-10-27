#![feature(proc_macro)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Point {
    /// This is in meters.
    pub x: f64,
    /// This is in meters.
    pub y: f64,
    /// This is a variance in meter squared.
    pub v: f64,
    /// This is the angle in radians.
    pub angle: f64,
    /// This is the angle's variance in radians^2.
    pub av: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Netmessage {
    ReqName,
    NameJosh,
    NameGeordon,
    NameZach,
    NameJoe,
    NameDebugJosh,
    NameDebugGeordon,
    NameDebugZach,
    NameDebugJoe,
    Netstats {
        #[serde(rename = "myName")]
        my_name: String,
        #[serde(rename = "numGoodMessagesRecved")]
        num_good_messages_recved: u32,
        #[serde(rename = "numCommErrors")]
        num_comm_errors: u32,
        #[serde(rename = "numJSONRequestsRecved")]
        num_json_requests_recved: u32,
        #[serde(rename = "numJSONResponsesRecved")]
        num_json_responses_recved: u32,
        #[serde(rename = "numJSONRequestsSent")]
        num_json_requests_sent: u32,
        #[serde(rename = "numJSONResponsesSent")]
        num_json_responses_sent: u32,
    },
    Heartbeat,
    ReqNetstats,
    /// Joe
    ReqMovement,
    /// Geordon
    Movement(Point),
    /// Geordon
    JoeReqPoints,
    /// Joe
    JF(u32),
    /// Joe
    JE(u32),
    /// Geordon
    JoshReqPoints,
    /// Josh
    CF(u32),
    /// Josh
    CE(u32),
    /// Josh
    CT(u32),
    /// Geordon
    ReqStopped,
    /// Josh
    Stopped(bool),
    /// Josh
    ReqInPosition,
    /// Zach
    InPosition(bool),
    /// Zach
    ReqEdgeDetect,
    /// Josh
    EdgeDetect(bool),
    /// Zach
    ReqEdgeDropped,
    /// Josh
    EdgeDropped(bool),
    /// Zach
    ReqDistance,
    /// Josh; Value is in meters.
    Distance(f64),
    /// Zach
    ReqGrabbed,
    /// Josh
    Grabbed(bool),
    /// Zach
    ReqDropped,
    /// Josh
    Dropped(bool),
	PDebugJosh(Vec<u64>),
	ADebugJosh(Vec<u64>),
	TestMove(u32),
	TestRotate(u32),
	ReqTestReset,
	RDebugJosh(Vec<u64>),
	TestRow(Vec<u64>),
}

impl Netmessage {
    pub fn bot_name(&self) -> String {
        String::from(match *self {
            Netmessage::NameGeordon => "Geordon",
            Netmessage::NameJoe => "Joe",
            Netmessage::NameJosh => "Josh",
            Netmessage::NameZach => "Zach",
            Netmessage::NameDebugGeordon => "DebugGeordon",
            Netmessage::NameDebugJoe => "DebugJoe",
            Netmessage::NameDebugJosh => "DebugJosh",
            Netmessage::NameDebugZach => "DebugZach",
            _ => "Unnamed",
        })
    }
}

#[test]
fn test_world_json() {
    use std::io::{stderr, Write};
    writeln!(&mut stderr(),
             "ReqMovement json: {}",
             serde_json::to_string(&Netmessage::ReqMovement)
                 .unwrap())
        .unwrap();
    writeln!(&mut stderr(),
             "CF json: {}",
             serde_json::to_string(&Netmessage::CF(457))
                 .unwrap())
        .unwrap();
    writeln!(&mut stderr(),
             "Movement json: {}",
             serde_json::to_string(&Netmessage::Movement(Point{
                 x: 1.0,
                 y: -1.0,
                 v: 0.1,
                 angle: 0.2,
                 av: 0.01,
             }))
                 .unwrap())
        .unwrap();
    writeln!(&mut stderr(),
             "Stopped json: {}",
             serde_json::to_string(&Netmessage::Stopped(true))
                 .unwrap())
        .unwrap();
    writeln!(&mut stderr(),
             "ReqName json: {}",
             serde_json::to_string(&Netmessage::ReqName)
                 .unwrap())
        .unwrap();
}

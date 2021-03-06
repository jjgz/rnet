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
pub struct Coordinate {
    /// This is in meters.
    pub x: f64,
    /// This is in meters.
    pub y: f64,
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
    Initialize{
        /// Number of targets.
        nt: usize,
        /// Rover A position.
        ra: Coordinate,
        /// Arena border global coordinates in meters.
        bd: Vec<Coordinate>,
    },
    GDBuild,
    /// Joe
    ReqMovement,
    /// Geordon
    Movement(Point),
    /// Geordon
    ReqAssumed,
    /// Joe
    Assumed(Point),
    /// Geordon
    ReqTargets,
    /// Josh
    Targets(Vec<Coordinate>),
    /// Geordon
    ReqHalfRow(u8),
    /// Josh
    HalfRow(Vec<u8>),
    /// Geordon
    ReqStopped,
    /// Josh
    Stopped(bool),
    /// Josh
    ReqInPosition,
    /// Zach
    InPosition(bool),
    /// Zach
    ReqEdgeDetectLeft,
	ReqEdgeDetectRight,
    /// Josh
    EdgeDetect(u8, u8, u8),
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
	GReqGrabbed,
	DReqDropped,
    /// DebugJoe filled from module
    DebugJF(u32),
    /// DebugJoe empty from module
    DebugJE(u32),
    //DebugJoe speed for motors
    DebugJoeOC(f64,f64,u32,u32),
    //DebugJoe movement test for rover
    DebugJoeTread(bool, bool),
    //DebugJoeDistance for straight movement distance guess output
    DebugJoeDistance(u32),
    //DebugJoeUltra sends fake ultrasonic and photosensor data from client for debugging
    DebugJoeUltra(f64,f64,f64),
    /// DebugJoe movement test for rover
    DebugGeordon(String),
    /// Geordon
    GDReqHalfRow(u8),
    /// GeordonDebug
    GDHalfRow(Vec<u8>),
    GDReqPing,
    GDPing,
    GDFinish,
    GDAligned,
	PDebugJosh(Vec<u64>),
	ADebugJosh(Vec<u64>),
	TestMove(u32),
	TestRotate(u32),
	ReqTestReset,
	RDebugJosh(Vec<u64>),
	TestRow(Vec<u64>),
	JCHalfRow(Vec<u64>),
	HDebugJosh(u8),
    /// Geordon from Joe for proximity data.
    ReqProximity,
    /// Joe from Geordon with proximity data.
    Proximity {
        left_ir: f64,
        right_ir: f64,
    },
    GDStartAlign,
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
    writeln!(&mut stderr(),
             "ReqHalfRow json: {}",
             serde_json::to_string(&Netmessage::ReqHalfRow(4))
                 .unwrap())
        .unwrap();
    writeln!(&mut stderr(),
             "HalfRow json: {}",
             serde_json::to_string(&Netmessage::HalfRow(vec![0; 64]))
                 .unwrap())
        .unwrap();
}

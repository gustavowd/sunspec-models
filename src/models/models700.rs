#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum ACType {
    #[default]
    SINGLEPHASE = 0,
    SPLITPHASE = 1,
    THREEPHASE = 1 << 1,
    PAD=0xFFFF
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum State {
    #[default]
    OFF = 0,
    ON = 1,
    PAD=0xFFFF
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum InvState {
    #[default]
    OFF = 0,
    SLEEPING = 1,
    STARTING = 2,
    RUNNING = 3,
    THROTTLED = 4,
    SHUTTINGDOWN = 5,
    FAULT = 6,
    STANDBY = 7,
    PAD=0xFFFF
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(unused)]
#[repr(u16)]
pub enum ConnState {
    #[default]
    DISCONNECTED = 0,
    CONNECTED = 1,
    PAD=0xFFFF
}


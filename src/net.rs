use core::slice::Iter;

#[derive(Debug)]
pub struct NetConnection {
    device: usize,
    pin: usize,
}

impl NetConnection {
    pub fn new(device: usize, pin: usize) -> NetConnection {
        return NetConnection { device, pin };
    }

    pub fn get_device(&self) -> usize {
        return self.device;
    }

    pub fn get_pin(&self) -> usize {
        return self.pin;
    }
}

#[derive(Debug)]
pub struct Net {
    connections: Vec<NetConnection>,
    value: u32,
}

impl Net {
    pub fn new(connections: Vec<NetConnection>) -> Net {
        return Net {
            connections,
            value: 0,
        };
    }

    pub fn get_value(&self) -> u32 {
        return self.value;
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    pub fn connections_iter(&self) -> Iter<NetConnection> {
        return self.connections.iter();
    }
}

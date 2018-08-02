use std;

struct StatusPacket {
    cnt: u8,
    err: u8,
    tim: u16,
    sts: u8,
}

impl StatusPacket {
    fn serialize(&self) -> [u8; 8] {
        let mut a = [0; 8];
        a[0] = self.cnt;
        a[1] = self.err;
        a[3] = (self.tim & 0xff) as u8;
        a[4] = ((self.tim >> 8) & 0xff) as u8;
        a[5] = self.sts;
        a
    }
}

struct WarningPacket {
    level: u8,
}

impl WarningPacket {
    fn serialize(&self) -> [u8; 8] {
        let mut a = [0; 8];
        a[0] = 0x10;
        a[1] = 0x08;
        a[2] = 0x70;
        a[4] = self.level;
        a
    }
}

struct ItemPacket {
    error_flag: u8,
    exist_flag: u16,
    timestamp: u16,
    sync_counter: u8,
}

impl ItemPacket {
    fn serialize(&self) -> [u8; 8] {
        let mut a = [0; 8];
        a[0] = ((self.error_flag & 0x0f) << 4) as u8;

        a[1] = ((self.exist_flag >> 8) & 0xff) as u8;
        a[2] = ((self.exist_flag & 0x3) << 6) as u8;

        a[3] = ((self.timestamp >> 12) & 0xf) as u8;
        a[4] = ((self.timestamp >> 4) & 0xff) as u8;
        a[5] = ((self.timestamp & 0xf) << 4) as u8;

        a[7] = self.sync_counter;
        a
    }
}

struct ItemDetailPacket {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    sync_counter: u8,
}

impl ItemDetailPacket {
    fn serialize(&self) -> [u8; 8] {
        assert!(self.x >= -128f32 && self.x <= 127f32);
        assert!(self.y >= -128f32 && self.y <= 127f32);

        let mut a = [0; 8];
        let x = (self.x * (2f32.powf(8f32))) as i16;
        a[0] = ((x >> 8) & 0xff) as u8;
        a[1] = (x & 0xfc) as u8;

        let y = (self.y * (2f32.powf(8f32))) as i16;
        a[1] |= ((y >> 14) & 0x3) as u8;
        a[2] = ((y >> 6) & 0xff) as u8;
        a[3] = ((y & 0x7c) << 2) as u8;

        assert!(self.vx >= -64f32 && self.vx <= 63f32);
        assert!(self.vy >= -64f32 && self.vy <= 63f32);

        let vx = (self.vx * (2f32.powf(9f32))) as i16;
        a[3] |= ((vx >> 12) & 0x0f) as u8;
        a[4] = ((vx >> 4) & 0xfe) as u8;

        let vy = (self.vy * (2f32.powf(9f32))) as i16;
        a[4] |= ((vy >> 15) & 0x01) as u8;
        a[5] = ((vy >> 6) & 0xff) as u8;
        a[6] = (((vy & (3 << 5)) << 1) & 0xff) as u8;

        a[7] = self.sync_counter;
        a
    }
}

fn create_packet(can_id: u32, data: Vec<u8>) -> Vec<u8> {
    assert!(data.len() <= std::u8::MAX as usize);

    let mut packet = Vec::new();

    packet.push(((can_id >> 24) & 0xff) as u8);
    packet.push(((can_id >> 16) & 0xff) as u8);
    packet.push(((can_id >> 8) & 0xff) as u8);
    packet.push((can_id & 0xff) as u8);

    packet.push(data.len() as u8);

    packet.push(0 as u8); // padding
    packet.push(0 as u8); // reserved
    packet.push(0 as u8); // reserved

    for d in data {
        packet.push(d);
    }

    packet
}

pub fn create_status_packet(radar_id: u8, cnt: u8,
                            err: u8, timestamp: u16, status: u8) -> Vec<u8> {
    let p = StatusPacket{cnt: cnt, err: err, tim: timestamp, sts: status};

    create_packet(0x7d0u32 + radar_id as u32, p.serialize().to_vec())
}

pub fn create_item_packet(radar_id: u8, 
                          error_flag: u8, exist_flag: u16,
                          timestamp: u16, sync_counter: u8) -> Vec<u8> {
    let p = ItemPacket{error_flag: error_flag, exist_flag: exist_flag,
                       timestamp: timestamp, sync_counter: sync_counter};

    create_packet(0x600u32 + radar_id as u32, p.serialize().to_vec())
}

pub fn create_detail_packet(radar_id: u8, item_index: u8,
                            x: f32, y: f32, vx: f32, vy: f32,
                            sync_counter: u8) -> Vec<u8> {
    let p = ItemDetailPacket { x: x, y: y, vx: vx, vy: vy, sync_counter: sync_counter};

    create_packet(0x600u32 + radar_id as u32 * 0x10 + item_index as u32,
                  p.serialize().to_vec())
}

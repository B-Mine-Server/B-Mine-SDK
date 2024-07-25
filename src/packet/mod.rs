use tracing::info;

pub struct DataPacket {
    pub data: Vec<u8>,
}

impl DataPacket {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn set_data(data: Vec<u8>) -> Self {
        Self { data }
    }
    pub fn sethand_id(&mut self, id: u8) {
        self.data.insert(0, id);
    }
}

pub trait DataPacketReader {
    // 获取数据包的 ID
    fn get_pid(&self) -> u8;

    // 解码协议包数据。
    fn decode(&mut self, data: &Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        // 解码数据包并返回 DataPacket 实例。
        unimplemented!()
    }
    // 编码协议包数据。
    fn encode(&self) -> Vec<u8> {
        // 编码数据包并返回 Vec<u8> 实例。
        unimplemented!()
    }

    // 处理协议包数据
    fn process(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        // 处理数据并返回 Ok(())。
        unimplemented!()
    }
}

/*
// 包结构体
pub struct Packet {
    // 数据包ID，用于标识不同的数据包类型
    id: u8,
    // 当前读取数据的偏移量
    offset: usize,
    // 数据包的缓冲区
    buffer: Vec<u8>,
    send_time: Option<SystemTime>,
}
impl Packet {
    // 从数据包中获取指定长度的字节序列。
    pub fn get(&self, len: i64) -> Result<Vec<u8>, std::io::Error> {
        let start = self.offset as i64;
        if start + len > self.buffer.len() as i64 {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Buffer not enough to read",
            ))
        } else {
            let data = &self.buffer[start as usize..(start + len) as usize];
            Ok(data.to_vec())
        }
    }
    // 获取数据包中剩余的所有字节。
    pub fn remaining(&self) -> Result<Vec<u8>, std::io::Error> {
        self.get(self.buffer.len() as i64 - self.offset as i64)
    }

    // 从数据包中读取一个长整型数值。
    pub fn read_long(&mut self) -> Result<i64, std::io::Error> {
        let slice = self.get(8).unwrap();
        let mut buf = [0; 8];
        buf.copy_from_slice(&slice);
        Ok(i64::from_be_bytes(buf))
    }

    // 从数据包中读取一个整型数值。
    pub fn read_int(&mut self) -> Result<i32, std::io::Error> {
        let slice = self.get(4).unwrap();
        let mut buf = [0; 4];
        buf.copy_from_slice(&slice);
        Ok(i32::from_be_bytes(buf))
    }

    // 从数据包中读取一个短整型数值。
    pub fn read_short(&mut self) -> Result<i16, std::io::Error> {
        let slice = self.get(2).unwrap();
        let mut buf = [0; 2];
        buf.copy_from_slice(&slice);
        Ok(i16::from_be_bytes(buf))
    }

    // 从数据包中读取一个短整型数值，可指定是否为有符号数。
    pub fn read_short_signed(&mut self, signed: bool) -> Result<i16, std::io::Error> {
        let short = self.read_short()?;
        if signed {
            Ok(short)
        } else {
            Ok(short as i16)
        }
    }
    // 从数据包中读取一个三字节的i32数值。
    pub fn get_triad(&mut self) -> Result<i32, std::io::Error> {
        let slice: Vec<u8> = self.get(3).unwrap();
        let mut buf = [0; 4];
        buf.copy_from_slice(&slice);
        Ok(i32::from_ne_bytes(buf)) // 转换为 u32
    }

    // 从数据包中读取一个大端三字节的整型数值。
    pub fn get_ltriad(&mut self) -> Result<u8, std::io::Error> {
        let slice: Vec<u8> = self.get(3).unwrap();
        let mut buf = [0; 4];
        buf.copy_from_slice(&slice);
        Ok(slice[0])
    }
    // 从数据包中读取一个字节。
    pub fn read_byte_array(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let slice = self.get((self.offset + 1).try_into().unwrap())?;
        Ok(slice.to_vec())
    }
    // 从数据包中读取一个UTF-8编码的字符串。
    pub fn read_string(&mut self) -> Result<String, std::io::Error> {
        let short = self.read_short().unwrap();
        let slice = self.get(short.clone().into())?;
        let string = String::from_utf8_lossy(&slice);
        Ok(string.to_string())
    }
}

    */

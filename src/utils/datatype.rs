use std::{i16, u32};

use uuid::Uuid;

// 使用枚举来定义不同的数据类型
#[derive(Debug)]
pub enum DataType {
    Byte(u8),
    Boolean(bool),
    Short(i16),
    UnsignedShort(u32),
    Int(i32),
    Bigendian(i32), // 大端表示的有符号32位整数
    UnsignedInt(u32),
    Long(i64),
    UnsignedLong(u64),
    Float(f32),
    Double(f64),
    VarInt(u32), // 可变长度整数，这里使用u32作为示例
    SignedVarInt(i32),
    VarLong(i64),
    SignedVarLong(i64),
    String(String), // UTF-8编码的字符串
    Vector3(Vector3),
    Vector2(Vector2),
    NBT(/* NBT数据的结构 */),
    ByteArray(Vec<u8>),                // 以VarInt为前缀的字节数组
    BlockCoordinates((i32, u32, i32)), // 三个SignedVarInt或VarInt
    PlayerLocation(PlayerLocation),    // 三个浮点数和三个字节
    UUID(Uuid),
}

// 测试数据类型
#[test]
fn test() {
    let u8data_type_min = DataType::Byte(u8::MIN);
    let u8data_type_max = DataType::Byte(u8::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        u8data_type_min,
        u8data_type_max,
        std::mem::size_of::<u8>(),
    );

    let bool_data_type = DataType::Boolean(true);
    println!(
        "Data Size :{:?} 占位大小:{}",
        bool_data_type,
        std::mem::size_of::<bool>(),
    );

    let i16data_type_min = DataType::Short(i16::MIN);
    let i16data_type_max = DataType::Short(i16::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        i16data_type_min,
        i16data_type_max,
        std::mem::size_of::<i16>(),
    );

    let u16data_type_min = DataType::UnsignedShort(u32::MIN);
    let u16data_type_max = DataType::UnsignedShort(u32::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        u16data_type_min,
        u16data_type_max,
        std::mem::size_of::<u16>(),
    );

    let i32data_type_min = DataType::Int(i32::MIN);
    let i32data_type_max = DataType::Int(i32::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        i32data_type_min,
        i32data_type_max,
        std::mem::size_of::<i32>(),
    );

    let i32data_type_min = DataType::Bigendian(i32::MIN);
    let i32data_type_max = DataType::Bigendian(i32::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        i32data_type_min,
        i32data_type_max,
        std::mem::size_of::<i32>(),
    );

    let u32data_type_min = DataType::UnsignedInt(u32::MIN);
    let u32data_type_max = DataType::UnsignedInt(u32::MAX);
    println!(
        "Data Size :{:?} {:?}占位大小:{}",
        u32data_type_min,
        u32data_type_max,
        std::mem::size_of::<u32>(),
    );

    let i64data_type_min = DataType::Long(i64::MIN);
    let i64data_type_max = DataType::Long(i64::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        i64data_type_min,
        i64data_type_max,
        std::mem::size_of::<i64>(),
    );

    let u64data_type_min = DataType::UnsignedLong(u64::MIN);
    let u64data_type_max = DataType::UnsignedLong(u64::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        u64data_type_min,
        u64data_type_max,
        std::mem::size_of::<u64>(),
    );

    let f32data_type_min = DataType::Float(f32::MIN);
    let f32data_type_max = DataType::Float(f32::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        f32data_type_min,
        f32data_type_max,
        std::mem::size_of::<f32>(),
    );

    let f64data_type_min = DataType::Double(f64::MIN);
    let f64data_type_max = DataType::Double(f64::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        f64data_type_min,
        f64data_type_max,
        std::mem::size_of::<f64>(),
    );

    // 跳过String

    // todo ≥ 1；≤ 5
    let varint_data_type_min = DataType::VarInt(u32::MIN);
    let varint_data_type_max = DataType::VarInt(u32::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        varint_data_type_min,
        varint_data_type_max,
        std::mem::size_of::<u32>(),
    );

    // todo ≥ 1；≤ 5
    let signed_varint_data_type_min = DataType::SignedVarInt(i32::MIN);
    let signed_varint_data_type_max = DataType::SignedVarInt(i32::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        signed_varint_data_type_min,
        signed_varint_data_type_max,
        std::mem::size_of::<i32>(),
    );

    // todo 缺少文档
    let varlong_data_type_min = DataType::VarLong(i64::MIN);
    let varlong_data_type_max = DataType::VarLong(i64::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        varlong_data_type_min,
        varlong_data_type_max,
        std::mem::size_of::<i64>(),
    );

    // ! 范围应该在 -2^63 and 2^63-1
    let signed_varlong_data_type_min = DataType::SignedVarLong(i64::MIN);
    let signed_varlong_data_type_max = DataType::SignedVarLong(i64::MAX);
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        signed_varlong_data_type_min,
        signed_varlong_data_type_max,
        std::mem::size_of::<i64>(),
    );
    println!("跳过String类型");

    let vector3_data_type = DataType::Vector3(Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
    println!(
        "Data Size :{:?} 占位大小:{}",
        vector3_data_type,
        std::mem::size_of::<Vector3>(),
    );

    let vector2_data_type = DataType::Vector2(Vector2 { x: 0.0, y: 0.0 });
    println!(
        "Data Size :{:?} 占位大小:{}",
        vector2_data_type,
        std::mem::size_of::<Vector2>(),
    );

    println!("跳过NBt类型");

    let byte_array_data_type = DataType::ByteArray(vec![0u8; 10]);
    println!(
        "Data Size :{:?} 占位大小:{}",
        byte_array_data_type,
        std::mem::size_of::<Vec<u8>>(),
    );

    // todo: ≥ 3 ； ≤ 15
    let block_coordinates_data_type = DataType::BlockCoordinates((0, 0, 0));
    println!(
        "Data Size :{:?} 占位大小:{}",
        block_coordinates_data_type,
        std::mem::size_of::<(i32, u32, i32)>(),
    );

    // !占位应该为15 ，但实际为16
    let player_location_data_type = DataType::PlayerLocation(PlayerLocation {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        yaw: 0,
        pitch: 0,
        flags: 0,
    });
    println!(
        "Data Size :{:?} 占位大小:{}",
        player_location_data_type,
        std::mem::size_of::<PlayerLocation>(),
    );
    let uuid_data_type_min = DataType::UUID(Uuid::new_v4());
    let uuid_data_type_max = DataType::UUID(Uuid::new_v4());
    println!(
        "Data Size :{:?} {:?} 占位大小:{}",
        uuid_data_type_min,
        uuid_data_type_max,
        std::mem::size_of::<Uuid>(),
    );
}

#[derive(Debug)]
pub struct PlayerLocation {
    x: f32,
    y: f32,
    z: f32,
    yaw: u8,
    pitch: u8,
    flags: u8,
}

// 定义Vector3结构体
#[derive(Debug)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

// 定义Vector2结构体
#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

// 对于复杂的结构，如NBT，需要定义相应的数据结构来表示NBT的格式

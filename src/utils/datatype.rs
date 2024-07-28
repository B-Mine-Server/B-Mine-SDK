// pub enum DataType {
//     // Byte 范围：0-255 字节大小：1
//     Byte,
//     // Boolean  0 or 1  0 或 1
//     Boolean,
//     // Short  范围：-32768 to 32767  -32768 到 32767  字节大小：2
//     Short,
//     //  Unsigned short  范围：0 to 65535  0 到 65535  字节大小：2
//     UnsignedShort,
//     //  Int  范围：-2147483648 2147483647 字节大小：4
//     Int,
//     //  Bigendian 范围：-2147483648 2147483647  有符号 32 位整数
//     Bigendian,
//     //  Unsigned int  范围：0 to 4294967295  0 到 4294967295  字节大小：4
//     UnsignedInt,
//     //   Long  范围：-2^63 to 2^63-1  -2^63 至 2^63-1  字节大小：8
//     Long,
//     // Unsigned 范围：2^64-1   字节大小：8
//     UnsignedLong,
//     // Float 范围：缺少说明  字节大小：单精度 32 位 IEEE 754 浮点 poInt 数
//     Float,
//     // 双精度  范围：缺少说明  字节大小：双精度 64 位 IEEE 754 浮点 poInt 数
//     Double,
//     // 范围： 0 to 4294967295 字节大小：≥ 1; ≤ 5;
//     VarInt,
//     // SignedVarInt   0 to 4294967295   字节大小：≥ 1; ≤ 5;
//     SignedVarInt,
//     //  VarLong  范围：缺少说明 字节大小：≥ 1; ≤ 10;
//     VarLong,
//     // SignedVarLong  范围：-2^63 and 2^63-1  字节大小：≥ 1; ≤ 10;
//     SignedVarLong,
//     // UTF-8 格式以 VarInt 为前缀的 String 大小（以字节为单位）
//     String,
//     // Vector3  三个浮点值（分别为 X、Y 和 Z）字节大小：12
//     Vector3,
//     // Vector2   两个浮点值（分别为 X 和 Y） 字节大小：8
//     Vector2,
//     // NBT  Named Binary Tag 格式
//     NBT,
//     // ByteArray 以 VarInt 为前缀的任意 Bytes 数组，其大小以 Bytes 为前缀。
//     ByteArray,
//     // 一个 SignedVarInt、一个普通 VarInt 和另一个 SignedVarInt（分别为 X、Y 和 Z） 字节大小：≥ 3;≤ 15;
//     BlockCoordinates,
//     // 三个浮点值（分别为 X、Y 和 Z），后跟三个 Bytes（分别为音高、头部偏航和偏航）。
//     // 要将 Bytes 转换为正常音高和偏航值，请将它们除以 0.71
//     // 字节大小：15
//     PlayerLocation,
//     // UUID  编码为两个无符号 64 位整数：最高有效 64 位和最低有效 64 位 字节大小：16
//     UUID,
// }

// 使用枚举来定义不同的数据类型
enum DataType {
    Byte(u8),
    Boolean(bool),
    Short(i16),
    UnsignedShort(u16),
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
    ByteArray(Vec<u8>),                          // 以VarInt为前缀的字节数组
    BlockCoordinates((i32, u32, i32)),           // 三个SignedVarInt或VarInt
    PlayerLocation((f32, f32, f32, u8, u8, u8)), // 三个浮点数和三个字节
    UUID((u64, u64)),                            // 两个无符号64位整数
}

// 定义Vector3结构体
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

// 定义Vector2结构体
pub struct Vector2 {
    x: f32,
    y: f32,
}

// 对于复杂的结构，如NBT，需要定义相应的数据结构来表示NBT的格式

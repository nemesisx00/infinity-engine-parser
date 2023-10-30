#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod bif;
mod bifc;
mod bifcc;

pub use bif::Bif;
pub use bifc::Bifc;
pub use bifcc::Bifcc;

/// 0x0001
pub const ResourceType_BMP: i16 = 1;
/// 0x0002
pub const ResourceType_MVE: i16 = 2;
/// 0x0004
pub const ResourceType_WAV: i16 = 4;
/// 0x0004
pub const ResourceType_WAVC: i16 = 4;
/// 0x0005
pub const ResourceType_WFX: i16 = 5;
/// 0x0006
pub const ResourceType_PLT: i16 = 6;
/// 0x03e8
pub const ResourceType_BAM: i16 = 1000;
/// 0x03e8
pub const ResourceType_BAMC: i16 = 1000;
/// 0x03e9
pub const ResourceType_WED: i16 = 1001;
/// 0x03ea
pub const ResourceType_CHU: i16 = 1002;
/// 0x03eb
pub const ResourceType_TIS: i16 = 1003;
/// 0x03ec
pub const ResourceType_MOS: i16 = 1004;
/// 0x03ec
pub const ResourceType_MOSC: i16 = 1004;
/// 0x03ed
pub const ResourceType_ITM: i16 = 1005;
/// 0x03ee
pub const ResourceType_SPL: i16 = 1006;
/// 0x03ef
pub const ResourceType_BCS: i16 = 1007;
/// 0x03f0
pub const ResourceType_IDS: i16 = 1008;
/// 0x03f1
pub const ResourceType_CRE: i16 = 1009;
/// 0x03f2
pub const ResourceType_ARE: i16 = 1010;
/// 0x03f3
pub const ResourceType_DLG: i16 = 1011;
/// 0x03f4
pub const ResourceType_TwoDA: i16 = 1012;
/// 0x03f5
pub const ResourceType_GAM: i16 = 1013;
/// 0x03f6
pub const ResourceType_STO: i16 = 1014;
/// 0x03f7
pub const ResourceType_WMP: i16 = 1015;
/// 0x03f8
pub const ResourceType_CHR: i16 = 1016;
/// 0x03f8
pub const ResourceType_EFF: i16 = 1016;
/// 0x03f9
pub const ResourceType_BS: i16 = 1017;
/// 0x03fa
pub const ResourceType_CHR2: i16 = 1018;
/// 0x03fb
pub const ResourceType_VVC: i16 = 1019;
/// 0x03fc
pub const ResourceType_VEF: i16 = 1020;
/// 0x03fd
pub const ResourceType_PRO: i16 = 1021;
/// 0x03fe
pub const ResourceType_BIO: i16 = 1022;
/// 0x03ff
pub const ResourceType_WBM: i16 = 1023;
/// 0x0400
pub const ResourceType_FNT: i16 = 1024;
/// 0x0402
pub const ResourceType_GUI: i16 = 1026;
/// 0x0403
pub const ResourceType_SQL: i16 = 1027;
/// 0x0404
pub const ResourceType_PVRZ: i16 = 1028;
/// 0x0405
pub const ResourceType_GLSL: i16 = 1029;
/// 0x0408
pub const ResourceType_MENU: i16 = 1032;
/// 0x0409
pub const ResourceType_MENU2: i16 = 1033;
/// 0x040a
pub const ResourceType_TTF: i16 = 1034;
/// 0x040b
pub const ResourceType_PNG: i16 = 1035;
/// 0x044c
pub const ResourceType_BAH: i16 = 1100;
/// 0x0802
pub const ResourceType_INI: i16 = 2050;
/// 0x0803
pub const ResourceType_SRC: i16 = 2051;

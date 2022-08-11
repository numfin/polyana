pub enum SampleSize {
    I16(i16),
    U16(u16),
    F32(f32),
}

// impl SampleSize {
//     fn to_i16(self) -> i16 {
//         match self {
//             Self::F32(v) => v as i16,
//             Self::U16(v) => v as i16,
//             Self::I16(v) => v,
//         }
//     }
//     fn to_f32(self) -> i16 {
//         match self {
//             Self::U16(v) => v as f32,
//             Self::I16(v) => v as ,
//             Self::F32(v) => v as i16,
//         }
//     }
//     fn to_f32(self) -> i16 {
//         match self {
//             Self::F32(v) => v as i16,
//             Self::U16(v) => v as i16,
//             Self::I16(v) => v,
//         }
//     }
// }

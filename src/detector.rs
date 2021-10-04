use std::time::{UNIX_EPOCH, SystemTime};
use std::fmt::{Display, Formatter};
use core_simd::u64x8;

pub struct SingleEventUpset {
    time: SystemTime,
    bits: [u8; 64],
    address: String,
    value: u64
}

impl SingleEventUpset {
    fn new(num: &u64) -> SingleEventUpset {
        // get the value of each bit
        let mut bits = [0; 64];
        for (index, bit) in (0..63).map(|n| num & (1 << n)).enumerate() {
            bits[index] = bit as u8;
        }

        SingleEventUpset {time: SystemTime::now(), bits, address: format!("{:p}", num), value: *num}
    }
}

impl Display for SingleEventUpset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.time
            .duration_since(UNIX_EPOCH)
            .expect("Single Event Upset time is earlier than EPOCH")
            .as_secs_f64();

        let bits = self.bits
            .iter()
            .map(|bit| format!("{}", bit))
            .collect::<Vec<String>>()
            .join("|");

        write!(f, "Single Event Upset detected - timestamp: {}, address: {}, bits: {}, value: {}",
               timestamp , self.address, bits, self.value)
    }
}

pub struct Detector {
    simd_vecs: Vec<u64x8>
}

impl Detector {
    pub fn new(bytes: usize) -> Detector {
        Detector { simd_vecs: vec![u64x8::splat(0); bytes / 8 / 8] }
    }

    pub fn from_vec(simd_vecs: Vec<u64x8>) -> Detector {
        Detector { simd_vecs }
    }

    pub fn get_upsets(&self) -> Vec<SingleEventUpset> {
        // Because this will almost always be unused, we leave it unallocated
        let mut upsets = Vec::with_capacity(0);

        for simd_vec in &self.simd_vecs {
            // Use bitwise or to detect none zero values
            if 0 != simd_vec.horizontal_or() {
                // Find the upset integer and record info
                for lane in simd_vec.to_array().iter() {
                    if *lane != 0 {
                        upsets.push(SingleEventUpset::new(lane));
                    }
                }
            }
        }

        upsets
    }

    pub fn reset(&mut self) {
        self.simd_vecs.fill(u64x8::splat(0));
    }
}
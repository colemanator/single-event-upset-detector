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
    pub fn new(num: &u64) -> SingleEventUpset {
        // get the value of each bit
        let mut bits = [0; 64];
        for (index, bit) in (0..63).map(|n| num & (1 << n)).enumerate() {
            if bit > 0 {
                bits[index] = 1;
            } else {
                bits[index] = 0;
            }
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

    pub fn get_num_bytes(&self) -> usize { self.simd_vecs.len() * 8 * 8 }
}

#[cfg(test)]
mod test {
    use core_simd::u64x8;
    use crate::detector::{Detector, SingleEventUpset};

    #[test]
    fn can_get_upsets() {
        let mut simd_vecs = vec![u64x8::splat(0); 5];
        simd_vecs.push(u64x8::from_array([0, 0, 0, 0, 0, 0, 0, 1]));

        let detector = Detector::from_vec(simd_vecs);

        assert!(!detector.get_upsets().is_empty());
    }

    #[test]
    fn can_get_no_upsets() {
        let simd_vecs = vec![u64x8::splat(0); 5];
        let detector = Detector::from_vec(simd_vecs);

        assert!(detector.get_upsets().is_empty());
    }

    #[test]
    fn can_reset() {
        let simd_vecs = vec![u64x8::splat(1); 5];
        let mut detector = Detector::from_vec(simd_vecs);

        detector.reset();

        assert!(detector.get_upsets().is_empty());
    }

    #[test]
    fn can_get_num_bytes() {
        let simd_vecs = vec![u64x8::splat(0); 5];
        let detector = Detector::from_vec(simd_vecs);

        assert_eq!(320, detector.get_num_bytes());
    }

    #[test]
    fn can_create_single_event_upset() {
        let num = 42;
        let upset = SingleEventUpset::new(&num);

        let bits: [u8; 64] = [0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(42, upset.value);
        assert_eq!(bits, upset.bits);
        assert_eq!(format!("{:p}", &num), upset.address);
    }
}
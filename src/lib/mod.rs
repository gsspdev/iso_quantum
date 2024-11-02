use deku::bitvec::BitVec;

type BitArray = [u8; 4];

const ARR: BitArray = [0, 1, 2, 3];
const BITS: BitVec<u8, Msb0> = BitVec::from_slice(&ARR);
const DATA: u16 = 0x2021u16;

pub fn test_print_data() {
    println!("ARR: {:?}", ARR);
    println!("BITS: {:?}", BITS);
    println!("DATA: {:#06x}", DATA);
}

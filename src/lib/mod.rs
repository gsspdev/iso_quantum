use deku::bitvec::BitVec;
use bitvec::prelude::Msb0;

type BitArray = [u8; 4];

const ARR: BitArray = [0, 1, 2, 3];
const DATA: u16 = 0x2021u16;

pub fn test_print_data() {
    let bits: BitVec<u8, Msb0> = BitVec::from_slice(&ARR).clone();

    let slice0: BitVec<u8, Msb0> = BitVec::from_element(0);
    let slice1: BitVec<u8, Msb0> = BitVec::from_element( 1);
    let slice2: BitVec<u8, Msb0> = BitVec::from_element( 2);
    let slice3: BitVec<u8, Msb0> = BitVec::from_element( 3);

    println!("ARR: {:?}", ARR);

    println!("slice: {:?}", slice0);
    println!("slice: {:?}", slice1);
    println!("slice: {:?}", slice2);
    println!("slice: {:?}", slice3);

    println!("{}", ("putting these together").to_string());

    println!("bits: {:?}", bits);
    println!("DATA: {:#06x}", DATA);
}

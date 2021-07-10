extern crate curve25519_dalek;

#[derive(Copy, Clone)]
pub struct FieldElement51(pub (crate) [u64; 5]);

impl FieldElement51 {
    /// Serialize this `FieldElement51` to a 32-byte array.  The
    /// encoding is canonical.
    pub fn to_bytes(&self) -> [u8; 32] {
        // Let h = limbs[0] + limbs[1]*2^51 + ... + limbs[4]*2^204.
        //
        // Write h = pq + r with 0 <= r < p.
        //
        // We want to compute r = h mod p.
        //
        // If h < 2*p = 2^256 - 38,
        // then q = 0 or 1,
        //
        // with q = 0 when h < p
        //  and q = 1 when h >= p.
        //
        // Notice that h >= p <==> h + 19 >= p + 19 <==> h + 19 >= 2^255.
        // Therefore q can be computed as the carry bit of h + 19.

        // First, reduce the limbs to ensure h < 2*p.
        let mut limbs = FieldElement51::reduce(self.0).0;

        let mut q = (limbs[0] + 19) >> 51;
        q = (limbs[1] + q) >> 51;
        q = (limbs[2] + q) >> 51;
        q = (limbs[3] + q) >> 51;
        q = (limbs[4] + q) >> 51;

        // Now we can compute r as r = h - pq = r - (2^255-19)q = r + 19q - 2^255q

        limbs[0] += 19*q;

        // Now carry the result to compute r + 19q ...
        let low_51_bit_mask = (1u64 << 51) - 1;
        limbs[1] +=  limbs[0] >> 51;
        limbs[0] = limbs[0] & low_51_bit_mask;
        limbs[2] +=  limbs[1] >> 51;
        limbs[1] = limbs[1] & low_51_bit_mask;
        limbs[3] +=  limbs[2] >> 51;
        limbs[2] = limbs[2] & low_51_bit_mask;
        limbs[4] +=  limbs[3] >> 51;
        limbs[3] = limbs[3] & low_51_bit_mask;
        // ... but instead of carrying (limbs[4] >> 51) = 2^255q
        // into another limb, discard it, subtracting the value
        limbs[4] = limbs[4] & low_51_bit_mask;

        // Now arrange the bits of the limbs.
        let mut s = [0u8;32];
        s[ 0] =   limbs[0]        as u8;
        s[ 1] =  (limbs[0] >>  8) as u8;
        s[ 2] =  (limbs[0] >> 16) as u8;
        s[ 3] =  (limbs[0] >> 24) as u8;
        s[ 4] =  (limbs[0] >> 32) as u8;
        s[ 5] =  (limbs[0] >> 40) as u8;
        s[ 6] = ((limbs[0] >> 48) | (limbs[1] << 3)) as u8;
        s[ 7] =  (limbs[1] >>  5) as u8;
        s[ 8] =  (limbs[1] >> 13) as u8;
        s[ 9] =  (limbs[1] >> 21) as u8;
        s[10] =  (limbs[1] >> 29) as u8;
        s[11] =  (limbs[1] >> 37) as u8;
        s[12] = ((limbs[1] >> 45) | (limbs[2] << 6)) as u8;
        s[13] =  (limbs[2] >>  2) as u8;
        s[14] =  (limbs[2] >> 10) as u8;
        s[15] =  (limbs[2] >> 18) as u8;
        s[16] =  (limbs[2] >> 26) as u8;
        s[17] =  (limbs[2] >> 34) as u8;
        s[18] =  (limbs[2] >> 42) as u8;
        s[19] = ((limbs[2] >> 50) | (limbs[3] << 1)) as u8;
        s[20] =  (limbs[3] >>  7) as u8;
        s[21] =  (limbs[3] >> 15) as u8;
        s[22] =  (limbs[3] >> 23) as u8;
        s[23] =  (limbs[3] >> 31) as u8;
        s[24] =  (limbs[3] >> 39) as u8;
        s[25] = ((limbs[3] >> 47) | (limbs[4] << 4)) as u8;
        s[26] =  (limbs[4] >>  4) as u8;
        s[27] =  (limbs[4] >> 12) as u8;
        s[28] =  (limbs[4] >> 20) as u8;
        s[29] =  (limbs[4] >> 28) as u8;
        s[30] =  (limbs[4] >> 36) as u8;
        s[31] =  (limbs[4] >> 44) as u8;

        // High bit should be zero.
        debug_assert!((s[31] & 0b1000_0000u8) == 0u8);

        s
    }

    /// Given 64-bit input limbs, reduce to enforce the bound 2^(51 + epsilon).
    #[inline(always)]
    fn reduce(mut limbs: [u64; 5]) -> FieldElement51 {
        const LOW_51_BIT_MASK: u64 = (1u64 << 51) - 1;

        // Since the input limbs are bounded by 2^64, the biggest
        // carry-out is bounded by 2^13.
        //
        // The biggest carry-in is c4 * 19, resulting in
        //
        // 2^51 + 19*2^13 < 2^51.0000000001
        //
        // Because we don't need to canonicalize, only to reduce the
        // limb sizes, it's OK to do a "weak reduction", where we
        // compute the carry-outs in parallel.

        let c0 = limbs[0] >> 51;
        let c1 = limbs[1] >> 51;
        let c2 = limbs[2] >> 51;
        let c3 = limbs[3] >> 51;
        let c4 = limbs[4] >> 51;

        limbs[0] &= LOW_51_BIT_MASK;
        limbs[1] &= LOW_51_BIT_MASK;
        limbs[2] &= LOW_51_BIT_MASK;
        limbs[3] &= LOW_51_BIT_MASK;
        limbs[4] &= LOW_51_BIT_MASK;

        limbs[0] += c4 * 19;
        limbs[1] += c0;
        limbs[2] += c1;
        limbs[3] += c2;
        limbs[4] += c3;

        FieldElement51(limbs)
    }
}


fn main() {
    print!("{:?}", FieldElement51([
        u64::from_str_radix(&std::env::args().nth(1).expect("missing a field element"), 10).expect("couldn't parse int"),
        u64::from_str_radix(&std::env::args().nth(2).expect("missing a field element"), 10).expect("couldn't parse int"),
        u64::from_str_radix(&std::env::args().nth(3).expect("missing a field element"), 10).expect("couldn't parse int"),
        u64::from_str_radix(&std::env::args().nth(4).expect("missing a field element"), 10).expect("couldn't parse int"),
        u64::from_str_radix(&std::env::args().nth(5).expect("missing a field element"), 10).expect("couldn't parse int"),
    ]).to_bytes());

    /*
    println!("
pub(crate) const APLUS2_OVER_FOUR: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([121666, 0, 0, 0, 0]).to_bytes());

    println!("
pub(crate) const MONTGOMERY_A: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([486662, 0, 0, 0, 0]).to_bytes());


    println!("
pub(crate) const EDWARDS_D: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([
        929955233495203,
        466365720129213,
        1662059464998953,
        2033849074728123,
        1442794654840575,
    ]).to_bytes());

    println!("
pub(crate) const EDWARDS_D2: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([
        1859910466990425,
        932731440258426,
        1072319116312658,
        1815898335770999,
        633789495995903,
    ]).to_bytes());

    println!("
pub(crate) const SQRT_M1: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([
        1718705420411056,
        234908883556509,
        2233514472574048,
        2117202627021982,
        765476049583133,
    ]).to_bytes());

    println!("
pub(crate) const INVSQRT_A_MINUS_D: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([
        278908739862762,
        821645201101625,
        8113234426968,
        1777959178193151,
        2118520810568447,
    ]).to_bytes());

    println!("
pub(crate) const SQRT_AD_MINUS_ONE: Engine25519 =
    Engine25519({:?});
    ", FieldElement51([
        2241493124984347,
        425987919032274,
        2207028919301688,
        1220490630685848,
        974799131293748,
    ]).to_bytes());

    println!("
pub const ED25519_BASEPOINT_POINT: EdwardsPoint = EdwardsPoint {{
    X: Engine25519({:?}),
    Y: Engine25519({:?}),
    Z: Engine25519({:?}),
    T: Engine25519({:?}),
}};", FieldElement51([
    1738742601995546,
    1146398526822698,
    2070867633025821,
    562264141797630,
    587772402128613,
    ]).to_bytes(),
    FieldElement51([
        1801439850948184,
        1351079888211148,
        450359962737049,
        900719925474099,
        1801439850948198,
    ]).to_bytes(),
    FieldElement51([1, 0, 0, 0, 0]).to_bytes(),
    FieldElement51([
        1841354044333475,
        16398895984059,
        755974180946558,
        900171276175154,
        1821297809914039,
    ]).to_bytes(),
    );

    println!("{:?}",         FieldElement51([2251799813685228, 2251799813685247, 2251799813685247, 2251799813685247, 2251799813685247]).to_bytes()
    );*/
}
use glam::IVec3;
use util::identifier::{Identifier, IdentifierBuf};

pub trait RandomSource {
    fn fork(&mut self) -> Self;
    fn fork_positional(&mut self) -> impl PositionalRandomFactory;
    fn set_seed(&mut self, seed: u64);
    fn next_u32_unbounded(&mut self) -> u32;
    fn next_u32(&mut self, bound: u32) -> u32;

    fn next_i32_between_inclusive(&mut self, min: i32, max: i32) -> i32 {
        assert!(min <= max);
        self.next_u32((max - min + 1) as u32) as i32 + min
    }
    fn next_u64(&mut self) -> u64;
    fn next_bool(&mut self) -> bool;
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;
    fn next_gaussian(&mut self) -> f64;
    fn triangle(&mut self, middle: f64, spread: f64) -> f64 {
        middle + spread * (self.next_f64() - self.next_f64())
    }
    fn consume_count(&mut self, count: u64) {
        for _ in 0..count {
            self.next_u32_unbounded();
        }
    }
    fn next_i32_between(&mut self, origin: i32, bound: i32) -> i32 {
        assert!(origin < bound, "bound - origin is non positive");
        origin + self.next_u32((origin - bound) as u32) as i32
    }
}

#[derive(Debug)]
pub struct LegacyRandomSource {
    seed: u64,
    next_next_gaussian: Option<f64>,
}

impl LegacyRandomSource {
    const MULTIPLIER: u64 = 0x5deece66d;
    const ADDEND: u64 = 11;
    const MASK: u64 = (1 << 48) - 1;

    #[inline]
    pub fn new(seed: u64) -> LegacyRandomSource {
        LegacyRandomSource {
            seed: (seed ^ Self::MULTIPLIER) & Self::MASK,
            next_next_gaussian: None,
        }
    }

    #[inline]
    fn next(&mut self, bits: u32) -> u32 {
        self.seed = self
            .seed
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::ADDEND)
            & Self::MASK;
        (self.seed >> (48 - bits)) as u32
    }
}

impl RandomSource for LegacyRandomSource {
    #[inline]
    fn fork(&mut self) -> Self {
        LegacyRandomSource::new(self.next_u64())
    }

    #[inline]
    fn fork_positional(&mut self) -> impl PositionalRandomFactory {
        LegacyPositionalRandomFactory {
            seed: self.next_u64(),
        }
    }

    #[inline]
    fn set_seed(&mut self, seed: u64) {
        *self = LegacyRandomSource::new(seed);
    }

    #[inline]
    fn next_u32_unbounded(&mut self) -> u32 {
        self.next(32)
    }

    #[inline]
    fn next_u32(&mut self, bound: u32) -> u32 {
        assert!(bound > 0, "bound must be positive");
        if bound.is_power_of_two() {
            ((bound as u64 * self.next(31) as u64) >> 31) as u32
        } else {
            loop {
                let u = self.next(31);
                let r = u % bound;
                if r + (bound - 1) <= u {
                    return r;
                }
            }
        }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let hi = self.next(32);
        let lo = self.next(32);
        (((hi as u64) << 32) as i64).wrapping_add(lo as i32 as i64) as u64
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        self.next(1) != 0
    }

    #[inline]
    fn next_f32(&mut self) -> f32 {
        (self.next(24) as f32) * (1.0 / (1 << 24) as f32)
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        let hi = self.next(26);
        let lo = self.next(27);
        let l = ((hi as u64) << 27) + lo as u64;
        (l as f64) * (1.0 / (1u64 << 53) as f64)
    }

    #[inline]
    fn next_gaussian(&mut self) -> f64 {
        if let Some(next_next_gaussian) = self.next_next_gaussian.take() {
            next_next_gaussian
        } else {
            let (g1, g2) = next_gaussian(|| self.next_f64());
            self.next_next_gaussian = Some(g2);
            g1
        }
    }
}

#[derive(Debug)]
pub struct XoroshiroRandomSource {
    seed_lo: u64,
    seed_hi: u64,
    next_next_gaussian: Option<f64>,
}

impl XoroshiroRandomSource {
    const GOLDEN_RATIO_64: u64 = 11400714819323198485;
    const SILVER_RATIO_64: u64 = 7640891576956012809;

    #[inline]
    fn mix_stafford_13(mut n: u64) -> u64 {
        n = (n ^ n >> 30).wrapping_mul(13787848793156543929);
        n = (n ^ n >> 27).wrapping_mul(10723151780598845931);
        n ^ n >> 31
    }

    #[inline]
    pub fn new(seed: u64) -> XoroshiroRandomSource {
        let lo = seed ^ Self::SILVER_RATIO_64;
        let hi = seed.wrapping_add(Self::GOLDEN_RATIO_64);
        Self::new128(Self::mix_stafford_13(lo), Self::mix_stafford_13(hi))
    }

    #[inline]
    pub fn new128(mut seed_lo: u64, mut seed_hi: u64) -> XoroshiroRandomSource {
        if seed_lo == 0 && seed_hi == 0 {
            seed_lo = Self::GOLDEN_RATIO_64;
            seed_hi = Self::SILVER_RATIO_64;
        }
        XoroshiroRandomSource {
            seed_lo,
            seed_hi,
            next_next_gaussian: None,
        }
    }
}

impl RandomSource for XoroshiroRandomSource {
    #[inline]
    fn fork(&mut self) -> Self {
        XoroshiroRandomSource::new128(self.next_u64(), self.next_u64())
    }

    #[inline]
    fn fork_positional(&mut self) -> impl PositionalRandomFactory {
        XoroshiroPositionalRandomFactory {
            seed_lo: self.next_u64(),
            seed_hi: self.next_u64(),
        }
    }

    #[inline]
    fn set_seed(&mut self, seed: u64) {
        *self = XoroshiroRandomSource::new(seed);
    }

    #[inline]
    fn next_u32_unbounded(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u32(&mut self, bound: u32) -> u32 {
        assert!(bound > 0, "bound must be positive");
        let mut next_int = self.next_u32_unbounded() as u64;
        let mut n_long = next_int * bound as u64;
        let mut n = n_long as u32;
        if n < bound {
            while n < (!bound + 1) % bound {
                next_int = self.next_u32_unbounded() as u64;
                n_long = next_int * bound as u64;
                n = n_long as u32;
            }
        }

        (n_long >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result = self
            .seed_lo
            .wrapping_add(self.seed_hi)
            .rotate_left(17)
            .wrapping_add(self.seed_lo);
        self.seed_hi ^= self.seed_lo;
        self.seed_lo = self.seed_lo.rotate_left(49) ^ self.seed_hi ^ (self.seed_hi << 21);
        self.seed_hi = self.seed_hi.rotate_left(28);
        result
    }

    #[inline]
    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) != 0
    }

    #[inline]
    fn next_f32(&mut self) -> f32 {
        let bits = self.next_u64() >> 40;
        (bits as f32) * (1.0 / (1 << 24) as f32)
    }

    #[inline]
    fn next_f64(&mut self) -> f64 {
        let bits = self.next_u64() >> 11;
        (bits as f64) * (1.0 / (1u64 << 53) as f64)
    }

    #[inline]
    fn next_gaussian(&mut self) -> f64 {
        if let Some(next_next_gaussian) = self.next_next_gaussian.take() {
            next_next_gaussian
        } else {
            let (g1, g2) = next_gaussian(|| self.next_f64());
            self.next_next_gaussian = Some(g2);
            g1
        }
    }
}

#[inline]
fn next_gaussian(mut f64_source: impl FnMut() -> f64) -> (f64, f64) {
    loop {
        let v1 = 2.0 * f64_source() - 1.0;
        let v2 = 2.0 * f64_source() - 1.0;
        let s = v1 * v1 + v2 * v2;
        if s < 1.0 && s != 0.0 {
            let multiplier = (-2.0 * rust_strictmath::log(s) / s).sqrt();
            return (v1 * multiplier, v2 * multiplier);
        }
    }
}

pub trait PositionalRandomFactory {
    type Hash;

    fn at(&self, pos: IVec3) -> impl RandomSource;
    fn from_seed(&self, seed: u64) -> impl RandomSource;
    fn from_hash(&self, hash: Self::Hash) -> impl RandomSource;
    fn hash<T>(&self, value: T) -> Self::Hash
    where
        T: Hashable;
    fn from_hash_of<T>(&self, value: T) -> impl RandomSource
    where
        T: Hashable,
    {
        self.from_hash(self.hash(value))
    }
}

pub trait Hashable {
    fn digest_md5(&self, context: &mut md5::Context);
    fn chars_utf16(&self) -> impl Iterator<Item = u16>;
}

impl Hashable for str {
    #[inline]
    fn digest_md5(&self, context: &mut md5::Context) {
        context.consume(self)
    }

    #[inline]
    fn chars_utf16(&self) -> impl Iterator<Item = u16> {
        str::encode_utf16(self)
    }
}

impl Hashable for String {
    #[inline]
    fn digest_md5(&self, context: &mut md5::Context) {
        context.consume(self)
    }

    #[inline]
    fn chars_utf16(&self) -> impl Iterator<Item = u16> {
        str::encode_utf16(self)
    }
}

impl Hashable for Identifier {
    #[inline]
    fn digest_md5(&self, context: &mut md5::Context) {
        let (namespace, path) = self.namespace_and_path();
        context.consume(namespace);
        context.consume([b':']);
        context.consume(path);
    }

    #[inline]
    fn chars_utf16(&self) -> impl Iterator<Item = u16> {
        let (namespace, path) = self.namespace_and_path();
        namespace
            .encode_utf16()
            .chain(std::iter::once(b':' as u16))
            .chain(path.encode_utf16())
    }
}

impl Hashable for IdentifierBuf {
    #[inline]
    fn digest_md5(&self, context: &mut md5::Context) {
        <Identifier as Hashable>::digest_md5(self, context)
    }

    #[inline]
    fn chars_utf16(&self) -> impl Iterator<Item = u16> {
        <Identifier as Hashable>::chars_utf16(self)
    }
}

#[derive(Debug)]
struct LegacyPositionalRandomFactory {
    seed: u64,
}

impl PositionalRandomFactory for LegacyPositionalRandomFactory {
    type Hash = i32;

    #[inline]
    fn at(&self, pos: IVec3) -> impl RandomSource {
        LegacyRandomSource::new(get_seed(pos) ^ self.seed)
    }

    #[inline]
    fn from_seed(&self, seed: u64) -> impl RandomSource {
        LegacyRandomSource::new(seed)
    }

    #[inline]
    fn from_hash(&self, hash: i32) -> impl RandomSource {
        LegacyRandomSource::new(hash as i64 as u64 ^ self.seed)
    }

    fn hash<T>(&self, value: T) -> i32
    where
        T: Hashable,
    {
        let mut hash: i32 = 0;
        for char in value.chars_utf16() {
            hash = hash.wrapping_mul(31).wrapping_add(char as i32);
        }
        hash
    }
}

#[derive(Debug)]
struct XoroshiroPositionalRandomFactory {
    seed_lo: u64,
    seed_hi: u64,
}

impl PositionalRandomFactory for XoroshiroPositionalRandomFactory {
    type Hash = [u8; 16];

    #[inline]
    fn at(&self, pos: IVec3) -> impl RandomSource {
        XoroshiroRandomSource::new128(get_seed(pos) ^ self.seed_lo, self.seed_hi)
    }

    #[inline]
    fn from_seed(&self, seed: u64) -> impl RandomSource {
        XoroshiroRandomSource::new128(seed ^ self.seed_lo, seed ^ self.seed_hi)
    }

    #[inline]
    fn from_hash(&self, hash: [u8; 16]) -> impl RandomSource {
        let mut lower_hash = [0; 8];
        lower_hash.copy_from_slice(&hash[..8]);
        let lower = u64::from_be_bytes(lower_hash);
        let mut upper_hash = [0; 8];
        upper_hash.copy_from_slice(&hash[8..]);
        let upper = u64::from_be_bytes(upper_hash);
        XoroshiroRandomSource::new128(lower ^ self.seed_lo, upper ^ self.seed_hi)
    }

    fn hash<T>(&self, value: T) -> [u8; 16]
    where
        T: Hashable,
    {
        let mut context = md5::Context::new();
        value.digest_md5(&mut context);
        context.compute().into()
    }
}

fn get_seed(pos: IVec3) -> u64 {
    let mut n =
        pos.x.wrapping_mul(3129871) as i64 ^ (pos.z as i64).wrapping_mul(116129781) ^ pos.y as i64;
    n = n
        .wrapping_mul(n)
        .wrapping_mul(42317861)
        .wrapping_add(n.wrapping_mul(11));
    (n >> 16) as u64
}

#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;

    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use test::Bencher;

    const KEY_SIZE: usize = 24;
    const VAL_SIZE: usize = 4;
    const LEN: usize = 32;

    fn gen_pairs<R: Rng>(rng: &mut R) -> HashMap<[u8; KEY_SIZE], [u8; VAL_SIZE]> {
        let mut map = HashMap::with_capacity(LEN);

        while map.len() < LEN {
            let mut key = [0; KEY_SIZE];
            let mut val = [0; VAL_SIZE];
            rng.fill(&mut key);
            rng.fill(&mut val);
            map.insert(key, val);
        }

        map
    }

    fn next_key<'a, R: Rng>(rng: &mut R, keys: &'a [[u8; KEY_SIZE]]) -> &'a [u8; KEY_SIZE] {
        &keys[rng.gen_range(0..LEN)]
    }

    macro_rules! generate_bench {
        ($name:ident, $hasher:path) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                type BuildHasher = BuildHasherDefault<$hasher>;

                let mut rng = SmallRng::from_entropy();
                let mut map = HashMap::with_capacity_and_hasher(LEN, BuildHasher::default());
                let mut keys: [[u8; KEY_SIZE]; LEN] = Default::default();

                for (i, (key, val)) in gen_pairs(&mut rng).into_iter().enumerate() {
                    keys[i] = key;
                    map.insert(key, val);
                }

                b.iter(|| {
                    map.get(next_key(&mut rng, &keys));
                });
            }
        };
    }

    generate_bench!(std, std::collections::hash_map::DefaultHasher);
    generate_bench!(ahash, ahash::AHasher);
    generate_bench!(fxhash, fxhash::FxHasher);
    generate_bench!(fxhash32, fxhash::FxHasher32);
    generate_bench!(siphasher13, siphasher::sip::SipHasher13);
    generate_bench!(siphasher24, siphasher::sip::SipHasher24);
    generate_bench!(metrohash64, metrohash::MetroHash64);
    generate_bench!(wyhash, wyhash::WyHash);
    generate_bench!(t1h1, t1ha::T1haHasher);
}

fn main() {}

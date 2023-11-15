//! HashMap from stand library
use hashbrown::hash_map as base;

#[allow(deprecated)]
use core::hash::{BuildHasher, Hasher, Hash, SipHasher13};
// Soft random
use crate::time::current_ticks;


pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V> HashMap<K, V, RandomState> {
    /// init for Hashmap default
    #[inline] 
    #[must_use]
    pub fn new() -> HashMap<K, V, RandomState> {
        Default::default()
    }
}

impl<K, V, S> HashMap<K, V, S> {
    /// with_hahser
    #[inline]
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_hasher(hash_builder) }
    }
    /// iter
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
}

// The S must have the Default trait.
impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    /// Creates an empty `HashMap<K, V, S>`, with the `Default` value for hasher.
    #[inline]
    fn default() -> Self {
        HashMap::with_hasher(Default::default())  
    }
}

// The K must have the Eq and Hash trait.
// S must have BuildHasher trait.
impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    // insert
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
}

// Iter struct
pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

// implement the Clone trait for Iter
impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { base: self.base.clone() }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

/// Random key
fn hashmap_random_keys() -> (u64, u64) {
    let time = current_ticks();
    let r1 = ((time & ((1 << 64) -1)) * 32768 + 7658) as u64; 
    let r2 = ((time >> 64) * 32768 + 77658) as u64;
    (r1, r2)
}

/// `RandomState` is the default state for [`HahsMap`] types.
#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    /// Constructs a new `RandomState` that is initialized with random key 
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn new() -> RandomState {
        let keys: (u64, u64) = hashmap_random_keys();

        RandomState {
            k0: keys.0, 
            k1: keys.1,
        }
    }
}

impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
    }
}

#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct DefaultHasher(SipHasher13);

impl DefaultHasher {
    /// Create a new `DefualtHasher` 
    #[inline]
    #[allow(deprecated)]
    #[must_use]
    pub fn new() -> DefaultHasher {
        DefaultHasher(SipHasher13::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    /// Create a new `DefaultHasher` using [`new`]
    #[inline]
    fn default() -> DefaultHasher {
        DefaultHasher::new() 
    }
}

// #[stable(feature = "hashmap_default_hasher", since = "1.13.0")]
impl Hasher for DefaultHasher {
    // The underlying `SipHasher13` doesn't override the other
    // `write_*` methods, so it's ok not to forward them here.

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

// implement the Default for RandomState
impl Default for RandomState {
    /// Constructs a new `RandomState`.
    #[inline]
    fn default() -> RandomState {
        RandomState::new()
    }
}
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::{Cycle, Skip, Take};
use std::slice::Iter;
use uint::construct_uint;
use tari_utilities::hex;
use rand::{thread_rng, RngCore};
use tari_crypto::hash::blake2::Blake256;
use digest::Digest;

// U1024 with 1024 bits consisting of 16 x 64-bit words
construct_uint! {
	pub struct U256(4);
}

pub type CommitteeIter<'a> = Take<Skip<Cycle<Iter<'a, VnAddress>>>>;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ShardAddress {
    value: U256
}

impl Display for ShardAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut bytes = [0u8; 32];
        self.value.to_big_endian(&mut bytes);
        f.write_str(hex::to_hex(&bytes).as_str())
    }
}

impl ShardAddress {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        thread_rng().fill_bytes(&mut bytes);
        Self {
            value: U256::from_little_endian(&bytes)
        }
    }

    pub fn value(&self) -> U256 {
        self.value
    }

    pub fn as_hex(&self) -> String {
        self.value.to_string()
    }
}

//---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VnAddress {
    vn_key: ShardAddress,
    address: ShardAddress,
}

impl Display for VnAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.vn_key, self.address)
    }
}

impl PartialOrd for VnAddress {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.address.value.partial_cmp(&other.address.value)
    }
}

impl Ord for VnAddress {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.value.cmp(&other.address.value)
    }
}

impl VnAddress {
    pub fn new(epoch: u64, num_buckets: u64) -> Self {
        let vn_key = ShardAddress::random();
        let address = Self::address_at_epoch(&vn_key, 0, num_buckets);
        Self {
            vn_key,
            address,
        }
    }

    pub fn address(&self) -> &ShardAddress {
        &self.address
    }

    pub fn key(&self) -> &ShardAddress {
        &self.vn_key
    }

    pub fn is_in_committee(&self, committee: &[VnAddress]) -> bool {
        committee.iter().any(|vn| vn.vn_key == self.vn_key)
    }

    // In prod, we'd want an additional source of entropy in the hash, taken from a block hash or something. But for
    // this, we can just use the vn_key + epoch
    pub fn address_at_epoch(vn_key: &ShardAddress, epoch: u64, num_buckets: u64) -> ShardAddress {
        let hasher = Blake256::new();
        let le_bytes: [u8; 32] = vn_key.value.into();
        let shuffle_counter = (vn_key.value + epoch) / num_buckets;
        let counter_bytes: [u8; 32] = shuffle_counter.into();
        let address = hasher.chain(&le_bytes)
            .chain(&counter_bytes)
            .finalize();
        let address = U256::from_little_endian(address.as_ref());
        ShardAddress {
            value: address
        }

    }
}


pub struct ValidatorNodes {
    pub committee_size: u64,
    pub vns: Vec<VnAddress>,
    pub epoch: u64,
    pub num_buckets: u64,
}

impl Display for ValidatorNodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- Epoch {}, committee size: {} --", self.epoch, self.committee_size)?;
        for vn in self.vns.iter() {
            writeln!(f, "{vn}")?
        }
        Ok(())
    }
}

impl ValidatorNodes {
    pub fn new(committee_size: u64, num_vns: u64, num_buckets: u64) -> Self {
        let mut vns = Vec::new();
        let epoch = 0;
        for _ in 0..num_vns {
            let vn = VnAddress::new(epoch, num_buckets);
            match vns.binary_search(&vn) {
                Ok(_) => panic!("Duplicate VN address"),
                Err(pos) => {
                    vns.insert(pos, vn)
                },
            }
        }
        Self {
            committee_size,
            vns,
            epoch,
            num_buckets,
        }
    }

    /// Return the committee for the shard by returning the n + committee_size VN addresses
    pub fn committee_for<'a, 'b>(&'a self, shard: &'b ShardAddress) -> CommitteeIter<'a> {
        let start = match self.vns.binary_search_by_key(&shard, |vn| vn.address()) {
            Ok(pos) => pos,
            Err(pos) => pos
        };
        self.vns.iter().cycle().skip(start).take(self.committee_size as usize)
    }

    pub fn next_epoch(&mut self) {
        self.epoch += 1;
        self.vns.iter_mut().for_each(|vn| {
            vn.address = VnAddress::address_at_epoch(vn.key(), self.epoch, self.num_buckets);
        });
        self.vns.sort();
    }

    pub fn node_population(&self) -> u64 {
        self.vns.len() as u64
    }
}







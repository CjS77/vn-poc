extern crate core;

mod shards;

use shards::*;
use std::env;
use histo::Histogram;

fn main() {
    let mut args = env::args()
        .skip(1)
        .take(2)
        .map(|s| s.parse::<u64>().unwrap());
    let num_buckets = args.next().unwrap();
    let com_size = args.next().unwrap();

    let num_shards = 1_000_000u64;
    let epochs = 50;
    let vn_vec = &[50u64, 100, 500, 1_000, 10_000, 100_000];

    for &num_vns in vn_vec {
        let mut stats = CommitteeStats::new(num_shards, com_size, num_vns, num_buckets);
        stats.simulate(epochs);
        stats.print_stats();
    }
}

fn print_set(vn_set: &ValidatorNodes) {
    println!("--------- VN set - epoch {} ------------", vn_set.epoch);
    vn_set.vns.iter().for_each(|vn| {
        println!("{}: {}", vn.key(), vn.address());
    });
}

fn print_committee_set(shards: &[ShardAddress], set: &Vec<Vec<VnAddress>>) {
    shards.iter()
        .zip(set.iter())
        .for_each(|(shard, committee)| {
            println!("Shard: {shard}");
            println!("------------------------------------------");
            committee.iter().for_each(|vn| println!("{vn}"));
            println!("------------------------------------------\n");
        });
}

struct CommitteeStats {
    pub shards: Vec<ShardAddress>,
    pub change_counts: Vec<Vec<u64>>,
    pub nodes: ValidatorNodes,
    pub last_committee_set: Vec<Vec<VnAddress>>,
    pub hist: Histogram,
    // a shard rotates address every num_buckets epochs
    pub num_buckets: u64,
}

impl CommitteeStats {
    pub fn new(num_shards: u64, committee_size: u64, num_vns: u64, num_buckets: u64) -> Self {
        let mut shards = Vec::with_capacity(num_shards as usize);
        for _ in 0..num_shards {
            shards.push(ShardAddress::random());
        }
        let nodes = ValidatorNodes::new(committee_size, num_vns, num_buckets);
        let last_committee_set = Self::calculate_committees(&shards, &nodes);
        Self {
            shards,
            change_counts: Vec::new(),
            nodes,
            last_committee_set,
            hist: Histogram::with_buckets(u64::min(20, num_vns / 10)),
            num_buckets,
        }
    }

    fn calculate_committees<'a>(shards: &'a [ShardAddress], nodes: &'a ValidatorNodes) -> Vec<Vec<VnAddress>> {
        shards.iter()
            .map(|shard| nodes.committee_for(shard)
                .cloned()
                .collect::<Vec<VnAddress>>()
            ).collect()
    }

    pub fn simulate(&mut self, epochs: u64) {
        // println!("-------------\n{}\n-------------\n\n", self.nodes);
        // print_committee_set(&self.shards, &self.last_committee_set);
        for _ in 1..=epochs {
            self.nodes.next_epoch();
            // println!("-------------\n{}\n-------------\n\n", self.nodes);
            let new_committee_set = Self::calculate_committees(&self.shards, &self.nodes);
            // print_committee_set(&self.shards, &new_committee_set);
            let changes = self.count_changes(&new_committee_set);
            // Update histogram
            changes.iter().for_each(|n| self.hist.add(*n));
            self.change_counts.push(changes);
            self.last_committee_set = new_committee_set;
        }
    }

    fn count_changes(&mut self, new_committee_set: &Vec<Vec<VnAddress>>) -> Vec<u64> {
        new_committee_set.iter()
            .zip(self.last_committee_set.iter())
            .map(|(new, old)| {
                old.iter().fold(0u64, |total, old_vn| {
                    if !old_vn.is_in_committee(new) {
                        total + 1
                    } else {
                        total
                    }
                })
            }).collect::<Vec<u64>>()
    }

    pub fn print_stats(&self) {
        let min_changes = self.nodes.committee_size / self.num_buckets;
        let cov = (100 * self.nodes.committee_size) as f64 / self.nodes.node_population() as f64;
        let num_comms = self.nodes.node_population() / self.nodes.committee_size;
        println!("{} Nodes population. {} committee size. {num_comms} committees. Each node covers {cov:0.2}% space.\n\
        Nodes rotate every {} epochs.\n\
        Expect approx {min_changes} nodes to join each committee each epoch",
                 self.nodes.node_population(), self.nodes.committee_size, self.num_buckets);

        let threshold = (self.nodes.committee_size - 1) / 3;
        let stats = self.change_counts.iter()
            .map(|changes| {
                let change_sum = changes.iter().sum::<u64>() as f64;
                let change_max = changes.iter().max().unwrap_or(&0);
                let change_min = changes.iter().min().unwrap_or(&0);
                let byzantine_count = changes.iter().filter(|n| **n > threshold).count();
                let mean = change_sum / changes.len() as f64;
                (*change_max, *change_min, mean, byzantine_count)
            }).collect::<Vec<_>>();
        for (epoch, (max, min, mean, byzantine_count)) in stats.iter().enumerate() {
            let n = self.nodes.committee_size as f64;
            let ave_pct = 100.0 * mean / n;
            let max_pct = *max as f64 / n * 100.0;
            let min_pct = *min as f64 / n * 100.0;
            println!("Epoch {epoch}: Ave changes {mean:0.2}. Change pct: {ave_pct:0.2}%, Biggest change: {max} \
            ({max_pct:0.1}%), Smallest change: {min} ({min_pct:0.1}%). {byzantine_count} syncing committees.");
        }
        println!("{}", self.hist);
    }
}
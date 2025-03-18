use hdrhistogram::Histogram;

/// One latency histogram per feature-store shard, so a hot shard shows up as its own p99.
pub fn per_shard(shards: usize) -> Vec<Histogram<u64>> { (0..shards).map(|_| Histogram::new(3).unwrap()).collect() }

use criterion::{criterion_group, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rtp::unmarshal", |b| {
        b.iter(|| {
            let raw_pkt = vec![
                0x90, 0xe0, 0x69, 0x8f, 0xd9, 0xc2, 0x93, 0xda, 0x1c, 0x64, 0x27, 0x82, 0xBE, 0xDE,
                0x00, 0x01, 0x50, 0xAA, 0x00, 0x00, 0x98, 0x36, 0xbe, 0x88, 0x9e,
            ];

            let mut reader = std::io::BufReader::new(raw_pkt.as_slice());
            rtp::packet::Packet::unmarshal(&mut reader).expect("Error marshalling data");
        })
    });
}

criterion_group!(unmarshal, criterion_benchmark);

use criterion::criterion_main;

mod marshal;
mod unmarshal;

criterion_main!(marshal::marshal, unmarshal::unmarshal);

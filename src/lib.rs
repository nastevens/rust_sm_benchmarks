#![feature(test)]
extern crate test;

pub mod boxed;
pub mod mem_replace;
pub mod static_ref;

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn bench_boxed(b: &mut Bencher) {
        b.iter(|| assert!(super::boxed::run() > 1_000_000));
    }

    #[bench]
    fn bench_static_ref(b: &mut Bencher) {
        b.iter(|| assert!(super::static_ref::run() > 1_000_000));
    }

    #[bench]
    fn bench_mem_replace(b: &mut Bencher) {
        b.iter(|| assert!(super::mem_replace::run() > 1_000_000));
    }
}

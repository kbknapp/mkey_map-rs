#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn insert1(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::new();
        vec.push("One")
    });
}

#[bench]
fn insert10(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::new();
        for i in 0..10 {
            vec.push(concat!("One", stringify!(i)));
        }
    });
}

#[bench]
fn insert100(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::new();
        for i in 0..100 {
            vec.push(concat!("One", stringify!(i)));
        }
    });
}

#[bench]
fn insert1000(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::new();
        for i in 0..1000 {
            vec.push(concat!("One", stringify!(i)));
        }
    });
}

#[bench]
fn insert100000(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::new();
        for i in 0..100000 {
            vec.push(concat!("One", stringify!(i)));
        }
    });
}

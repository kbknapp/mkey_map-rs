#![feature(test)]

extern crate test;

use std::collections::HashMap;

use test::Bencher;

#[bench]
fn insert1(b: &mut Bencher) {
    b.iter(|| {
        let mut map = HashMap::new();
        map.insert("One", "Value")
    });
}

#[bench]
fn insert10(b: &mut Bencher) {
    b.iter(|| {
        let mut map = HashMap::new();
        for i in 0..10 {
            map.insert(
                concat!("One", stringify!(i)),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#[bench]
fn insert100(b: &mut Bencher) {
    b.iter(|| {
        let mut map = HashMap::new();
        for i in 0..100 {
            map.insert(
                concat!("One", stringify!(i)),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#[bench]
fn insert1000(b: &mut Bencher) {
    b.iter(|| {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(
                concat!("One", stringify!(i)),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#[bench]
fn insert100000(b: &mut Bencher) {
    b.iter(|| {
        let mut map = HashMap::new();
        for i in 0..100000 {
            map.insert(
                concat!("One", stringify!(i)),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#![feature(test)]

extern crate mkey_map;
extern crate test;

use mkey_map::MKeyMap;
use mkey_map::KeyType::*;

use std::ffi::OsStr;

use test::Bencher;

#[bench]
fn insert1(b: &mut Bencher) {
    b.iter(|| {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        map.insert(Long(OsStr::new("One").as_bytes()), "Value")
    });
}

#[bench]
fn insert10(b: &mut Bencher) {

    b.iter(|| {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        for i in 0..10 {
            map.insert(
                Long(concat!("One", stringify!(i))),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#[bench]
fn insert100(b: &mut Bencher) {
    b.iter(|| {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        for i in 0..100 {
            map.insert(
                Long(concat!("One", stringify!(i))),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#[bench]
fn insert1000(b: &mut Bencher) {
    b.iter(|| {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        for i in 0..1000 {
            map.insert(
                Long(concat!("One", stringify!(i))),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#[bench]
fn insert100000(b: &mut Bencher) {
    b.iter(|| {
        let mut map: MKeyMap<&str> = MKeyMap::new();
        for i in 0..100000 {
            map.insert(
                Long(concat!("One", stringify!(i))),
                concat!("Value", stringify!(i)),
            );
        }
    });
}

#![feature(test)]

extern crate clap;
extern crate mkey_map;
extern crate test;

use clap::Arg;
use mkey_map::KeyType::*;
use mkey_map::MKeyMap;

use std::ffi::OsStr;

use test::Bencher;

#[bench]
fn insert1(b: &mut Bencher) {
    b.iter(|| {
        let mut map = MKeyMap::new();
        map.insert(Long(OsStr::new("One")), Arg::with_name("Value"))
    });
}

#[bench]
fn insert10(b: &mut Bencher) {
    b.iter(|| {
        let mut map = MKeyMap::new();
        for i in 0..10 {
            map.insert(
                Long(&OsStr::new(concat!("One", stringify!(i)))),
                Arg::with_name(concat!("Value", stringify!(i))),
            );
        }
    });
}

#[bench]
fn insert100(b: &mut Bencher) {
    b.iter(|| {
        let mut map = MKeyMap::new();
        for i in 0..100 {
            map.insert(
                Long(&OsStr::new(concat!("One", stringify!(i)))),
                Arg::with_name(concat!("Value", stringify!(i))),
            );
        }
    });
}

#[bench]
fn insert1000(b: &mut Bencher) {
    b.iter(|| {
        let mut map = MKeyMap::new();
        for i in 0..1000 {
            map.insert(
                Long(&OsStr::new(concat!("One", stringify!(i)))),
                Arg::with_name(concat!("Value", stringify!(i))),
            );
        }
    });
}

#[bench]
fn insert100000(b: &mut Bencher) {
    b.iter(|| {
        let mut map = MKeyMap::new();
        for i in 0..100000 {
            map.insert(
                Long(&OsStr::new(concat!("One", stringify!(i)))),
                Arg::with_name(concat!("Value", stringify!(i))),
            );
        }
    });
}

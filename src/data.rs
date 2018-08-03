#![allow(dead_code)]

use csv::{self, StringRecord};

use std::fs::File;
use std::error::Error;
use std::iter::FromIterator;
use std::slice::Iter;

use packet::ItemDetailPacket;

pub struct CanData {
    records: Vec<Vec<Option<ItemDetailPacket>>>,
}

fn deserialize(record: StringRecord) -> Vec<Option<ItemDetailPacket>> {
    let mut iter = record.iter();

    let mut v = Vec::new();

    for _ in 0..10 {
        let sr = StringRecord::from_iter(iter.by_ref().take(5));
        match sr.deserialize::<ItemDetailPacket>(None) {
            Ok(x) => v.push(Some(x)),
            _ => v.push(None),
        }
    }

    v
}

impl CanData {
    pub fn new(csv_path: String) -> Result<Self, Box<Error>> {
        let file = File::open(csv_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        let mut rds = Vec::new();

        for result in reader.records() {
            let record = result?;
            rds.push(deserialize(record));
        }

        Ok(CanData{records: rds})
    }

    pub fn iter(&self) -> CanDataIterator {
        CanDataIterator{iter: self.records.iter()}
    }
}

pub struct CanDataIterator<'a> {
    iter: Iter<'a, Vec<Option<ItemDetailPacket>>>,
}

impl<'a> Iterator for CanDataIterator<'a> {
    type Item = &'a Vec<Option<ItemDetailPacket>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod test {
    use data::*;
    use csv::StringRecord;

    #[test]
    fn read_test() {
        let csv_data: StringRecord = StringRecord::from(",,,,,-20.5156,14.3125,1.125,-0.1875,0,,,,,,-32.625,12.8125,0.9375,0.1875,0,,,,,,,,,,,-22.4688,14.0469,1.375,-0.375,300,,,,,,-9.98438,8.07812,-0.5625,0.3125,54000,,,,,".split(',').collect::<Vec<&str>>());

        let v = deserialize(csv_data);

        assert_eq!(v[0], None);
        assert_eq!(v[1], Some(ItemDetailPacket::new(-20.5156, 14.3125, 1.125, -0.1875, 0)));
        assert_eq!(v[9], None);
    }
}


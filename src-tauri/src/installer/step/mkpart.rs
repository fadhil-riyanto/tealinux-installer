// this is very simple library, the goal is partitioning sdb

use crate::installer::blueprint::Partition;
use duct::cmd;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DiskInfo {
//     pub number: u32,
//     #[serde(rename(deserialize = "diskPath"))]
//     pub disk_path: String,
//     pub path: String,
//     pub mountpoint: Option<String>,
//     pub filesystem: String,
//     pub format: bool,
//     pub start: u64,
//     pub end: u64,
//     pub size: u64,
//     pub autogenerated: bool,
// }

pub struct Partgen;

impl Partgen {
    // pub fn readjson() -> Vec<DiskInfo> {
    //     let file = File::open("disk.json");

    //     let mut file_val = file.unwrap();

    //     let mut contents = String::new();
    //     file_val.read_to_string(&mut contents);

    //     let v: Vec<DiskInfo>  = serde_json::from_str(&contents).unwrap();
    //     v
    // }

    pub fn set_blkdev_partition_table(blk: String, parttype: String) -> () {
        let _ = cmd!("parted", blk, "--script", "mklabel", parttype).run();
    }

    pub fn mkpart(blktarget: String, start: u64, end: u64, fs: String, name: String) {
        let startf = format!("{}s", start);
        let endf = format!("{}s", end);

        println!(
            "Executing parted command: parted {} --script mkpart {} {} {} {}",
            blktarget, name, fs, startf, endf
        );

        let _ = cmd!("parted", blktarget, "--script", "mkpart", name, fs, startf, endf).run();
    }

    pub fn setflags(blktarget: String, subblk: u32) -> () {
        let _ = cmd!(
            "parted",
            blktarget,
            "--script",
            "set",
            subblk.to_string(),
            "boot",
            "on"
        )
        .run();
    }

    pub fn do_dangerous_task_on(blk_orig: &Option<String>, partition_style: Vec<Partition>) {
        if let Some(blk) = blk_orig {
            Self::set_blkdev_partition_table(blk.clone(), "gpt".to_string());

            // do partitioning
            let mut start_blk_idx = 1;
            for partition in partition_style {
                // runn
                let blk_name = format!("{}{}", blk, start_blk_idx);
                Self::mkpart(
                    blk.clone(),
                    partition.start,
                    partition.end,
                    partition.filesystem.unwrap(),
                    blk_name,
                );

                if start_blk_idx == 1 {
                    // set flags
                    Self::setflags(blk.clone(), start_blk_idx);
                }

                start_blk_idx = start_blk_idx + 1;
            }
        }
        
    }
}

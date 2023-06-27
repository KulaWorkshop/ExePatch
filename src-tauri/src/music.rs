use crate::types::GameType;

use std::{io::{Seek, SeekFrom}, fs::File};
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};

pub fn get_tracks(game: GameType, file: &mut File) -> Vec<u16> {
    let offset: u64 = match game {
        GameType::SCES_01000 => 0x62C02,
        GameType::SLUS_00724 => 0x63D2E,
        GameType::SCPS_10064 => 0x666DE,
        GameType::UNKNOWN => 0,
    };

    let mut tracks: Vec<u16> = Vec::<u16>::new();

    file.seek(SeekFrom::Start(offset)).unwrap();
    for _ in 0..16 {
        let track: u16 = file.read_u16::<LittleEndian>().expect("Failed to read track from executable file.");
        tracks.push(track);

        file.seek(SeekFrom::Current(6)).unwrap();
    }

    return tracks;
}

pub fn set_tracks(game: GameType, file: &mut File, tracks: &Vec<u16>) -> bool {
    let offset: u64 = match game {
        GameType::SCES_01000 => 0x62C02,
        GameType::SLUS_00724 => 0x63D2E,
        GameType::SCPS_10064 => 0x666DE,
        GameType::UNKNOWN => 0,
    };

    file.seek(SeekFrom::Start(offset)).unwrap();

    for x in 0..16 {
        file.write_u16::<LittleEndian>(tracks[x]).expect("Failed to write track to executable file.");
        file.seek(SeekFrom::Current(6)).unwrap();
    }

    return true;
}
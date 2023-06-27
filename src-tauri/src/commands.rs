use crate::music;
use crate::types::GameType;

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Cursor};

pub fn check<T: Read + Seek>(cursor: &mut T, offset: u64, id: &str) -> bool {
    let mut buffer = [0u8; 4];

    cursor.seek(SeekFrom::Start(offset)).unwrap();
    cursor.read_exact(&mut buffer).map_err(|err| err.to_string()).unwrap();
    if String::from_utf8_lossy(&buffer) == id {
        return true;
    }

    return false;
}

#[tauri::command]
pub fn open(path: String) -> Result<GameType, String> {
    let mut file = File::open(path).map_err(|err| err.to_string()).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Failed to read executable file.");

    let mut cursor = Cursor::new(buffer);

    // Check Kula Quest
    if check(&mut cursor, 0x8FDE4, "BISC") {
        return Ok(GameType::SCPS_10064.into());
    }

    // Check Kula World
    if check(&mut cursor, 0x927B4, "BESC") {
        return Ok(GameType::SCES_01000.into());
    }

    // Check Roll Away
    if check(&mut cursor, 0x93D0C, "BASL") {
        return Ok(GameType::SLUS_00724.into());
    }

    // Unknown
    Err("Unknown executable loaded.".into())
}

#[tauri::command]
pub fn get_tracks(game: GameType, path: String) -> Result<Vec<u16>, String> {
    let mut file = File::open(path).map_err(|err| err.to_string()).unwrap();
    let tracks: Vec<u16> = music::get_tracks(game, &mut file);
    Ok(tracks.into())
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_tracks(game: GameType, path: String, tracks: Vec<u16>) -> Result<(), String> {
    let result = OpenOptions::new().read(true).write(true).open(path);
    match result {
        Ok(mut file) => {
            music::set_tracks(game, &mut file, &tracks);
        }
        Err(err) => {
            return Err(err.to_string().into());
        }
    }

    Ok(())
}
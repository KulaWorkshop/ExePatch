#[allow(non_camel_case_types)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum GameType {
    SCES_01000,
    SLUS_00724,
    SCPS_10064,
    UNKNOWN,
}
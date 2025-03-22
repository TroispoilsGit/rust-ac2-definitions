use std::{collections::HashMap, io};

use crate::{
    data_id::DataId,
    property::property_collection::PropertyCollection,
    reader::binary_reader::BinaryReader,
    types::{cell_id::CellId, vector3::Vector3},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct CLightInfo {
    pub did: DataId,
    pub lookup_hash: HashMap<u32, Vec<u32>>,
    pub light_source_infos: Vec<LightSourceInfo>,
    pub sunlight_enabled: bool,
    pub sunlights: Vec<Vec<LightCellInfo>>,
}

impl CLightInfo {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let did = data
            .read_dataid()
            .expect("Erreur lors de la lecture de 'did'");
        let light_source_infos = data
            .read_list(|d| LightSourceInfo::new(d), 4)
            .expect("Erreur lors de la lecture de 'light_source_infos'");

        let lookup_hash = data
            .read_multi_dictionary(|d| d.read_u32(), |d| d.read_u32())
            .expect("Erreur lors de la lecture de 'lookup_hash'");

        let sunlight_enabled = data
            .read_bool()
            .expect("Erreur lors de la lecture de 'sunlight_enabled'");
        let mut sunlights = Vec::new();
        sunlights.reserve(9);

        if sunlight_enabled {
            for i in 0..9 {
                let light_cell_info = data
                    .read_list(|d| LightCellInfo::new(d), 4)
                    .expect("Erreur lors de la lecture des 'sunlights'");
                sunlights.insert(i, light_cell_info);
            }
        }

        Ok(Self {
            did,
            lookup_hash,
            light_source_infos,
            sunlight_enabled,
            sunlights,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightSourceInfo {
    pub cell_id: CellId,
    pub properties: PropertyCollection,
    pub origin: Vector3,
    pub light_cell_infos: Vec<LightCellInfo>,
}

impl LightSourceInfo {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let cell_id = data
            .read_cellid()
            .expect("Erreur lors de la lecture de 'cell_id'");
        let properties =
            PropertyCollection::new(data).expect("Erreur lors de la lecture des 'properties'");
        let origin = data
            .read_vector3()
            .expect("Erreur lors de la lecture du 'origin'");
        let light_cell_infos = data
            .read_list(|d| LightCellInfo::new(d), 4)
            .expect("Erreur lors de la lecture de 'light_cell_infos'");

        Ok(Self {
            cell_id,
            properties,
            origin,
            light_cell_infos,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightCellInfo {
    pub cell_id: CellId,
    pub indices: Vec<u16>,
}

impl LightCellInfo {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let cell_id = data
            .read_cellid()
            .expect("Erreur lors de la lecture de 'cell_id' dans LightCellInfo");
        let total_indices = data
            .read_u32()
            .expect("Erreur lors de la lecture de 'total_indices'");
        let mut indices = Vec::new();

        if total_indices > 0 {
            let count_ushort = data.read_u32()? / 2;
            for i in 0..count_ushort {
                indices.insert(i as usize, data.read_u16()?);
            }
            data.align(4);
        }

        Ok(Self { cell_id, indices })
    }
}

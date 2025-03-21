use std::{collections::HashMap, sync::OnceLock};

use crate::types::data_id::DataId;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum DatType {
    PORTAL,
    CELL,
    LOCAL,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum DbType {
    UNDEFINED = 0, // DB_TYPE_UNDEFINED

    CELLMESH = 0x40000004, // DB_TYPE_CELLMESH
    SCENE = 0x40000005,    // DB_TYPE_SCENE
    ANIMMAP = 0x40000006,  // DB_TYPE_ANIMMAP

    SETUP = 0x40000008,          // DB_TYPE_SETUP
    DBANIMATOR = 0x40000009,     // DB_TYPE_DBANIMATOR
    MESH = 0x4000000A,           // DB_TYPE_MESH
    REGION = 0x4000000B,         // DB_TYPE_REGION
    SOUND_DESC = 0x4000000C,     // DB_TYPE_SOUND_DESC
    SCENE_DESC = 0x4000000D,     // DB_TYPE_SCENE_DESC
    TERRAIN_DESC = 0x4000000E,   // DB_TYPE_TERRAIN_DESC
    SURFACE_DESC = 0x4000000F,   // DB_TYPE_SURFACE_DESC
    ENCOUNTER_DESC = 0x40000010, // DB_TYPE_ENCOUNTER_DESC
    SKY_DESC = 0x40000011,       // DB_TYPE_SKY_DESC
    PROPERTY_DESC = 0x40000012,  // DB_TYPE_PROPERTY_DESC
    WATER_DESC = 0x40000013,     // DB_TYPE_WATER_DESC
    FOG_DESC = 0x40000014,       // DB_TYPE_FOG_DESC
    BLOCK_MAP = 0x40000015,      // DB_TYPE_BLOCK_MAP
    DAY_DESC = 0x40000016,       // DB_TYPE_DAY_DESC

    WAVE = 0x40000018,      // DB_TYPE_WAVE
    QUALITIES = 0x40000019, // DB_TYPE_QUALITIES

    QUALITY_FILTER = 0x4000001D, // DB_TYPE_QUALITY_FILTER
    STRING = 0x4000001E,         // DB_TYPE_STRING
    CDB_TABLE = 0x4000001F,      // DB_TYPE_CDB_TABLE

    VISUAL_DESC = 0x40000021, // DB_TYPE_VISUAL_DESC
    APPEARANCE = 0x40000022,  // DB_TYPE_APPEARANCE
    UI_SCRIPT = 0x40000023,   // DB_TYPE_UI_SCRIPT

    UI_LAYOUT = 0x40000025,        // DB_TYPE_UI_LAYOUT
    STRING_TABLE = 0x40000026,     // DB_TYPE_STRING_TABLE
    FONT_TABLE = 0x40000027,       // DB_TYPE_FONT_TABLE
    ENUM_MAPPER = 0x40000028,      // DB_TYPE_ENUM_MAPPER
    MUSIC = 0x40000029,            // DB_TYPE_MUSIC
    DID_MAPPER = 0x4000002A,       // DB_TYPE_DID_MAPPER
    ENVINFO = 0x4000002B,          // DB_TYPE_ENVINFO
    SOUNDINFO = 0x4000002C,        // DB_TYPE_SOUNDINFO
    MUSICINFO = 0x4000002D,        // DB_TYPE_MUSICINFO
    RENDERMATERIAL = 0x4000002E,   // DB_TYPE_RENDERMATERIAL
    WSTATE = 0x4000002F,           // DB_TYPE_WSTATE
    CHARTEMPLATE = 0x40000030,     // DB_TYPE_CHARTEMPLATE
    MATERIALMODIFIER = 0x40000031, // DB_TYPE_MATERIALMODIFIER
    MATERIALINSTANCE = 0x40000032, // DB_TYPE_MATERIALINSTANCE
    PHYSICS_MATERIAL = 0x40000033, // DB_TYPE_PHYSICS_MATERIAL
    MOTIONINTERPDESC = 0x40000034, // DB_TYPE_MOTIONINTERPDESC
    PATHMAP = 0x40000035,          // DB_TYPE_PATHMAP
    FILEFORMAT = 0x40000036,       // DB_TYPE_FILEFORMAT
    LBO = 0x40000037,              // DB_TYPE_LBO
    SHELL = 0x40000038,            // DB_TYPE_SHELL
    VALIDMODES = 0x40000039,       // DB_TYPE_VALIDMODES
    TOOLCONFIG = 0x4000003A,       // DB_TYPE_TOOLCONFIG
    INPUTMAPPER = 0x4000003B,      // DB_TYPE_INPUTMAPPER
    RENDERSURFACE = 0x4000003C,    // DB_TYPE_RENDERSURFACE
    RENDERTEXTURE = 0x4000003D,    // DB_TYPE_RENDERTEXTURE
    FONT = 0x4000003E,             // DB_TYPE_FONT
    UI_SCENE = 0x4000003F,         // DB_TYPE_UI_SCENE
    BEHAVIORTABLE = 0x40000040,    // DB_TYPE_BEHAVIORTABLE

    MASTER_PROPERTY = 0x40000042,     // DB_TYPE_MASTER_PROPERTY
    GAME_TIME = 0x40000043,           // DB_TYPE_GAME_TIME
    LANDSCAPE_DEFS = 0x40000044,      // DB_TYPE_LANDSCAPE_DEFS
    MUSICDESC = 0x40000045,           // DB_TYPE_MUSICDESC
    FILE2ID_TABLE = 0x40000046,       // DB_TYPE_FILE2ID_TABLE
    PSDESC = 0x40000047,              // DB_TYPE_PSDESC
    ENTITYGROUP = 0x40000048,         // DB_TYPE_ENTITYGROUP
    ENTITYDESC = 0x40000049,          // DB_TYPE_ENTITYDESC
    KEYMAP = 0x4000004A,              // DB_TYPE_KEYMAP
    ACTIONMAP = 0x4000004B,           // DB_TYPE_ACTIONMAP
    FX_TABLE = 0x4000004C,            // DB_TYPE_FX_TABLE
    TABOO_TABLE = 0x4000004D,         // DB_TYPE_TABOO_TABLE
    CAMERA_FX = 0x4000004E,           // DB_TYPE_CAMERA_FX
    WLIB = 0x4000004F,                // DB_TYPE_WLIB
    LANDBLOCKDATA = 0x40000050,       // DB_TYPE_LANDBLOCKDATA
    LANDBLOCKINFO = 0x40000051,       // DB_TYPE_LANDBLOCKINFO
    ENVCELL = 0x40000052,             // DB_TYPE_ENVCELL
    DATFILEDATA = 0x40000053,         // DB_TYPE_DATFILEDATA
    PLACES_TABLE = 0x40000054,        // DB_TYPE_PLACES_TABLE
    STRING_STATE = 0x40000055,        // DB_TYPE_STRING_STATE
    FXSCRIPT = 0x40000056,            // DB_TYPE_FXSCRIPT
    CONVERSATION_TREE = 0x40000057,   // DB_TYPE_CONVERSATION_TREE
    LIGHTCACHE = 0x40000058,          // DB_TYPE_LIGHTCACHE
    LIGHTINFO = 0x40000059,           // DB_TYPE_LIGHTINFO
    DETAILMAP_DESC = 0x4000005A,      // DB_TYPE_DETAILMAP_DESC
    OBSTACLE_DESC = 0x4000005B,       // DB_TYPE_OBSTACLE_DESC
    ENCODED_WAV = 0x4000005C,         // DB_TYPE_ENCODED_WAV
    DBCATEGORIES = 0x4000005D,        // DB_TYPE_DBCATEGORIES
    MAPNOTE_DESC = 0x4000005E,        // DB_TYPE_MAPNOTE_DESC
    PERFORMANCE = 0x4000005F,         // DB_TYPE_PERFORMANCE
    FONT_LOCAL = 0x40000060,          // DB_TYPE_FONT_LOCAL
    RENDERSURFACE_LOCAL = 0x40000061, // DB_TYPE_RENDERSURFACE_LOCAL
    RENDERTEXTURE_LOCAL = 0x40000062, // DB_TYPE_RENDERTEXTURE_LOCAL
}

pub struct FileDef {
    pub start_did: u32,
    pub end_did: u32,
    pub dat_type: DatType,
}

impl FileDef {
    pub fn new(start_did: u32, end_did: u32, dat_type: DatType) -> Self {
        FileDef {
            start_did,
            end_did,
            dat_type,
        }
    }
}

pub struct TypeDefFile;

impl TypeDefFile {
    pub fn file_type() -> &'static HashMap<DbType, FileDef> {
        static FILE_TYPE: OnceLock<HashMap<DbType, FileDef>> = OnceLock::new();
        FILE_TYPE.get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(
                DbType::LANDBLOCKINFO,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::LIGHTINFO,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::LBO,
                FileDef::new(0x00000000, 0x00000000, DatType::PORTAL),
            );
            map.insert(
                DbType::SHELL,
                FileDef::new(0x00000000, 0x00000000, DatType::PORTAL),
            );
            map.insert(
                DbType::LIGHTCACHE,
                FileDef::new(0x00000000, 0x00000000, DatType::PORTAL),
            );
            map.insert(
                DbType::PATHMAP,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::CELLMESH,
                FileDef::new(0x01000000, 0x01FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SCENE,
                FileDef::new(0x02000000, 0x02FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ANIMMAP,
                FileDef::new(0x03000000, 0x03FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SETUP,
                FileDef::new(0x04000000, 0x04FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::DBANIMATOR,
                FileDef::new(0x05000000, 0x05FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MESH,
                FileDef::new(0x06000000, 0x06FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::QUALITY_FILTER,
                FileDef::new(0x0A000000, 0x0AFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::WAVE,
                FileDef::new(0x0C000000, 0x0CFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FILE2ID_TABLE,
                FileDef::new(0x0E000003, 0x0E000003, DatType::PORTAL),
            );
            map.insert(
                DbType::PLACES_TABLE,
                FileDef::new(0x0E000004, 0x0E000004, DatType::PORTAL),
            );
            map.insert(
                DbType::TABOO_TABLE,
                FileDef::new(0x0EBADA55, 0x0EBADA55, DatType::PORTAL),
            );
            map.insert(
                DbType::REGION,
                FileDef::new(0x0F000000, 0x0FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SOUND_DESC,
                FileDef::new(0x10000000, 0x10FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SCENE_DESC,
                FileDef::new(0x11000000, 0x11FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::TERRAIN_DESC,
                FileDef::new(0x12000000, 0x12FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SURFACE_DESC,
                FileDef::new(0x13000000, 0x13FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ENCOUNTER_DESC,
                FileDef::new(0x14000000, 0x14FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SKY_DESC,
                FileDef::new(0x15000000, 0x15FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::WATER_DESC,
                FileDef::new(0x16000000, 0x16FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FOG_DESC,
                FileDef::new(0x17000000, 0x17FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::PROPERTY_DESC,
                FileDef::new(0x18000000, 0x18FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::BLOCK_MAP,
                FileDef::new(0x19000000, 0x19FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::DAY_DESC,
                FileDef::new(0x1A000000, 0x1AFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::KEYMAP,
                FileDef::new(0x1D000000, 0x1DFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FX_TABLE,
                FileDef::new(0x1E000000, 0x1EFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::VISUAL_DESC,
                FileDef::new(0x1F000000, 0x1FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::APPEARANCE,
                FileDef::new(0x20000000, 0x20FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::UI_SCENE,
                FileDef::new(0x21000000, 0x21FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::UI_LAYOUT,
                FileDef::new(0x22000000, 0x22FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::ENUM_MAPPER,
                FileDef::new(0x23000000, 0x23FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MUSICDESC,
                FileDef::new(0x24000000, 0x24FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::STRING_TABLE,
                FileDef::new(0x25000000, 0x26FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::FILEFORMAT,
                FileDef::new(0x27000000, 0x27FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::INPUTMAPPER,
                FileDef::new(0x28000000, 0x28FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ENVINFO,
                FileDef::new(0x29000000, 0x29FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SOUNDINFO,
                FileDef::new(0x2A000000, 0x2AFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::RENDERMATERIAL,
                FileDef::new(0x2B000000, 0x2BFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MUSICINFO,
                FileDef::new(0x2C000000, 0x2CFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::CHARTEMPLATE,
                FileDef::new(0x2F000000, 0x2FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MATERIALMODIFIER,
                FileDef::new(0x30000000, 0x30FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MATERIALINSTANCE,
                FileDef::new(0x31000000, 0x31FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MOTIONINTERPDESC,
                FileDef::new(0x32000000, 0x32FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MASTER_PROPERTY,
                FileDef::new(0x34000000, 0x34FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::GAME_TIME,
                FileDef::new(0x35000000, 0x35FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::LANDSCAPE_DEFS,
                FileDef::new(0x36000000, 0x36FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::PHYSICS_MATERIAL,
                FileDef::new(0x37000000, 0x37FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::OBSTACLE_DESC,
                FileDef::new(0x38000000, 0x38FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::PSDESC,
                FileDef::new(0x39000000, 0x39FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::RENDERTEXTURE,
                FileDef::new(0x40000000, 0x400FFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::RENDERTEXTURE_LOCAL,
                FileDef::new(0x40100000, 0x40FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::RENDERSURFACE,
                FileDef::new(0x41000000, 0x410FFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::RENDERSURFACE_LOCAL,
                FileDef::new(0x41100000, 0x41FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::FONT,
                FileDef::new(0x42000000, 0x42000FFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FONT_LOCAL,
                FileDef::new(0x42001000, 0x42FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::BEHAVIORTABLE,
                FileDef::new(0x44000000, 0x44FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ENTITYGROUP,
                FileDef::new(0x46000000, 0x46FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ENTITYDESC,
                FileDef::new(0x47000000, 0x47FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ACTIONMAP,
                FileDef::new(0x51000000, 0x51FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::CDB_TABLE,
                FileDef::new(0x52000000, 0x527FFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::PERFORMANCE,
                FileDef::new(0x52800000, 0x52FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::VALIDMODES,
                FileDef::new(0x54000000, 0x54FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::CAMERA_FX,
                FileDef::new(0x55000000, 0x55FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::WLIB,
                FileDef::new(0x56000000, 0x56FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FXSCRIPT,
                FileDef::new(0x57000000, 0x57FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::STRING_STATE,
                FileDef::new(0x58000000, 0x58FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::DETAILMAP_DESC,
                FileDef::new(0x59000000, 0x59000FFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MAPNOTE_DESC,
                FileDef::new(0x5900F000, 0x59FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::CONVERSATION_TREE,
                FileDef::new(0x60000000, 0x60FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ENCODED_WAV,
                FileDef::new(0x61000000, 0x61FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::DBCATEGORIES,
                FileDef::new(0x62000000, 0x62FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::WSTATE,
                FileDef::new(0x6F000000, 0x7FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::QUALITIES,
                FileDef::new(0x80000000, 0x81FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::DATFILEDATA,
                FileDef::new(0xFFFF0000, 0xFFFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::LANDBLOCKDATA,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::ENVCELL,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map
        })
    }

    pub fn new() -> Self {
        TypeDefFile
    }

    pub fn get_db_type_from_data_id(dat_type: DatType, did: &DataId) -> DbType {
        Self::get_db_type_from_u32(dat_type, &did.id()) // Passage direct sans variable intermédiaire
    }

    pub fn get_db_type_from_u32(dat_type: DatType, did: &u32) -> DbType {
        let file_type = Self::file_type();
        let mut db_type = DbType::UNDEFINED;

        for (db_type_key, file_def) in file_type.iter() {
            if file_def.start_did <= *did && *did <= file_def.end_did {
                db_type = *db_type_key;
            }
        }

        if db_type != DbType::DATFILEDATA {
            if dat_type == DatType::CELL {
                let end = did & 0xFFFF;
                db_type = match end {
                    0xFFFC => DbType::LIGHTINFO,
                    0xFFFD => DbType::PATHMAP,
                    0xFFFE => DbType::LANDBLOCKINFO,
                    0xFFFF => DbType::LANDBLOCKDATA,
                    _ => DbType::ENVCELL,
                };
            }
        }
        db_type
    }
}

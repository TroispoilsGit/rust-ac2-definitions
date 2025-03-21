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
    Undefined = 0, // DB_TYPE_UNDEFINED

    Cellmesh = 0x40000004, // DB_TYPE_CELLMESH
    Scene = 0x40000005,    // DB_TYPE_SCENE
    Animmap = 0x40000006,  // DB_TYPE_ANIMMAP

    Setup = 0x40000008,         // DB_TYPE_SETUP
    Dbanimator = 0x40000009,    // DB_TYPE_DBANIMATOR
    Mesh = 0x4000000A,          // DB_TYPE_MESH
    Region = 0x4000000B,        // DB_TYPE_REGION
    SoundDesc = 0x4000000C,     // DB_TYPE_SOUND_DESC
    SceneDesc = 0x4000000D,     // DB_TYPE_SCENE_DESC
    TerrainDesc = 0x4000000E,   // DB_TYPE_TERRAIN_DESC
    SurfaceDesc = 0x4000000F,   // DB_TYPE_SURFACE_DESC
    EncounterDesc = 0x40000010, // DB_TYPE_ENCOUNTER_DESC
    SkyDesc = 0x40000011,       // DB_TYPE_SKY_DESC
    PropertyDesc = 0x40000012,  // DB_TYPE_PROPERTY_DESC
    WaterDesc = 0x40000013,     // DB_TYPE_WATER_DESC
    FogDesc = 0x40000014,       // DB_TYPE_FOG_DESC
    BlockMap = 0x40000015,      // DB_TYPE_BLOCK_MAP
    DayDesc = 0x40000016,       // DB_TYPE_DAY_DESC

    Wave = 0x40000018,      // DB_TYPE_WAVE
    Qualities = 0x40000019, // DB_TYPE_QUALITIES

    QualityFilter = 0x4000001D, // DB_TYPE_QUALITY_FILTER
    String = 0x4000001E,        // DB_TYPE_STRING
    CdbTable = 0x4000001F,      // DB_TYPE_CDB_TABLE

    VisualDesc = 0x40000021, // DB_TYPE_VISUAL_DESC
    Appearance = 0x40000022, // DB_TYPE_APPEARANCE
    UiScript = 0x40000023,   // DB_TYPE_UI_SCRIPT

    UiLayout = 0x40000025,         // DB_TYPE_UI_LAYOUT
    StringTable = 0x40000026,      // DB_TYPE_STRING_TABLE
    FontTable = 0x40000027,        // DB_TYPE_FONT_TABLE
    EnumMapper = 0x40000028,       // DB_TYPE_ENUM_MAPPER
    Music = 0x40000029,            // DB_TYPE_MUSIC
    DidMapper = 0x4000002A,        // DB_TYPE_DID_MAPPER
    Envinfo = 0x4000002B,          // DB_TYPE_ENVINFO
    Soundinfo = 0x4000002C,        // DB_TYPE_SOUNDINFO
    Musicinfo = 0x4000002D,        // DB_TYPE_MUSICINFO
    Rendermaterial = 0x4000002E,   // DB_TYPE_RENDERMATERIAL
    Wstate = 0x4000002F,           // DB_TYPE_WSTATE
    Chartemplate = 0x40000030,     // DB_TYPE_CHARTEMPLATE
    Materialmodifier = 0x40000031, // DB_TYPE_MATERIALMODIFIER
    Materialinstance = 0x40000032, // DB_TYPE_MATERIALINSTANCE
    PhysicsMaterial = 0x40000033,  // DB_TYPE_PHYSICS_MATERIAL
    Motioninterpdesc = 0x40000034, // DB_TYPE_MOTIONINTERPDESC
    Pathmap = 0x40000035,          // DB_TYPE_PATHMAP
    Fileformat = 0x40000036,       // DB_TYPE_FILEFORMAT
    Lbo = 0x40000037,              // DB_TYPE_LBO
    Shell = 0x40000038,            // DB_TYPE_SHELL
    Validmodes = 0x40000039,       // DB_TYPE_VALIDMODES
    Toolconfig = 0x4000003A,       // DB_TYPE_TOOLCONFIG
    Inputmapper = 0x4000003B,      // DB_TYPE_INPUTMAPPER
    Rendersurface = 0x4000003C,    // DB_TYPE_RENDERSURFACE
    Rendertexture = 0x4000003D,    // DB_TYPE_RENDERTEXTURE
    Font = 0x4000003E,             // DB_TYPE_FONT
    UiScene = 0x4000003F,          // DB_TYPE_UI_SCENE
    Behaviortable = 0x40000040,    // DB_TYPE_BEHAVIORTABLE

    MasterProperty = 0x40000042,     // DB_TYPE_MASTER_PROPERTY
    GameTime = 0x40000043,           // DB_TYPE_GAME_TIME
    LandscapeDefs = 0x40000044,      // DB_TYPE_LANDSCAPE_DEFS
    Musicdesc = 0x40000045,          // DB_TYPE_MUSICDESC
    File2idTable = 0x40000046,       // DB_TYPE_FILE2ID_TABLE
    Psdesc = 0x40000047,             // DB_TYPE_PSDESC
    Entitygroup = 0x40000048,        // DB_TYPE_ENTITYGROUP
    Entitydesc = 0x40000049,         // DB_TYPE_ENTITYDESC
    Keymap = 0x4000004A,             // DB_TYPE_KEYMAP
    Actionmap = 0x4000004B,          // DB_TYPE_ACTIONMAP
    FxTable = 0x4000004C,            // DB_TYPE_FX_TABLE
    TabooTable = 0x4000004D,         // DB_TYPE_TABOO_TABLE
    CameraFx = 0x4000004E,           // DB_TYPE_CAMERA_FX
    Wlib = 0x4000004F,               // DB_TYPE_WLIB
    Landblockdata = 0x40000050,      // DB_TYPE_LANDBLOCKDATA
    Landblockinfo = 0x40000051,      // DB_TYPE_LANDBLOCKINFO
    Envcell = 0x40000052,            // DB_TYPE_ENVCELL
    Datfiledata = 0x40000053,        // DB_TYPE_DATFILEDATA
    PlacesTable = 0x40000054,        // DB_TYPE_PLACES_TABLE
    StringState = 0x40000055,        // DB_TYPE_STRING_STATE
    Fxscript = 0x40000056,           // DB_TYPE_FXSCRIPT
    ConversationTree = 0x40000057,   // DB_TYPE_CONVERSATION_TREE
    Lightcache = 0x40000058,         // DB_TYPE_LIGHTCACHE
    Lightinfo = 0x40000059,          // DB_TYPE_LIGHTINFO
    DetailmapDesc = 0x4000005A,      // DB_TYPE_DETAILMAP_DESC
    ObstacleDesc = 0x4000005B,       // DB_TYPE_OBSTACLE_DESC
    EncodedWav = 0x4000005C,         // DB_TYPE_ENCODED_WAV
    Dbcategories = 0x4000005D,       // DB_TYPE_DBCATEGORIES
    MapnoteDesc = 0x4000005E,        // DB_TYPE_MAPNOTE_DESC
    Performance = 0x4000005F,        // DB_TYPE_PERFORMANCE
    FontLocal = 0x40000060,          // DB_TYPE_FONT_LOCAL
    RendersurfaceLocal = 0x40000061, // DB_TYPE_RENDERSURFACE_LOCAL
    RendertextureLocal = 0x40000062, // DB_TYPE_RENDERTEXTURE_LOCAL
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
            map.insert(DbType::Landblockdata, FileDef::new(0, 0, DatType::CELL));
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
        let mut db_type = DbType::Undefined;

        for (db_type_key, file_def) in file_type.iter() {
            if file_def.start_did <= *did && *did <= file_def.end_did {
                db_type = *db_type_key;
            }
        }

        if db_type != DbType::Datfiledata {
            if dat_type == DatType::CELL {
                let end = did & 0xFFFF;
                db_type = match end {
                    0xFFFC => DbType::Lightinfo,
                    0xFFFD => DbType::Pathmap,
                    0xFFFE => DbType::Landblockinfo,
                    0xFFFF => DbType::Landblockdata,
                    _ => DbType::Envcell,
                };
            }
        }
        db_type
    }
}

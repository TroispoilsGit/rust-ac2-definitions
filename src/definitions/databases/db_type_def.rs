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
    DbAnimator = 0x40000009,    // DB_TYPE_DBANIMATOR
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
    BehaviorTable = 0x40000040,    // DB_TYPE_BEHAVIORTABLE

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
            map.insert(
                DbType::Landblockinfo,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::Lightinfo,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::Lbo,
                FileDef::new(0x00000000, 0x00000000, DatType::PORTAL),
            );
            map.insert(
                DbType::Shell,
                FileDef::new(0x00000000, 0x00000000, DatType::PORTAL),
            );
            map.insert(
                DbType::Lightcache,
                FileDef::new(0x00000000, 0x00000000, DatType::PORTAL),
            );
            map.insert(
                DbType::Pathmap,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::Cellmesh,
                FileDef::new(0x01000000, 0x01FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Scene,
                FileDef::new(0x02000000, 0x02FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Animmap,
                FileDef::new(0x03000000, 0x03FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Setup,
                FileDef::new(0x04000000, 0x04FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::DbAnimator,
                FileDef::new(0x05000000, 0x05FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Mesh,
                FileDef::new(0x06000000, 0x06FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::QualityFilter,
                FileDef::new(0x0A000000, 0x0AFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Wave,
                FileDef::new(0x0C000000, 0x0CFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::File2idTable,
                FileDef::new(0x0E000003, 0x0E000003, DatType::PORTAL),
            );
            map.insert(
                DbType::PlacesTable,
                FileDef::new(0x0E000004, 0x0E000004, DatType::PORTAL),
            );
            map.insert(
                DbType::TabooTable,
                FileDef::new(0x0EBADA55, 0x0EBADA55, DatType::PORTAL),
            );
            map.insert(
                DbType::Region,
                FileDef::new(0x0F000000, 0x0FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SoundDesc,
                FileDef::new(0x10000000, 0x10FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SceneDesc,
                FileDef::new(0x11000000, 0x11FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::TerrainDesc,
                FileDef::new(0x12000000, 0x12FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SurfaceDesc,
                FileDef::new(0x13000000, 0x13FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::EncounterDesc,
                FileDef::new(0x14000000, 0x14FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::SkyDesc,
                FileDef::new(0x15000000, 0x15FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::WaterDesc,
                FileDef::new(0x16000000, 0x16FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FogDesc,
                FileDef::new(0x17000000, 0x17FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::PropertyDesc,
                FileDef::new(0x18000000, 0x18FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::BlockMap,
                FileDef::new(0x19000000, 0x19FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::DayDesc,
                FileDef::new(0x1A000000, 0x1AFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Keymap,
                FileDef::new(0x1D000000, 0x1DFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FxTable,
                FileDef::new(0x1E000000, 0x1EFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::VisualDesc,
                FileDef::new(0x1F000000, 0x1FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Appearance,
                FileDef::new(0x20000000, 0x20FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::UiScene,
                FileDef::new(0x21000000, 0x21FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::UiLayout,
                FileDef::new(0x22000000, 0x22FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::EnumMapper,
                FileDef::new(0x23000000, 0x23FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Musicdesc,
                FileDef::new(0x24000000, 0x24FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::StringTable,
                FileDef::new(0x25000000, 0x26FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::Fileformat,
                FileDef::new(0x27000000, 0x27FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Inputmapper,
                FileDef::new(0x28000000, 0x28FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Envinfo,
                FileDef::new(0x29000000, 0x29FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Soundinfo,
                FileDef::new(0x2A000000, 0x2AFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Rendermaterial,
                FileDef::new(0x2B000000, 0x2BFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Musicinfo,
                FileDef::new(0x2C000000, 0x2CFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Chartemplate,
                FileDef::new(0x2F000000, 0x2FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Materialmodifier,
                FileDef::new(0x30000000, 0x30FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Materialinstance,
                FileDef::new(0x31000000, 0x31FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Motioninterpdesc,
                FileDef::new(0x32000000, 0x32FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MasterProperty,
                FileDef::new(0x34000000, 0x34FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::GameTime,
                FileDef::new(0x35000000, 0x35FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::LandscapeDefs,
                FileDef::new(0x36000000, 0x36FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::PhysicsMaterial,
                FileDef::new(0x37000000, 0x37FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ObstacleDesc,
                FileDef::new(0x38000000, 0x38FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Psdesc,
                FileDef::new(0x39000000, 0x39FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Rendertexture,
                FileDef::new(0x40000000, 0x400FFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::RendertextureLocal,
                FileDef::new(0x40100000, 0x40FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::Rendersurface,
                FileDef::new(0x41000000, 0x410FFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::RendersurfaceLocal,
                FileDef::new(0x41100000, 0x41FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::Font,
                FileDef::new(0x42000000, 0x42000FFF, DatType::PORTAL),
            );
            map.insert(
                DbType::FontLocal,
                FileDef::new(0x42001000, 0x42FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::BehaviorTable,
                FileDef::new(0x44000000, 0x44FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Entitygroup,
                FileDef::new(0x46000000, 0x46FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Entitydesc,
                FileDef::new(0x47000000, 0x47FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Actionmap,
                FileDef::new(0x51000000, 0x51FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::CdbTable,
                FileDef::new(0x52000000, 0x527FFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Performance,
                FileDef::new(0x52800000, 0x52FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Validmodes,
                FileDef::new(0x54000000, 0x54FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::CameraFx,
                FileDef::new(0x55000000, 0x55FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Wlib,
                FileDef::new(0x56000000, 0x56FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Fxscript,
                FileDef::new(0x57000000, 0x57FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::StringState,
                FileDef::new(0x58000000, 0x58FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::DetailmapDesc,
                FileDef::new(0x59000000, 0x59000FFF, DatType::PORTAL),
            );
            map.insert(
                DbType::MapnoteDesc,
                FileDef::new(0x5900F000, 0x59FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::ConversationTree,
                FileDef::new(0x60000000, 0x60FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::EncodedWav,
                FileDef::new(0x61000000, 0x61FFFFFF, DatType::LOCAL),
            );
            map.insert(
                DbType::Dbcategories,
                FileDef::new(0x62000000, 0x62FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Wstate,
                FileDef::new(0x6F000000, 0x7FFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Qualities,
                FileDef::new(0x80000000, 0x81FFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Datfiledata,
                FileDef::new(0xFFFF0000, 0xFFFFFFFF, DatType::PORTAL),
            );
            map.insert(
                DbType::Landblockdata,
                FileDef::new(0x00000000, 0x00000000, DatType::CELL),
            );
            map.insert(
                DbType::Envcell,
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

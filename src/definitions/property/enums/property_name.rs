use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone, FromPrimitive, strum_macros::Display,
)]
pub enum PropertyName {
    Undef = 0,

    PhysicsMaterial = 0x40000001, // PhysicsMaterial
    LightType = 0x40000002,       // LightType
    LightDirection = 0x40000003,  // LightDirection
    LightRange = 0x40000004,      // LightRange

    LightDiffuse = 0x40000008,          // LightDiffuse
    LightDiffuseIntensity = 0x40000009, // LightDiffuseIntensity
    LightSpecular = 0x4000000A,         // LightSpecular

    LandscapeShadowCast = 0x40000014, // LandscapeShadowCast
    VolumeShadowCast = 0x40000015,    // VolumeShadowCast

    IndoorShadowCast = 0x40000017, // IndoorShadowCast

    EtherealEnvironment = 0x4000001B, // EtherealEnvironment
    EtherealWater = 0x4000001C,       // EtherealWater

    CameraOrthographic = 0x4000001E,  // CameraOrthographic
    CameraFOV = 0x4000001F,           // CameraFOV
    CameraVerticalScale = 0x40000020, // CameraVerticalScale
    CameraNearDistance = 0x40000021,  // CameraNearDistance
    CameraFarDistance = 0x40000022,   // CameraFarDistance
    EtherealTest = 0x40000023,        // EtherealTest
    DefaultEthereal = 0x40000024,     // DefaultEthereal

    CameraEthereal = 0x40000026, // CameraEthereal

    ClearanceBBoxMin1 = 0x40000028,         // ClearanceBBoxMin1
    ClearanceBBoxMin2 = 0x40000029,         // ClearanceBBoxMin2
    ClearanceBBoxMin3 = 0x4000002A,         // ClearanceBBoxMin3
    ClearanceBBoxMin4 = 0x4000002B,         // ClearanceBBoxMin4
    ClearanceBBoxMax1 = 0x4000002C,         // ClearanceBBoxMax1
    ClearanceBBoxMax2 = 0x4000002D,         // ClearanceBBoxMax2
    ClearanceBBoxMax3 = 0x4000002E,         // ClearanceBBoxMax3
    ClearanceBBoxMax4 = 0x4000002F,         // ClearanceBBoxMax4
    ClearanceCylRadius1 = 0x40000030,       // ClearanceCylRadius1
    ClearanceCylRadius2 = 0x40000031,       // ClearanceCylRadius2
    ClearanceCylCenter1 = 0x40000032,       // ClearanceCylCenter1
    ClearanceCylCenter2 = 0x40000033,       // ClearanceCylCenter2
    ClearanceCylHeight1 = 0x40000034,       // ClearanceCylHeight1
    ClearanceCylHeight2 = 0x40000035,       // ClearanceCylHeight2
    TerrainType = 0x40000036,               // TerrainType
    DetailMap = 0x40000037,                 // DetailMap
    StaticEthereal = 0x40000038,            // StaticEthereal
    ContentID = 0x40000039,                 // ContentID
    ContentDescription = 0x4000003A,        // ContentDescription
    ContentType = 0x4000003B,               // ContentType
    ContentName = 0x4000003C,               // ContentName
    BlockID = 0x4000003D,                   // BlockID
    BlockX = 0x4000003E,                    // BlockX
    BlockY = 0x4000003F,                    // BlockY
    ControlledVelocity = 0x40000040,        // ControlledVelocity
    ControlledAcceleration = 0x40000041,    // ControlledAcceleration
    UnControlledGeneralVel = 0x40000042,    // UnControlledGeneralVel
    UnControlledGeneralAccel = 0x40000043,  // UnControlledGeneralAccel
    UnControlledEarthVel = 0x40000044,      // UnControlledEarthVel
    UnControlledEarthAccel = 0x40000045,    // UnControlledEarthAccel
    UnControlledAirVel = 0x40000046,        // UnControlledAirVel
    UnControlledAirAccel = 0x40000047,      // UnControlledAirAccel
    UnControlledSeafloorVel = 0x40000048,   // UnControlledSeafloorVel
    UnControlledSeafloorAccel = 0x40000049, // UnControlledSeafloorAccel
    UnControlledWaterVel = 0x4000004A,      // UnControlledWaterVel
    UnControlledWaterAccel = 0x4000004B,    // UnControlledWaterAccel
    LocalSpace = 0x4000004C,                // LocalSpace
    InvisibleIngame = 0x4000004D,           // InvisibleIngame

    LandblockType = 0x40000051, // LandblockType

    InteriorFogType = 0x40000056,      // InteriorFogType
    InteriorFogColor = 0x40000057,     // InteriorFogColor
    InteriorFogNear = 0x40000058,      // InteriorFogNear
    InteriorFogFar = 0x40000059,       // InteriorFogFar
    InteriorAmbientLight = 0x4000005A, // InteriorAmbientLight
    LightShadowType = 0x4000005B,      // LightShadowType
    Translucent = 0x4000005C,          // Translucent
    AppearanceKey = 0x4000005D,        // AppearanceKey
    AppearanceScalar = 0x4000005E,     // AppearanceScalar
    CellPhysicsMaterial = 0x4000005F,  // CellPhysicsMaterial
    CellAmbientSoundType = 0x40000060, // CellAmbientSoundType
    CoronaTexture = 0x40000061,        // CoronaTexture
    CoronaSize = 0x40000062,           // CoronaSize
    CoronaOpacity = 0x40000063,        // CoronaOpacity
    CoronaFadeSpeed = 0x40000064,      // CoronaFadeSpeed
    PortalDestination = 0x40000065,    // PortalDestination

    WaterHeight = 0x40000067,     // WaterHeight
    WaterType = 0x40000068,       // WaterType
    DistanceFogType = 0x40000069, // DistanceFogType

    UseSharedRep = 0x4000006B, // UseSharedRep
    ForceOutside = 0x4000006C, // ForceOutside

    EncounterBlock = 0x4000006E,  // Encounter-Block
    MissileEthereal = 0x4000006F, // MissileEthereal

    MaximumDegradeLevel = 0x40000071, // MaximumDegradeLevel

    PhysicsVelScale = 0x40000073,        // PhysicsVelScale
    PhysicsAccScale = 0x40000074,        // PhysicsAccScale
    PhysicsCombatAnimScale = 0x40000075, // PhysicsCombatAnimScale
    PhysicsJumpScale = 0x40000076,       // PhysicsJumpScale
    NeverHousekeep = 0x40000077,         // NeverHousekeep
    PlacesDatType = 0x40000078,          // PlacesDatType
    LinkColor = 0x40000079,              // LinkColor
    LinkName = 0x4000007A,               // LinkName

    UsageMinLevel = 0x41000001, // UsageMinLevel
    UsageMaxLevel = 0x41000002, // UsageMaxLevel

    OpenDoor = 0x41000008, // OpenDoor

    SkillParent = 0x41000016, // SkillParent
    SkillPrereq = 0x41000017, // SkillPrereq

    SkillZeroVigorModifier = 0x4100001B, // SkillZeroVigorModifier

    SkillNumHooks = 0x4100001D,                // SkillNumHooks
    GeneratorProfile = 0x4100001E,             // GeneratorProfile
    GeneratorRegenPeriod = 0x4100001F,         // GeneratorRegenPeriod
    GeneratorInitialDelay = 0x41000020,        // GeneratorInitialDelay
    GeneratorMinQuantityOverride = 0x41000021, // GeneratorMinQuantityOverride
    GeneratorMaxQuantityOverride = 0x41000022, // GeneratorMaxQuantityOverride

    CreatureEthereal = 0x4100002C, // CreatureEthereal
    PlayerEthereal = 0x4100002D,   // PlayerEthereal
    NPCEthereal = 0x4100002E,      // NPCEthereal
    AdminEthereal = 0x4100002F,    // AdminEthereal

    GeneratorNonPersonalizable = 0x41000032, // GeneratorNonPersonalizable
    GeneratorRegenAllIfUnbound = 0x41000033, // GeneratorRegenAllIfUnbound
    GeneratorEnterWorldPreserve = 0x41000034, // GeneratorEnterWorldPreserve
    GeneratorMunge = 0x41000035,             // GeneratorMunge
    GeneratorDontMunge = 0x41000036,         // GeneratorDontMunge
    GeneratorAbsoluteQuantity = 0x41000037,  // GeneratorAbsoluteQuantity
    GeneratorCheckpoint = 0x41000038,        // GeneratorCheckpoint
    GeneratorDontCheckpoint = 0x41000039,    // GeneratorDontCheckpoint

    SkillPassive = 0x4100003C,             // SkillPassive
    SkillMinVigor = 0x4100003D,            // SkillMinVigor
    SkillUsePrereq = 0x4100003E,           // SkillUsePrereq
    SkillRequiredWieldedItem = 0x4100003F, // SkillRequiredWieldedItem

    SkillDescription = 0x41000042, // SkillDescription

    SkillMinCharacterLevel = 0x41000044, // SkillMinCharacterLevel
    SkillAllowedSpecies = 0x41000045,    // SkillAllowedSpecies
    SkillAllowedFactions = 0x41000046,   // SkillAllowedFactions
    SkillRequiredWormItem = 0x41000047,  // SkillRequiredWormItem
    SkillRequiredEffect = 0x41000048,    // SkillRequiredEffect
    SkillUseWhileMoving = 0x41000049,    // SkillUseWhileMoving
    SkillResetTime = 0x4100004A,         // SkillResetTime
    SkillPowerupTime = 0x4100004B,       // SkillPowerupTime
    SkillIsHidden = 0x4100004C,          // SkillIsHidden
    SkillRecoveryTime = 0x4100004D,      // SkillRecoveryTime
    SkillShieldModifier = 0x4100004E,    // SkillShieldModifier
    SkillBarring = 0x4100004F,           // SkillBarring
    SkillCreditCost = 0x41000050,        // SkillCreditCost
    SkillIsUntrainable = 0x41000051,     // SkillIsUntrainable
    SkillAdvancementTable = 0x41000052,  // SkillAdvancementTable
    SkillAdvancementModifier = 0x41000053, // SkillAdvancementModifier
    SkillAdvancementCap = 0x41000054,    // SkillAdvancementCap
    SkillLevelWhenTrained = 0x41000055,  // SkillLevelWhenTrained
    SkillMultVigorModifier = 0x41000056, // SkillMultVigorModifier
    SkillRequiresMoveTo = 0x41000057,    // SkillRequiresMoveTo
    SkillRequiresTurnTo = 0x41000058,    // SkillRequiresTurnTo
    SkillMinHealth = 0x41000059,         // SkillMinHealth
    SkillMaxHealth = 0x4100005A,         // SkillMaxHealth
    SkillTargetEffect = 0x4100005B,      // SkillTargetEffect
    SkillMaxVigor = 0x4100005C,          // SkillMaxVigor
    SkillMinRange = 0x4100005D,          // SkillMinRange
    SkillMaxRange = 0x4100005E,          // SkillMaxRange
    SkillUserEffect = 0x4100005F,        // SkillUserEffect

    SkillBarringWieldedItem = 0x41000061, // SkillBarringWieldedItem
    NeutralFactionEthereal = 0x41000062,  // NeutralFactionEthereal
    SkillBarringWornItem = 0x41000063,    // SkillBarringWornItem
    Faction1Ethereal = 0x41000064,        // Faction1Ethereal
    SkillBarringEffect = 0x41000065,      // SkillBarringEffect
    SkillAllowedImplementsForRightHand = 0x41000066, // SkillAllowedImplementsForRightHand
    SkillAllowedImplementsForLeftHand = 0x41000067, // SkillAllowedImplementsForLeftHand
    Faction2Ethereal = 0x41000068,        // Faction2Ethereal
    Faction3Ethereal = 0x41000069,        // Faction3Ethereal
    ItemEnglishName = 0x4100006A,         // ItemEnglishName
    SkillEnglishName = 0x4100006B,        // SkillEnglishName
    SkillMinUseTime = 0x4100006C,         // SkillMinUseTime
    SkillAttackTimeModifier = 0x4100006D, // SkillAttackTimeModifier
    SkillArmorModifier = 0x4100006E,      // SkillArmorModifier
    SkillHookIsPrimaryImplement = 0x4100006F, // SkillHookIsPrimaryImplement
    SkillAddVigorModifier = 0x41000070,   // SkillAddVigorModifier
    SkillHookIsSecondaryImplement = 0x41000071, // SkillHookIsSecondaryImplement
    SkillAbilityLevelModifier = 0x41000072, // SkillAbilityLevelModifier
    SkillIgnoreWeaponTime = 0x41000073,   // SkillIgnoreWeaponTime
    SkillIgnoreArmorDelay = 0x41000074,   // SkillIgnoreArmorDelay
    SkillIgnoreWeaponOffense = 0x41000075, // SkillIgnoreWeaponOffense
    SkillIgnoreWeaponDefense = 0x41000076, // SkillIgnoreWeaponDefense
    SkillIgnoreWeaponVigor = 0x41000077,  // SkillIgnoreWeaponVigor
    SkillIgnoreWeaponDamage = 0x41000078, // SkillIgnoreWeaponDamage
    SkillIgnoreShieldVigor = 0x41000079,  // SkillIgnoreShieldVigor
    SkillIgnoreShieldDelay = 0x4100007A,  // SkillIgnoreShieldDelay
    SkillIsNonDamaging = 0x4100007B,      // SkillIsNonDamaging
    SkillBasicFightingWeights = 0x4100007C, // SkillBasicFightingWeights
    SkillIsAutoAttack = 0x4100007D,       // SkillIsAutoAttack
    SkillHookIsShieldAllowed = 0x4100007E, // SkillHookIsShieldAllowed
    SkillHookIsIntersection = 0x4100007F, // SkillHookIsIntersection
    SkillHookIsDistance = 0x41000080,     // SkillHookIsDistance
    SkillHookIsProjectile = 0x41000081,   // SkillHookIsProjectile
    SkillIsHarmful = 0x41000082,          // SkillIsHarmful
    SkillCriticalThreshold = 0x41000083,  // SkillCriticalThreshold
    SkillCriticalModifier = 0x41000084,   // SkillCriticalModifier

    ItemDescription = 0x41000086,                 // ItemDescription
    ItemIsWieldable = 0x41000087,                 // ItemIsWieldable
    ItemIsStackable = 0x41000088,                 // ItemIsStackable
    ItemMaxStackSize = 0x41000089,                // ItemMaxStackSize
    ItemPreferredInventoryLocation = 0x4100008A,  // ItemPreferredInventoryLocation
    ItemValidInventoryLocations = 0x4100008B,     // ItemValidInventoryLocations
    ItemPrecludedInventoryLocations = 0x4100008C, // ItemPrecludedInventoryLocations
    ItemPrimaryParentingLocation = 0x4100008D,    // ItemPrimaryParentingLocation
    ItemSecondaryParentingLocation = 0x4100008E,  // ItemSecondaryParentingLocation
    ItemImplementType = 0x4100008F,               // ItemImplementType
    ItemMinLevel = 0x41000090,                    // ItemMinLevel
    ItemMaxLevel = 0x41000091,                    // ItemMaxLevel
    ItemFactionRequired = 0x41000092,             // ItemFactionRequired
    ItemRequiredSkill1 = 0x41000093,              // ItemRequiredSkill1
    ItemRequiredSkill2 = 0x41000094,              // ItemRequiredSkill2
    ItemRestrictedSkill1 = 0x41000095,            // ItemRestrictedSkill1
    ItemRestrictedSkill2 = 0x41000096,            // ItemRestrictedSkill2
    ItemRequiredRace = 0x41000097,                // ItemRequiredRace
    ItemRequiredQuest = 0x41000098,               // ItemRequiredQuest
    ItemRequiredQuestStatus = 0x41000099,         // ItemRequiredQuestStatus

    ItemValue = 0x4100009C,           // ItemValue
    ItemIsDestroyOnSell = 0x4100009D, // ItemIsDestroyOnSell

    ItemEquipperEffects = 0x410000A4,      // ItemEquipperEffects
    ItemUsageEffects = 0x410000A5,         // ItemUsageEffects
    ItemTargetEffects = 0x410000A6,        // ItemTargetEffects
    ItemArmorLevel = 0x410000A7,           // ItemArmorLevel
    ItemCombatDelay = 0x410000A8,          // ItemCombatDelay
    ItemVigorCost = 0x410000A9,            // ItemVigorCost
    ItemDamage = 0x410000AA,               // ItemDamage
    ItemVariance = 0x410000AB,             // ItemVariance
    ItemWeaponTime = 0x410000AC,           // ItemWeaponTime
    ItemIsHarmless = 0x410000AD,           // ItemIsHarmless
    ItemOffenseModifier = 0x410000AE,      // ItemOffenseModifier
    SkillHookAttackAreaRange = 0x410000AF, // SkillHookAttackAreaRange
    SkillHookIsAreaOfEffect = 0x410000B0,  // SkillHookIsAreaOfEffect
    SkillHookRequiresLOS = 0x410000B1,     // SkillHookRequiresLOS
    SkillHookDetonatesOnMiss = 0x410000B2, // SkillHookDetonatesOnMiss
    SkillHookAddDamage = 0x410000B3,       // SkillHookAddDamage
    SkillHookMultDamage = 0x410000B4,      // SkillHookMultDamage
    ItemIsOpen = 0x410000B5,               // ItemIsOpen
    ItemIsLocked = 0x410000B6,             // ItemIsLocked
    ItemMinUseDist = 0x410000B7,           // ItemMinUseDist

    ItemCanUseWhileMoving = 0x410000B9, // ItemCanUseWhileMoving
    ItemCanUseOnlyInInventory = 0x410000BA, // ItemCanUseOnlyInInventory
    ItemCanUseWhenCollided = 0x410000BB, // ItemCanUseWhenCollided
    ItemIsLockOnUse = 0x410000BC,       // ItemIsLockOnUse
    ItemIsTargetedUsageItem = 0x410000BD, // ItemIsTargetedUsageItem
    ItemMoveToTarget = 0x410000BE,      // ItemMoveToTarget
    ItemValidTargetTypes = 0x410000BF,  // ItemValidTargetTypes
    ItemHealthCost = 0x410000C0,        // ItemHealthCost
    ItemTimeBetweenUses = 0x410000C1,   // ItemTimeBetweenUses
    ItemLockable = 0x410000C2,          // ItemLockable
    ItemLockOnClose = 0x410000C3,       // ItemLockOnClose
    ItemOpenOnUnlock = 0x410000C4,      // ItemOpenOnUnlock
    ItemIsAttunable = 0x410000C5,       // ItemIsAttunable
    ItemKeyID = 0x410000C6,             // ItemKeyID
    ItemIsUsable = 0x410000C7,          // ItemIsUsable
    ItemIsSelectable = 0x410000C8,      // ItemIsSelectable
    ItemIsTakeable = 0x410000C9,        // ItemIsTakeable
    ItemIsDestroyOnUse = 0x410000CA,    // ItemIsDestroyOnUse

    ItemMapDestination = 0x410000CC, // ItemMapDestination
    ItemLocDestination = 0x410000CD, // ItemLocDestination
    NPCEnglishName = 0x410000CE,     // NPCEnglishName
    NPCDescription = 0x410000CF,     // NPCDescription

    NPCAISuperClass = 0x410000D1,              // NPCAISuperClass
    NPCAISubClass = 0x410000D2,                // NPCAISubClass
    NPCDoesAgentDestroyOnUse = 0x410000D3,     // NPCDoesAgentDestroyOnUse
    NPCAICanUseDoors = 0x410000D4,             // NPCAICanUseDoors
    NPCAreAllItemsDestroyedOnRot = 0x410000D5, // NPCAreAllItemsDestroyedOnRot
    NPCIsLootProof = 0x410000D6,               // NPCIsLootProof
    NPCLootTimer = 0x410000D7,                 // NPCLootTimer
    NPCBaseRotTime = 0x410000D8,               // NPCBaseRotTime
    NPCMaxAcceleratedRotTime = 0x410000D9,     // NPCMaxAcceleratedRotTime
    NPCMinAcceleratedRotTime = 0x410000DA,     // NPCMinAcceleratedRotTime
    NPCLevel = 0x410000DB,                     // NPCLevel
    NPCDeathExperience = 0x410000DC,           // NPCDeathExperience
    NPCIsAcceptingItems = 0x410000DD,          // NPCIsAcceptingItems
    NPCArmorLevel = 0x410000DE,                // NPCArmorLevel

    NPCSkills = 0x410000E0, // NPCSkills
    NPCHealth = 0x410000E1, // NPCHealth
    NPCVigor = 0x410000E2,  // NPCVigor

    NPCInventory = 0x410000E5,                        // NPCInventory
    NPCHomesickRadius = 0x410000E6,                   // NPCHomesickRadius
    NPCPerceptionRadius = 0x410000E7,                 // NPCPerceptionRadius
    NPCIsWandering = 0x410000E8,                      // NPCIsWandering
    NPCMaxWanderRange = 0x410000E9,                   // NPCMaxWanderRange
    NPCFactionMembership = 0x410000EA,                // NPCFactionMembership
    NPCIsFactionBasedOnLandblock = 0x410000EB,        // NPCIsFactionBasedOnLandblock
    NPCIsFactionOwnershipChangeOnDeath = 0x410000EC,  // NPCIsFactionOwnershipChangeOnDeath
    NPCCanWieldImplements = 0x410000ED,               // NPCCanWieldImplements
    NPCCanUseWeaponAndShield = 0x410000EE,            // NPCCanUseWeaponAndShield
    NPCCanDualWield = 0x410000EF,                     // NPCCanDualWield
    NPCNaturalDamage = 0x410000F0,                    // NPCNaturalDamage
    NPCNaturalVariance = 0x410000F1,                  // NPCNaturalVariance
    NPCNaturalVigorCost = 0x410000F2,                 // NPCNaturalVigorCost
    NPCNaturalWeaponTime = 0x410000F3,                // NPCNaturalWeaponTime
    NPCNaturalOffenseModifier = 0x410000F4,           // NPCNaturalOffenseModifier
    ItemFullDestination = 0x410000F5,                 // ItemFullDestination
    NPCChallengeLevel = 0x410000F6,                   // NPCChallengeLevel
    NPCCanWieldTwoHanded = 0x410000F7,                // NPCCanWieldTwoHanded
    EffectEnglishName = 0x410000F8,                   // EffectEnglishName
    EffectDescription = 0x410000F9,                   // EffectDescription
    EffectDuration = 0x410000FA,                      // EffectDuration
    EffectProbabilityOfApplication = 0x410000FB,      // EffectProbabilityOfApplication
    EffectTsysName = 0x410000FC,                      // EffectTsysName
    EffectEquivalenceClass = 0x410000FD,              // EffectEquivalenceClass
    EffectClassPriority = 0x410000FE,                 // EffectClassPriority
    EffectIsClientVisible = 0x410000FF,               // EffectIsClientVisible
    EffectIsHarmful = 0x41000100,                     // EffectIsHarmful
    EffectIsRemoveOnDeath = 0x41000101,               // EffectIsRemoveOnDeath
    EffectIsFactionEffect = 0x41000102,               // EffectIsFactionEffect
    EffectIsHeartbeat = 0x41000103,                   // EffectIsHeartbeat
    EffectIsChainBreaker = 0x41000104,                // EffectIsChainBreaker
    EffectIsVitalChangeInterested = 0x41000105,       // EffectIsVitalChangeInterested
    EffectIsEffectApplicationInterested = 0x41000106, // EffectIsEffectApplicationInterested
    EffectIsInstantaneous = 0x41000107,               // EffectIsInstantaneous
    EffectMinLevel = 0x41000108,                      // EffectMinLevel
    EffectMaxLevel = 0x41000109,                      // EffectMaxLevel
    EffectFactionRequired = 0x4100010A,               // EffectFactionRequired
    EffectRequiredSkill1 = 0x4100010B,                // EffectRequiredSkill1
    EffectRequiredSkill2 = 0x4100010C,                // EffectRequiredSkill2
    EffectRestrictedSkill1 = 0x4100010D,              // EffectRestrictedSkill1
    EffectRestrictedSkill2 = 0x4100010E,              // EffectRestrictedSkill2
    EffectRequiredRace = 0x4100010F,                  // EffectRequiredRace
    EffectTrait = 0x41000110,                         // EffectTrait
    EffectTraitAmount = 0x41000111,                   // EffectTraitAmount
    EffectUncommonTrait = 0x41000112,                 // EffectUncommonTrait
    EffectMinTsysSpellcraft = 0x41000113,             // EffectMinTsysSpellcraft
    EffectMaxTsysSpellcraft = 0x41000114,             // EffectMaxTsysSpellcraft
    EffectTsysValue = 0x41000115,                     // EffectTsysValue
    ItemMinimumRank = 0x41000116,                     // ItemMinimumRank
    ItemMaximumRank = 0x41000117,                     // ItemMaximumRank
    ItemNonAllegianceOnly = 0x41000118,               // ItemNonAllegianceOnly
    ItemMonarchOnly = 0x41000119,                     // ItemMonarchOnly
    CraftRecipeEnglishName = 0x4100011A,              // CraftRecipeEnglishName
    CraftRecipeDescription = 0x4100011B,              // CraftRecipeDescription

    CraftRecipeDifficulty = 0x4100011E,   // CraftRecipeDifficulty
    CraftRecipeMaxSuccesses = 0x4100011F, // CraftRecipeMaxSuccesses
    CraftRecipeCharges = 0x41000120,      // CraftRecipeCharges
    CraftRecipeRecoveryTime = 0x41000121, // CraftRecipeRecoveryTime
    CraftRecipeSkillBonus = 0x41000122,   // CraftRecipeSkillBonus
    CraftRecipeNumIngredients = 0x41000123, // CraftRecipeNumIngredients
    CraftRecipeIngredients = 0x41000124,  // CraftRecipeIngredients
    CraftRecipeMasteryActions = 0x41000125, // CraftRecipeMasteryActions
    CraftRecipeCraftCheckEntries = 0x41000126, // CraftRecipeCraftCheckEntries
    SkillAttackerSuccessString = 0x41000127, // SkillAttackerSuccessString
    SkillAttackerFailureString = 0x41000128, // SkillAttackerFailureString
    SkillAttackerCriticalString = 0x41000129, // SkillAttackerCriticalString
    SkillDefenderSuccessString = 0x4100012A, // SkillDefenderSuccessString
    SkillDefenderFailureString = 0x4100012B, // SkillDefenderFailureString
    SkillDefenderCriticalString = 0x4100012C, // SkillDefenderCriticalString
    NPCLoveTable = 0x4100012D,            // NPCLoveTable

    UsageRequiredFaction = 0x41000131,     // UsageRequiredFaction
    UsageRequiredRace = 0x41000132,        // UsageRequiredRace
    UsageRequiredQuest = 0x41000133,       // UsageRequiredQuest
    UsageRequiredQuestStatus = 0x41000134, // UsageRequiredQuestStatus
    UsageRequiredSkill1 = 0x41000135,      // UsageRequiredSkill1
    UsageRequiredSkill2 = 0x41000136,      // UsageRequiredSkill2
    UsageRestrictedSkill1 = 0x41000137,    // UsageRestrictedSkill1
    UsageRestrictedSkill2 = 0x41000138,    // UsageRestrictedSkill2
    UsageMinRank = 0x41000139,             // UsageMinRank
    UsageMaxRank = 0x4100013A,             // UsageMaxRank
    UsageNonAllegianceOnly = 0x4100013B,   // UsageNonAllegianceOnly
    UsageMonarchOnly = 0x4100013C,         // UsageMonarchOnly
    UsageLandblockFactionRequired = 0x4100013D, // UsageLandblockFactionRequired
    LockLockable = 0x4100013E,             // LockLockable
    LockOpenOnUnlock = 0x4100013F,         // LockOpenOnUnlock
    LockLockOnClose = 0x41000140,          // LockLockOnClose
    LockAICanIgnoreLock = 0x41000141,      // LockAICanIgnoreLock
    LockKeyID = 0x41000142,                // LockKeyID
    Name = 0x41000143,                     // Name
    Description = 0x41000144,              // Description
    GeneratorEnglishName = 0x41000145,     // GeneratorEnglishName
    QuestBestowedSceneID = 0x41000146,     // QuestBestowedSceneID
    UsageEffect1 = 0x41000147,             // UsageEffect1
    UsageEffect1Spellcraft = 0x41000148,   // UsageEffect1Spellcraft
    UsageEffect2 = 0x41000149,             // UsageEffect2
    UsageEffect2Spellcraft = 0x4100014A,   // UsageEffect2Spellcraft

    QuestContentDescription = 0x4100014C, // QuestContentDescription
    QuestExamineDescription = 0x4100014D, // QuestExamineDescription
    QuestFailOnLogout = 0x4100014E,       // QuestFailOnLogout
    QuestFailOnDeath = 0x4100014F,        // QuestFailOnDeath
    QuestFellowshipPropagate = 0x41000150, // QuestFellowshipPropagate
    QuestActive = 0x41000151,             // QuestActive
    QuestPhases = 0x41000152,             // QuestPhases
    QuestSolutions = 0x41000153,          // QuestSolutions
    QuestDuration = 0x41000154,           // QuestDuration
    QuestRetry = 0x41000155,              // QuestRetry
    QuestCompletionXP = 0x41000156,       // QuestCompletionXP
    QuestChallengeLevel = 0x41000157,     // QuestChallengeLevel
    QuestPoints = 0x41000158,             // QuestPoints
    QuestLevelMinimum = 0x41000159,       // QuestLevelMinimum
    QuestLevelMaximum = 0x4100015A,       // QuestLevelMaximum
    QuestFaction = 0x4100015B,            // QuestFaction
    QuestRequiredSkill1 = 0x4100015C,     // QuestRequiredSkill1
    QuestRequiredSkill2 = 0x4100015D,     // QuestRequiredSkill2
    QuestRestrictedSkill1 = 0x4100015E,   // QuestRestrictedSkill1
    QuestRestrictedSkill2 = 0x4100015F,   // QuestRestrictedSkill2
    QuestRequiredRace = 0x41000160,       // QuestRequiredRace
    QuestRequiredQuest = 0x41000161,      // QuestRequiredQuest
    QuestRequiredQuestStatus = 0x41000162, // QuestRequiredQuestStatus
    QuestRankMinimum = 0x41000163,        // QuestRankMinimum
    QuestRankMaximum = 0x41000164,        // QuestRankMaximum
    QuestMonarchOnly = 0x41000165,        // QuestMonarchOnly
    QuestNonAllegianceOnly = 0x41000166,  // QuestNonAllegianceOnly
    QuestRequiredLandblockFaction = 0x41000167, // QuestRequiredLandblockFaction
    UsageEffect3 = 0x41000168,            // UsageEffect3
    UsageEffect3Spellcraft = 0x41000169,  // UsageEffect3Spellcraft
    UsageEffect4 = 0x4100016A,            // UsageEffect4
    UsageEffect4Spellcraft = 0x4100016B,  // UsageEffect4Spellcraft
    UsageEffect5 = 0x4100016C,            // UsageEffect5
    UsageEffect5Spellcraft = 0x4100016D,  // UsageEffect5Spellcraft
    GameplayStatisticsMaxHealth = 0x4100016E, // GameplayStatisticsMaxHealth
    GameplayStatisticsHealthRegenRate = 0x4100016F, // GameplayStatisticsHealthRegenRate
    GameplayStatisticsMaxVigor = 0x41000170, // GameplayStatisticsMaxVigor
    GameplayStatisticsVigorRegenRate = 0x41000171, // GameplayStatisticsVigorRegenRate
    GameplayStatisticsDeathExperience = 0x41000172, // GameplayStatisticsDeathExperience
    GameplayStatisticsLevel = 0x41000173, // GameplayStatisticsLevel
    MineRecipe = 0x41000174,              // MineRecipe
    MineTrait = 0x41000175,               // MineTrait
    MineTraitAmount = 0x41000176,         // MineTraitAmount
    CraftMineDifficulty = 0x41000177,     // CraftMineDifficulty
    AIHomesickRadius = 0x41000178,        // AIHomesickRadius
    AICanJoinCliques = 0x41000179,        // AICanJoinCliques
    AIPerceptionRadius = 0x4100017A,      // AIPerceptionRadius
    AIMovementType = 0x4100017B,          // AIMovementType
    AIFactionMembership = 0x4100017C,     // AIFactionMembership
    AIFactionMembershipBasedOnLandblock = 0x4100017D, // AIFactionMembershipBasedOnLandblock
    AIFactionOwnershipChangeOnDeath = 0x4100017E, // AIFactionOwnershipChangeOnDeath
    AIFreeAttacking = 0x4100017F,         // AIFreeAttacking
    AICombatWieldImplements = 0x41000180, // AICombatWieldImplements
    AICombatUseWeaponAndShield = 0x41000181, // AICombatUseWeaponAndShield
    AICombatCanDualWield = 0x41000182,    // AICombatCanDualWield
    AICombatCanWieldTwoHanded = 0x41000183, // AICombatCanWieldTwoHanded
    AICombatNaturalDamage = 0x41000184,   // AICombatNaturalDamage
    AICombatNaturalVariance = 0x41000185, // AICombatNaturalVariance
    AICombatNaturalVigorCost = 0x41000186, // AICombatNaturalVigorCost
    AICombatNaturalWeaponTime = 0x41000187, // AICombatNaturalWeaponTime
    AICombatNaturalOffenseModifier = 0x41000188, // AICombatNaturalOffenseModifier
    AINPCTable = 0x41000189,              // AINPCTable
    AILoveTable = 0x4100018A,             // AILoveTable
    AIInventory1 = 0x4100018B,            // AIInventory1
    AIInventory2 = 0x4100018C,            // AIInventory2
    AIInventory3 = 0x4100018D,            // AIInventory3
    PortalFlags = 0x4100018E,             // PortalFlags
    MaximumUpkeepPoints = 0x4100018F,     // MaximumUpkeepPoints
    ResetInterval = 0x41000190,           // ResetInterval
    FuelType = 0x41000191,                // FuelType
    SpellcraftModifier = 0x41000192,      // SpellcraftModifier
    UsageRequiredSkill1Rating = 0x41000193, // UsageRequiredSkill1Rating
    UsageRequiredSkill2Rating = 0x41000194, // UsageRequiredSkill2Rating
    LockIsLocked = 0x41000195,            // LockIsLocked
    WeaponDamage = 0x41000196,            // WeaponDamage
    WeaponVariance = 0x41000197,          // WeaponVariance
    WeaponVigorCost = 0x41000198,         // WeaponVigorCost
    WeaponSpeed = 0x41000199,             // WeaponSpeed
    WeaponOffenseMod = 0x4100019A,        // WeaponOffenseMod
    WeaponSingleWeaponStance = 0x4100019B, // WeaponSingleWeaponStance
    WeaponWithShieldStance = 0x4100019C,  // WeaponWithShieldStance
    WeaponDualWieldStance = 0x4100019D,   // WeaponDualWieldStance
    WeaponHarmless = 0x4100019E,          // WeaponHarmless
    AIWandering = 0x4100019F,             // AIWandering
    AIWanderingRange = 0x410001A0,        // AIWanderingRange
    AIWanderingProb = 0x410001A1,         // AIWanderingProb
    AICanUseDoors = 0x410001A2,           // AICanUseDoors
    AIDetectionSpheres = 0x410001A3,      // AIDetectionSpheres
    AIAISubClass = 0x410001A4,            // AIAISubClass
    AIAISuperClass = 0x410001A5,          // AIAISuperClass
    NPCDeathLootProfile = 0x410001A6,     // NPCDeathLootProfile
    NPCDeathRemoveLootProfile = 0x410001A7, // NPCDeathRemoveLootProfile

    NPCDeathEffect1 = 0x410001A9,     // NPCDeathEffect1
    NPCDeathEffect1Stat = 0x410001AA, // NPCDeathEffect1Stat
    NPCDeathEffect2 = 0x410001AB,     // NPCDeathEffect2
    NPCDeathEffect2Stat = 0x410001AC, // NPCDeathEffect2Stat
    NPCDeathEffect3 = 0x410001AD,     // NPCDeathEffect3
    NPCDeathEffect3Stat = 0x410001AE, // NPCDeathEffect3Stat
    NPCGrooveLevel = 0x410001AF,      // NPCGrooveLevel

    NPCStartingSkill1 = 0x410001BA, // NPCStartingSkill1

    NPCStartingSkill2 = 0x410001BC, // NPCStartingSkill2

    NPCStartingSkill3 = 0x410001BE, // NPCStartingSkill3

    NPCStartingSkill4 = 0x410001C0, // NPCStartingSkill4

    NPCStartingSkill5 = 0x410001C2, // NPCStartingSkill5

    NPCTSysCoarseItemClass = 0x410001C4, // NPCTSysCoarseItemClass
    NPCTSysFineItemClass = 0x410001C5,   // NPCTSysFineItemClass

    NPCUnlockedAfterFirstLoot = 0x410001C8, // NPCUnlockedAfterFirstLoot

    NPCStartingSkill6 = 0x410001D3, // NPCStartingSkill6

    NPCStartingSkill7 = 0x410001D5, // NPCStartingSkill7

    NPCStartingSkill8 = 0x410001D7, // NPCStartingSkill8

    NPCStartingSkill9 = 0x410001D9, // NPCStartingSkill9

    NPCStartingSkill10 = 0x410001DB, // NPCStartingSkill10

    AIInventory1Quantity = 0x410001DD, // AIInventory1Quantity
    AIInventory2Quantity = 0x410001DE, // AIInventory2Quantity
    AIInventory3Quantity = 0x410001DF, // AIInventory3Quantity

    NPCStartingSkill11 = 0x410001E0, // NPCStartingSkill11

    NPCStartingSkill12 = 0x410001E2, // NPCStartingSkill12

    NPCStartingSkill13 = 0x410001E4, // NPCStartingSkill13

    NPCStartingSkill14 = 0x410001E6, // NPCStartingSkill14

    NPCStartingSkill15 = 0x410001E8, // NPCStartingSkill15

    NPCStartingSkill16 = 0x410001EA, // NPCStartingSkill16

    NPCStartingSkill17 = 0x410001EC, // NPCStartingSkill17

    NPCStartingSkill18 = 0x410001EE, // NPCStartingSkill18

    NPCStartingSkill19 = 0x410001F0, // NPCStartingSkill19

    NPCStartingSkill20 = 0x410001F2, // NPCStartingSkill20

    NPCStartingSkill1Level = 0x410001F4, // NPCStartingSkill1Level
    NPCStartingSkill2Level = 0x410001F5, // NPCStartingSkill2Level
    NPCStartingSkill3Level = 0x410001F6, // NPCStartingSkill3Level
    NPCStartingSkill4Level = 0x410001F7, // NPCStartingSkill4Level
    NPCStartingSkill5Level = 0x410001F8, // NPCStartingSkill5Level
    NPCStartingSkill6Level = 0x410001F9, // NPCStartingSkill6Level
    NPCStartingSkill7Level = 0x410001FA, // NPCStartingSkill7Level
    NPCStartingSkill8Level = 0x410001FB, // NPCStartingSkill8Level
    NPCStartingSkill9Level = 0x410001FC, // NPCStartingSkill9Level
    NPCStartingSkill10Level = 0x410001FD, // NPCStartingSkill10Level
    NPCStartingSkill11Level = 0x410001FE, // NPCStartingSkill11Level
    NPCStartingSkill12Level = 0x410001FF, // NPCStartingSkill12Level
    NPCStartingSkill13Level = 0x41000200, // NPCStartingSkill13Level
    NPCStartingSkill14Level = 0x41000201, // NPCStartingSkill14Level
    NPCStartingSkill15Level = 0x41000202, // NPCStartingSkill15Level
    NPCStartingSkill16Level = 0x41000203, // NPCStartingSkill16Level
    NPCStartingSkill17Level = 0x41000204, // NPCStartingSkill17Level
    NPCStartingSkill18Level = 0x41000205, // NPCStartingSkill18Level
    NPCStartingSkill19Level = 0x41000206, // NPCStartingSkill19Level
    NPCStartingSkill20Level = 0x41000207, // NPCStartingSkill20Level

    Icon = 0x4100020A,                            // Icon
    PhysObj = 0x4100020B,                         // PhysObj
    Placeable = 0x4100020C,                       // Placeable
    NPCIsGroupMonster = 0x4100020D,               // NPCIsGroupMonster
    ItemValidInventoryLocation1 = 0x4100020E,     // ItemValidInventoryLocation1
    ItemValidInventoryLocation2 = 0x4100020F,     // ItemValidInventoryLocation2
    ItemPrecludedInventoryLocation1 = 0x41000210, // ItemPrecludedInventoryLocation1
    ItemPrecludedInventoryLocation2 = 0x41000211, // ItemPrecludedInventoryLocation2
    ItemPrecludedInventoryLocation3 = 0x41000212, // ItemPrecludedInventoryLocation3
    ItemPrecludedInventoryLocation4 = 0x41000213, // ItemPrecludedInventoryLocation4
    ItemParentingOrientation = 0x41000214,        // ItemParentingOrientation
    ContainerMaxContainerSize = 0x41000215,       // ContainerMaxContainerSize
    ContainerContainedWaitOn = 0x41000216,        // ContainerContainedWaitOn
    ContainerManagedGenerator = 0x41000217,       // ContainerManagedGenerator
    ContainerMaxItems = 0x41000218,               // ContainerMaxItems
    Scale = 0x41000219,                           // Scale
    ItemAcquireEffect1 = 0x4100021A,              // ItemAcquireEffect1
    ItemAcquireEffect1Stat = 0x4100021B,          // ItemAcquireEffect1Stat
    ItemAcquireEffect2 = 0x4100021C,              // ItemAcquireEffect2
    ItemAcquireEffect2Stat = 0x4100021D,          // ItemAcquireEffect2Stat
    ItemAcquireEffect3 = 0x4100021E,              // ItemAcquireEffect3
    ItemAcquireEffect3Stat = 0x4100021F,          // ItemAcquireEffect3Stat
    ItemEquipperEffect1 = 0x41000220,             // ItemEquipperEffect1
    ItemEquipperEffect1Stat = 0x41000221,         // ItemEquipperEffect1Stat
    ItemEquipperEffect2 = 0x41000222,             // ItemEquipperEffect2
    ItemEquipperEffect2Stat = 0x41000223,         // ItemEquipperEffect2Stat
    ItemEquipperEffect3 = 0x41000224,             // ItemEquipperEffect3
    ItemEquipperEffect3Stat = 0x41000225,         // ItemEquipperEffect3Stat
    ItemTargetEffect1 = 0x41000226,               // ItemTargetEffect1
    ItemTargetEffect1Stat = 0x41000227,           // ItemTargetEffect1Stat
    ItemTargetEffect2 = 0x41000228,               // ItemTargetEffect2
    ItemTargetEffect2Stat = 0x41000229,           // ItemTargetEffect2Stat
    ItemTargetEffect3 = 0x4100022A,               // ItemTargetEffect3
    ItemTargetEffect3Stat = 0x4100022B,           // ItemTargetEffect3Stat
    ItemPluralName = 0x4100022C,                  // ItemPluralName

    ItemQuantity = 0x4100022F,               // ItemQuantity
    ItemTSysCoarseItemClass = 0x41000230,    // ItemTSysCoarseItemClass
    ItemTSysFineItemClass = 0x41000231,      // ItemTSysFineItemClass
    ItemGenInternal = 0x41000232,            // ItemGenInternal
    ItemGenIsAGenerator = 0x41000233,        // ItemGenIsAGenerator
    ItemUsageAction = 0x41000234,            // ItemUsageAction
    ItemUsageAnimation = 0x41000235,         // ItemUsageAnimation
    ItemUsageAIHint1 = 0x41000236,           // ItemUsageAIHint1
    ItemUsageAIHint2 = 0x41000237,           // ItemUsageAIHint2
    ItemMinPermissionCheckDist = 0x41000238, // ItemMinPermissionCheckDist
    ItemUsagePermission = 0x41000239,        // ItemUsagePermission

    ItemUsageTargetType = 0x4100023B, // ItemUsageTargetType

    Mobile = 0x41000242,                  // Mobile
    IgnoreCollisions = 0x41000243,        // IgnoreCollisions
    ReportCollisions = 0x41000244,        // ReportCollisions
    ItemWornAppearanceRace1 = 0x41000245, // ItemWornAppearanceRace1
    ItemWornAppearanceSex1 = 0x41000246,  // ItemWornAppearanceSex1
    ItemWornAppearanceFile1 = 0x41000247, // ItemWornAppearanceFile1
    ItemWornAppearanceRace2 = 0x41000248, // ItemWornAppearanceRace2
    ItemWornAppearanceSex2 = 0x41000249,  // ItemWornAppearanceSex2
    ItemWornAppearanceFile2 = 0x4100024A, // ItemWornAppearanceFile2

    ItemMaxInscriptionLength = 0x4100024C, // ItemMaxInscriptionLength
    ItemPileAppearance = 0x4100024D,       // ItemPileAppearance

    ItemTakePermission = 0x4100024F, // ItemTakePermission
    UsageAllowedRaces = 0x41000250,  // UsageAllowedRaces

    UsageRequiredQuestForTaking = 0x41000257, // UsageRequiredQuestForTaking
    UsageRequiredQuestStatusForTaking = 0x41000258, // UsageRequiredQuestStatusForTaking
    AIChampionMonster = 0x41000259,           // AIChampionMonster
    AIUniqueMonster = 0x4100025A,             // AIUniqueMonster
    AISpecialEffectMonster = 0x4100025B,      // AISpecialEffectMonster
    AIQuestMonster = 0x4100025C,              // AIQuestMonster
    NPCStartingSkill1IsBase = 0x4100025D,     // NPCStartingSkill1IsBase
    NPCStartingSkill2IsBase = 0x4100025E,     // NPCStartingSkill2IsBase
    NPCStartingSkill3IsBase = 0x4100025F,     // NPCStartingSkill3IsBase
    NPCStartingSkill4IsBase = 0x41000260,     // NPCStartingSkill4IsBase
    NPCStartingSkill5IsBase = 0x41000261,     // NPCStartingSkill5IsBase
    NPCStartingSkill6IsBase = 0x41000262,     // NPCStartingSkill6IsBase
    NPCStartingSkill7IsBase = 0x41000263,     // NPCStartingSkill7IsBase
    NPCStartingSkill8IsBase = 0x41000264,     // NPCStartingSkill8IsBase
    NPCStartingSkill9IsBase = 0x41000265,     // NPCStartingSkill9IsBase
    NPCStartingSkill10IsBase = 0x41000266,    // NPCStartingSkill10IsBase
    NPCStartingSkill11IsBase = 0x41000267,    // NPCStartingSkill11IsBase
    NPCStartingSkill12IsBase = 0x41000268,    // NPCStartingSkill12IsBase
    NPCStartingSkill13IsBase = 0x41000269,    // NPCStartingSkill13IsBase
    NPCStartingSkill14IsBase = 0x4100026A,    // NPCStartingSkill14IsBase
    NPCStartingSkill15IsBase = 0x4100026B,    // NPCStartingSkill15IsBase
    NPCStartingSkill16IsBase = 0x4100026C,    // NPCStartingSkill16IsBase
    NPCStartingSkill17IsBase = 0x4100026D,    // NPCStartingSkill17IsBase
    NPCStartingSkill18IsBase = 0x4100026E,    // NPCStartingSkill18IsBase
    NPCStartingSkill19IsBase = 0x4100026F,    // NPCStartingSkill19IsBase
    NPCStartingSkill20IsBase = 0x41000270,    // NPCStartingSkill20IsBase
    RegionChat = 0x41000271,                  // RegionChat
    TradeChat = 0x41000272,                   // TradeChat
    GeneratorQualityOverride = 0x41000273,    // GeneratorQualityOverride
    GeneratorQualityVarianceOverride = 0x41000274, // GeneratorQualityVarianceOverride
    NPCDeathLootQualityOverride = 0x41000275, // NPCDeathLootQualityOverride
    NPCDeathLootQualityVarianceOverride = 0x41000276, // NPCDeathLootQualityVarianceOverride

    AIUnwieldItemsOnIdle = 0x41000278, // AIUnwieldItemsOnIdle
    ItemIsQuestItem = 0x41000279,      // ItemIsQuestItem
    MonsterStuckEthereal = 0x4100027A, // MonsterStuckEthereal
    MoveItemDistance = 0x4100027B,     // MoveItemDistance
    UsageAnimation = 0x4100027C,       // UsageAnimation
    UsageUserAnimation = 0x4100027D,   // UsageUserAnimation
    UsageUserAnimationTimeScale = 0x4100027E, // UsageUserAnimationTimeScale
    UsageSuccessMessage = 0x4100027F,  // UsageSuccessMessage
    GeneratorExitWorldBehavior = 0x41000280, // GeneratorExitWorldBehavior
    AIIdleOnly = 0x41000281,           // AIIdleOnly
    ContainerIsAcceptingItems = 0x41000282, // ContainerIsAcceptingItems
    GeneratorToggleStateGameEvent = 0x41000283, // GeneratorToggleStateGameEvent

    GameplayStatisticsCombatHealthRegenRate = 0x41000285, // GameplayStatisticsCombatHealthRegenRate

    GameplayStatisticsCombatVigorRegenRate = 0x41000287, // GameplayStatisticsCombatVigorRegenRate
    ItemInscribePermission = 0x41000288,                 // ItemInscribePermission
    UsageCrafterOnly = 0x41000289,                       // UsageCrafterOnly
    NPCDeathEffect4 = 0x4100028A,                        // NPCDeathEffect4
    NPCDeathEffect4Stat = 0x4100028B,                    // NPCDeathEffect4Stat

    PKPermissionCannotBeHarmedBySameFaction = 0x4100028E, // PKPermissionCannotBeHarmedBySameFaction
    PKPermissionCannotHarmSameFaction = 0x4100028F,       // PKPermissionCannotHarmSameFaction
    EnterWorldFX = 0x41000290,                            // EnterWorldFX
    AINeverSayDie = 0x41000291,                           // AINeverSayDie
    RadarBlip = 0x41000292,                               // RadarBlip
    WeaponCriticalHitMod = 0x41000293,                    // WeaponCriticalHitMod
    PluralName = 0x41000294,                              // PluralName

    ItemIsRareItem = 0x41000296, // ItemIsRareItem

    NPCNeverLeaveCorpse = 0x4100029A, // NPCNeverLeaveCorpse
    UsageHeroOnly = 0x4100029B,       // UsageHeroOnly
    UsageNonHeroOnly = 0x4100029C,    // UsageNonHeroOnly

    ItemCraftFlags = 0x4100029F,                       // ItemCraftFlags
    WeaponFocusCost = 0x410002A0,                      // WeaponFocusCost
    AICombatNaturalFocusCost = 0x410002A1,             // AICombatNaturalFocusCost
    GameplayStatisticsDeathFocus = 0x410002A2,         // GameplayStatisticsDeathFocus
    UsageShouldUnlockUserForUsageEffects = 0x410002A3, // UsageShouldUnlockUserForUsageEffects
    UsageRequiredArcaneLore = 0x410002A4,              // UsageRequiredArcaneLore
    ItemIsIncomparableItem = 0x410002A5,               // ItemIsIncomparableItem
    UsageCancelSafeModeOnUsage = 0x410002A6,           // UsageCancelSafeModeOnUsage
    AppearanceMutationKey = 0x410002A7,                // AppearanceMutationKey
    AppearanceMutationKeyValue = 0x410002A8,           // AppearanceMutationKeyValue
    UsageItemInteractionTable = 0x410002A9,            // UsageItemInteractionTable
    NPCSkillTargetFlags = 0x410002AA,                  // NPCSkillTargetFlags
    ItemDurabilityMaxLevel = 0x410002AB,               // ItemDurabilityMaxLevel
    ItemDurabilityDecayMod = 0x410002AC,               // ItemDurabilityDecayMod
    BookSource = 0x410002AD,                           // BookSource
    UsageRequiredLocation = 0x410002AE,                // UsageRequiredLocation
    UsageRequiredLocationTsysMin = 0x410002AF,         // UsageRequiredLocationTsysMin
    UsageRequiredLocationTsysMax = 0x410002B0,         // UsageRequiredLocationTsysMax
    UsageErrorMessagesTableID = 0x410002B1,            // UsageErrorMessagesTableID
    UsageRequiredLocationRadius = 0x410002B2,          // UsageRequiredLocationRadius
    UsageRequiredLocationCloseProximityMsgRadius = 0x410002B3, // UsageRequiredLocationCloseProximityMsgRadius
    UsageRequiredLocationMediumProximityMsgRadius = 0x410002B4, // UsageRequiredLocationMediumProximityMsgRadius
    UsageRequiredLocationFarProximityMsgRadius = 0x410002B5, // UsageRequiredLocationFarProximityMsgRadius
    UsageRequiredLocationFeedbackType = 0x410002B6,          // UsageRequiredLocationFeedbackType

    UsageRequiredLocationVeryFarProximityMsgRadius = 0x410002B8, // UsageRequiredLocationVeryFarProximityMsgRadius
    Inscription = 0x410002B9,                                    // Inscription
    AuthorName = 0x410002BA,                                     // AuthorName
    RadarColor = 0x410002BB,                                     // RadarColor
    UsageCriticalSuccessMessage = 0x410002BC,                    // UsageCriticalSuccessMessage
    UsageRequiredCraftSkill = 0x410002BD,                        // UsageRequiredCraftSkill
    UsageRequiredCraftSkillRating = 0x410002BE,                  // UsageRequiredCraftSkillRating
    UsageSummonerOnly = 0x410002BF,                              // UsageSummonerOnly
    UsageDuration = 0x410002C0,                                  // UsageDuration
    ForgeEffect = 0x410002C1,                                    // ForgeEffect
    ForgeEffectRadius = 0x410002C2,                              // ForgeEffectRadius
    MineRequiredEffect = 0x410002C3,                             // MineRequiredEffect
    CraftSkill = 0x410002C4,                                     // CraftSkill
    MineObject = 0x410002C5,                                     // MineObject
    MineObjectQuantity = 0x410002C6,                             // MineObjectQuantity
    MineObjectQuantityVariance = 0x410002C7,                     // MineObjectQuantityVariance
    MineMaxUses = 0x410002C8,                                    // MineMaxUses
    MineUsageResetTime = 0x410002C9,                             // MineUsageResetTime
    AIHealthWarningLevel = 0x410002CA,                           // AIHealthWarningLevel
    AIHealthAggressiveness = 0x410002CB,                         // AIHealthAggressiveness
    AIVigorWarningLevel = 0x410002CC,                            // AIVigorWarningLevel
    AIVigorAggressiveness = 0x410002CD,                          // AIVigorAggressiveness
    ButcheryProfile = 0x410002CE,                                // ButcheryProfile
    ItemSlots = 0x410002CF,                                      // ItemSlots
    ItemDurabilityCurrentLevel = 0x410002D0,                     // ItemDurabilityCurrentLevel
    ItemIsExtractable = 0x410002D1,                              // ItemIsExtractable
    EffectIsExtractable = 0x410002D2,                            // EffectIsExtractable
    CraftRecipeCraftSkill = 0x410002D3,                          // CraftRecipeCraftSkill
    CraftToolXPMod = 0x410002D4,                                 // CraftToolXPMod
    CraftToolQuantityMod = 0x410002D5,                           // CraftToolQuantityMod
    CraftToolCraftSkillMod = 0x410002D6,                         // CraftToolCraftSkillMod
    UsageToolRequiredSkillLevel = 0x410002D7,                    // UsageToolRequiredSkillLevel
    CraftDyePlantMod = 0x410002D8,                               // CraftDyePlantMod
    UsageUserAnimationRepeatCount = 0x410002D9,                  // UsageUserAnimationRepeatCount
    NPCBypassableArmorMod = 0x410002DA,                          // NPCBypassableArmorMod
    ActivationType = 0x410002DB,                                 // ActivationType
    UsageDurabilityLostOnUse = 0x410002DC,                       // UsageDurabilityLostOnUse

    StoreTemplate = 0x410002DE,            // StoreTemplate
    EffectIsMonsterOnly = 0x410002DF,      // EffectIsMonsterOnly
    NPCCombatSpeedResistance = 0x410002E0, // NPCCombatSpeedResistance

    BookShowControls = 0x410002E2,                // BookShowControls
    BookImage = 0x410002E3,                       // BookImage
    NPCDeathEffect5 = 0x410002E4,                 // NPCDeathEffect5
    NPCDeathEffect5Stat = 0x410002E5,             // NPCDeathEffect5Stat
    NPCDeathEffect6 = 0x410002E6,                 // NPCDeathEffect6
    NPCDeathEffect6Stat = 0x410002E7,             // NPCDeathEffect6Stat
    NPCDeathEffect7 = 0x410002E8,                 // NPCDeathEffect7
    NPCDeathEffect7Stat = 0x410002E9,             // NPCDeathEffect7Stat
    NPCDeathEffect8 = 0x410002EA,                 // NPCDeathEffect8
    NPCDeathEffect8Stat = 0x410002EB,             // NPCDeathEffect8Stat
    StoreGroup = 0x410002EC,                      // StoreGroup
    MaxConsignments = 0x410002ED,                 // MaxConsignments
    AINearDeathLevel = 0x410002EE,                // AINearDeathLevel
    PortalScene = 0x410002EF,                     // PortalScene
    StoreLocation = 0x410002F0,                   // StoreLocation
    UsageIsBindOnUse = 0x410002F1,                // UsageIsBindOnUse
    UsageLegionsExpansionOnly = 0x410002F2,       // UsageLegionsExpansionOnly
    AICombatTargetConsiderInterval = 0x410002F3,  // AICombatTargetConsiderInterval
    NPCArmorThreshold = 0x410002F4,               // NPCArmorThreshold
    NPCDeathLootMinQuantityOverride = 0x410002F5, // NPCDeathLootMinQuantityOverride
    NPCDeathLootMaxQuantityOverride = 0x410002F6, // NPCDeathLootMaxQuantityOverride
    NPCDeathLootAbsoluteOverride = 0x410002F7,    // NPCDeathLootAbsoluteOverride
    NPCDamageTypeMod = 0x410002F8,                // NPCDamageTypeMod
    NPCDamageType = 0x410002F9,                   // NPCDamageType
    NPCCorpseOverrideEntity = 0x410002FA,         // NPCCorpseOverrideEntity
    NPCMungeCorpseOverrideEntity = 0x410002FB,    // NPCMungeCorpseOverrideEntity
    NPCTrophyDrop1 = 0x410002FC,                  // NPCTrophyDrop1
    NPCTrophyDrop1Rate = 0x410002FD,              // NPCTrophyDrop1Rate
    NPCTrophyDrop2 = 0x410002FE,                  // NPCTrophyDrop2
    NPCTrophyDrop2Rate = 0x410002FF,              // NPCTrophyDrop2Rate
    NPCTrophyDrop3 = 0x41000300,                  // NPCTrophyDrop3
    NPCTrophyDrop3Rate = 0x41000301,              // NPCTrophyDrop3Rate
    NPCTrophyDrop4 = 0x41000302,                  // NPCTrophyDrop4
    NPCTrophyDrop4Rate = 0x41000303,              // NPCTrophyDrop4Rate
    NPCTrophyDrop5 = 0x41000304,                  // NPCTrophyDrop5
    NPCTrophyDrop5Rate = 0x41000305,              // NPCTrophyDrop5Rate
    NPCTrophyDrop6 = 0x41000306,                  // NPCTrophyDrop6
    NPCTrophyDrop6Rate = 0x41000307,              // NPCTrophyDrop6Rate
    NPCTrophyDrop7 = 0x41000308,                  // NPCTrophyDrop7
    NPCTrophyDrop7Rate = 0x41000309,              // NPCTrophyDrop7Rate
    NPCTrophyDrop8 = 0x4100030A,                  // NPCTrophyDrop8
    NPCTrophyDrop8Rate = 0x4100030B,              // NPCTrophyDrop8Rate
    NPCTrophyDrop9 = 0x4100030C,                  // NPCTrophyDrop9
    NPCTrophyDrop9Rate = 0x4100030D,              // NPCTrophyDrop9Rate
    NPCTrophyDrop10 = 0x4100030E,                 // NPCTrophyDrop10
    NPCTrophyDrop10Rate = 0x4100030F,             // NPCTrophyDrop10Rate
    NPCTrophyDrop11 = 0x41000310,                 // NPCTrophyDrop11
    NPCTrophyDrop11Rate = 0x41000311,             // NPCTrophyDrop11Rate
    NPCTrophyDrop12 = 0x41000312,                 // NPCTrophyDrop12
    NPCTrophyDrop12Rate = 0x41000313,             // NPCTrophyDrop12Rate
    NPCTrophyDrop13 = 0x41000314,                 // NPCTrophyDrop13
    NPCTrophyDrop13Rate = 0x41000315,             // NPCTrophyDrop13Rate
    NPCTrophyDrop14 = 0x41000316,                 // NPCTrophyDrop14
    NPCTrophyDrop14Rate = 0x41000317,             // NPCTrophyDrop14Rate
    NPCTrophyDrop15 = 0x41000318,                 // NPCTrophyDrop15
    NPCTrophyDrop15Rate = 0x41000319,             // NPCTrophyDrop15Rate
    NPCSpecialTrophyDrop1 = 0x4100031A,           // NPCSpecialTrophyDrop1
    NPCSpecialTrophyDrop1Rate = 0x4100031B,       // NPCSpecialTrophyDrop1Rate
    NPCSpecialTrophyDrop2 = 0x4100031C,           // NPCSpecialTrophyDrop2
    NPCSpecialTrophyDrop2Rate = 0x4100031D,       // NPCSpecialTrophyDrop2Rate
    NPCSpecialTrophyDrop3 = 0x4100031E,           // NPCSpecialTrophyDrop3
    NPCSpecialTrophyDrop3Rate = 0x4100031F,       // NPCSpecialTrophyDrop3Rate
    NPCSpecialTrophyDrop4 = 0x41000320,           // NPCSpecialTrophyDrop4
    NPCSpecialTrophyDrop4Rate = 0x41000321,       // NPCSpecialTrophyDrop4Rate
    NPCSpecialTrophyDrop5 = 0x41000322,           // NPCSpecialTrophyDrop5
    NPCSpecialTrophyDrop5Rate = 0x41000323,       // NPCSpecialTrophyDrop5Rate

    ItemValidTargetWeenieType = 0x41000325, // ItemValidTargetWeenieType
    ItemNatureDamageMod = 0x41000326,       // ItemNatureDamageMod
    ItemDecayDamageMod = 0x41000327,        // ItemDecayDamageMod
    ItemMartialDamageMod = 0x41000328,      // ItemMartialDamageMod
    ItemArcaneDamageMod = 0x41000329,       // ItemArcaneDamageMod
    ItemNatureDamageModCap = 0x4100032A,    // ItemNatureDamageModCap
    ItemDecayDamageModCap = 0x4100032B,     // ItemDecayDamageModCap
    ItemMartialDamageModCap = 0x4100032C,   // ItemMartialDamageModCap
    ItemArcaneDamageModCap = 0x4100032D,    // ItemArcaneDamageModCap
    ItemNatureDamageModGrowthRate = 0x4100032E, // ItemNatureDamageModGrowthRate
    ItemDecayDamageModGrowthRate = 0x4100032F, // ItemDecayDamageModGrowthRate
    ItemMartialDamageModGrowthRate = 0x41000330, // ItemMartialDamageModGrowthRate
    ItemArcaneDamageModGrowthRate = 0x41000331, // ItemArcaneDamageModGrowthRate
    ItemNatureDamageModBaseMutabilityChance = 0x41000332, // ItemNatureDamageModBaseMutabilityChance
    ItemDecayDamageModBaseMutabilityChance = 0x41000333, // ItemDecayDamageModBaseMutabilityChance
    ItemMartialDamageModBaseMutabilityChance = 0x41000334, // ItemMartialDamageModBaseMutabilityChance
    ItemArcaneDamageModBaseMutabilityChance = 0x41000335, // ItemArcaneDamageModBaseMutabilityChance
    ItemIsDamageModMutable = 0x41000336,                  // ItemIsDamageModMutable
    AICombatMeleeNPC = 0x41000337,                        // AICombatMeleeNPC
    AICombatMissileNPC = 0x41000338,                      // AICombatMissileNPC
    AIHarvestingVariance = 0x41000339,                    // AIHarvestingVariance
    NPCNeverDropTesserae = 0x4100033A,                    // NPCNeverDropTesserae
    NPCNeverDropLodestones = 0x4100033B,                  // NPCNeverDropLodestones
    QuestRetryCancelled = 0x4100033C,                     // QuestRetryCancelled

    Gravity = 0x80000001,     // Gravity
    Temperature = 0x80000002, // Temperature
    Humidity = 0x80000003,    // Humidity
    Passability = 0x80000004, // Passability

    Friction = 0x80000006,         // Friction
    EncounterDensity = 0x80000007, // Encounter-Density

    FactionStatus = 0x81000003, // FactionStatus
}

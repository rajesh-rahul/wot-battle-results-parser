mod methods;
mod structures;
pub(crate) use methods::*;
pub(crate) use structures::*;
mod v0_9_12_0;
mod v0_9_13_0;
mod v0_9_14_0;
mod v0_9_15_0;
mod v0_9_16_0;
mod v0_9_17_0;
mod v0_9_18_0;
mod v0_9_19_0;
mod v0_9_20_0;
mod v0_9_21_0;
mod v0_9_22_0;
mod v0_9_23_0;
mod v1_0_0_0;
mod v1_0_1_0;
mod v1_0_2_0;
mod v1_10_0_0;
mod v1_10_1_0;
mod v1_11_0_0;
mod v1_11_1_0;
mod v1_12_0_0;
mod v1_12_1_0;
mod v1_13_0_0;
mod v1_14_0_0;
mod v1_14_1_0;
mod v1_15_0_0;
mod v1_16_0_0;
mod v1_16_1_0;
mod v1_17_0_0;
mod v1_17_1_0;
mod v1_18_0_0;
mod v1_18_1_0;
mod v1_19_0_0;
mod v1_19_1_0;
mod v1_1_0_0;
mod v1_20_0_0;
mod v1_20_1_0;
mod v1_21_0_0;
mod v1_21_1_0;
mod v1_22_0_0;
mod v1_22_1_0;
mod v1_23_0_0;
mod v1_23_1_0;
mod v1_24_0_0;
mod v1_24_1_0;
mod v1_25_0_0;
mod v1_25_1_0;
mod v1_26_0_0;
mod v1_26_1_0;
mod v1_2_0_0;
mod v1_3_0_0;
mod v1_4_0_0;
mod v1_4_1_0;
mod v1_5_0_0;
mod v1_5_1_0;
mod v1_6_0_0;
mod v1_6_1_0;
mod v1_7_0_0;
mod v1_7_1_0;
mod v1_8_0_0;
mod v1_9_0_0;


pub const WOT_DATA_ALL_VERSIONS: phf::Map<&'static str, WotDataForVersion> = phf::phf_map! {
    "0_9_12_0" => v0_9_12_0::DATA_0_9_12_0,
    "0_9_13_0" => v0_9_13_0::DATA_0_9_13_0,
    "0_9_14_0" => v0_9_14_0::DATA_0_9_14_0,
    "0_9_15_0" => v0_9_15_0::DATA_0_9_15_0,
    "0_9_16_0" => v0_9_16_0::DATA_0_9_16_0,
    "0_9_17_0" => v0_9_17_0::DATA_0_9_17_0,
    "0_9_18_0" => v0_9_18_0::DATA_0_9_18_0,
    "0_9_19_0" => v0_9_19_0::DATA_0_9_19_0,
    "0_9_20_0" => v0_9_20_0::DATA_0_9_20_0,
    "0_9_21_0" => v0_9_21_0::DATA_0_9_21_0,
    "0_9_22_0" => v0_9_22_0::DATA_0_9_22_0,
    "0_9_23_0" => v0_9_23_0::DATA_0_9_23_0,
    "1_0_0_0" => v1_0_0_0::DATA_1_0_0_0,
    "1_0_1_0" => v1_0_1_0::DATA_1_0_1_0,
    "1_0_2_0" => v1_0_2_0::DATA_1_0_2_0,
    "1_1_0_0" => v1_1_0_0::DATA_1_1_0_0,
    "1_2_0_0" => v1_2_0_0::DATA_1_2_0_0,
    "1_3_0_0" => v1_3_0_0::DATA_1_3_0_0,
    "1_4_0_0" => v1_4_0_0::DATA_1_4_0_0,
    "1_4_1_0" => v1_4_1_0::DATA_1_4_1_0,
    "1_5_0_0" => v1_5_0_0::DATA_1_5_0_0,
    "1_5_1_0" => v1_5_1_0::DATA_1_5_1_0,
    "1_6_0_0" => v1_6_0_0::DATA_1_6_0_0,
    "1_6_1_0" => v1_6_1_0::DATA_1_6_1_0,
    "1_7_0_0" => v1_7_0_0::DATA_1_7_0_0,
    "1_7_1_0" => v1_7_1_0::DATA_1_7_1_0,
    "1_8_0_0" => v1_8_0_0::DATA_1_8_0_0,
    "1_9_0_0" => v1_9_0_0::DATA_1_9_0_0,
    "1_10_0_0" => v1_10_0_0::DATA_1_10_0_0,
    "1_10_1_0" => v1_10_1_0::DATA_1_10_1_0,
    "1_11_0_0" => v1_11_0_0::DATA_1_11_0_0,
    "1_11_1_0" => v1_11_1_0::DATA_1_11_1_0,
    "1_12_0_0" => v1_12_0_0::DATA_1_12_0_0,
    "1_12_1_0" => v1_12_1_0::DATA_1_12_1_0,
    "1_13_0_0" => v1_13_0_0::DATA_1_13_0_0,
    "1_14_0_0" => v1_14_0_0::DATA_1_14_0_0,
    "1_14_1_0" => v1_14_1_0::DATA_1_14_1_0,
    "1_15_0_0" => v1_15_0_0::DATA_1_15_0_0,
    "1_16_0_0" => v1_16_0_0::DATA_1_16_0_0,
    "1_16_1_0" => v1_16_1_0::DATA_1_16_1_0,
    "1_17_0_0" => v1_17_0_0::DATA_1_17_0_0,
    "1_17_1_0" => v1_17_1_0::DATA_1_17_1_0,
    "1_18_0_0" => v1_18_0_0::DATA_1_18_0_0,
    "1_18_1_0" => v1_18_1_0::DATA_1_18_1_0,
    "1_19_0_0" => v1_19_0_0::DATA_1_19_0_0,
    "1_19_1_0" => v1_19_1_0::DATA_1_19_1_0,
    "1_20_0_0" => v1_20_0_0::DATA_1_20_0_0,
    "1_20_1_0" => v1_20_1_0::DATA_1_20_1_0,
    "1_21_0_0" => v1_21_0_0::DATA_1_21_0_0,
    "1_21_1_0" => v1_21_1_0::DATA_1_21_1_0,
    "1_22_0_0" => v1_22_0_0::DATA_1_22_0_0,
    "1_22_1_0" => v1_22_1_0::DATA_1_22_1_0,
    "1_23_0_0" => v1_23_0_0::DATA_1_23_0_0,
    "1_23_1_0" => v1_23_1_0::DATA_1_23_1_0,
    "1_24_0_0" => v1_24_0_0::DATA_1_24_0_0,
    "1_24_1_0" => v1_24_1_0::DATA_1_24_1_0,
    "1_25_0_0" => v1_25_0_0::DATA_1_25_0_0,
    "1_25_1_0" => v1_25_1_0::DATA_1_25_1_0,
    "1_26_0_0" => v1_26_0_0::DATA_1_26_0_0,
    "1_26_1_0" => v1_26_1_0::DATA_1_26_1_0,
};

pub const VERSIONS: &'static [[u16; 4]] = &[
    [0, 9, 12, 0],
    [0, 9, 13, 0],
    [0, 9, 14, 0],
    [0, 9, 15, 0],
    [0, 9, 16, 0],
    [0, 9, 17, 0],
    [0, 9, 18, 0],
    [0, 9, 19, 0],
    [0, 9, 20, 0],
    [0, 9, 21, 0],
    [0, 9, 22, 0],
    [0, 9, 23, 0],
    [1, 0, 0, 0],
    [1, 0, 1, 0],
    [1, 0, 2, 0],
    [1, 1, 0, 0],
    [1, 2, 0, 0],
    [1, 3, 0, 0],
    [1, 4, 0, 0],
    [1, 4, 1, 0],
    [1, 5, 0, 0],
    [1, 5, 1, 0],
    [1, 6, 0, 0],
    [1, 6, 1, 0],
    [1, 7, 0, 0],
    [1, 7, 1, 0],
    [1, 8, 0, 0],
    [1, 9, 0, 0],
    [1, 10, 0, 0],
    [1, 10, 1, 0],
    [1, 11, 0, 0],
    [1, 11, 1, 0],
    [1, 12, 0, 0],
    [1, 12, 1, 0],
    [1, 13, 0, 0],
    [1, 14, 0, 0],
    [1, 14, 1, 0],
    [1, 15, 0, 0],
    [1, 16, 0, 0],
    [1, 16, 1, 0],
    [1, 17, 0, 0],
    [1, 17, 1, 0],
    [1, 18, 0, 0],
    [1, 18, 1, 0],
    [1, 19, 0, 0],
    [1, 19, 1, 0],
    [1, 20, 0, 0],
    [1, 20, 1, 0],
    [1, 21, 0, 0],
    [1, 21, 1, 0],
    [1, 22, 0, 0],
    [1, 22, 1, 0],
    [1, 23, 0, 0],
    [1, 23, 1, 0],
    [1, 24, 0, 0],
    [1, 24, 1, 0],
    [1, 25, 0, 0],
    [1, 25, 1, 0],
    [1, 26, 0, 0],
    [1, 26, 1, 0],
];

#[derive(Debug, Copy, Clone, strum::Display, PartialEq, Eq, serde::Serialize)]
pub enum EntityType {
    Vehicle,
    ClientSelectableCGFObject,
    EnvironmentSwitcher,
    PhotoZone,
    EventPointsPickup,
    Login,
    SE20HangarVehicle,
    EventVehicle,
    DormantSelectableObject,
    SectorBase,
    Avatar,
    HB1HangarVehicle,
    DetachedTurret,
    CustomizableNewYearSceneObject,
    BootcampAccount,
    MusicStageCameraTarget,
    HalloweenHangarTank,
    NewYearTalismanObject,
    NewYearTalismanPreviewObject,
    AvatarInfo,
    NewYearCelebrityObject,
    CustomizableHB1SceneObject,
    ClientSelectableAdventCalendarHangarObject,
    PlatoonLighting,
    StepRepairPoint,
    ClientSelectableRacingHangarObject,
    ClientSelectableRTSObject,
    ProtectionZone,
    HangarPoster,
    TeamBase,
    PianoMusician,
    NewYearTalismanBaseObject,
    NetworkEntity,
    PersonalDeathZonesHelper,
    HeroTank,
    ClientSelectableAdventCalendarObject,
    NewYearBaseEntryObject,
    EventShowcaseVehicle,
    TeamInfo,
    NewYearTalismanPreviewCamera,
    NewYearHeroTank,
    ArenaInfo,
    GameEventEntryPoint,
    ClientSelectableCameraVehicle,
    LimitedVisibilityEntity,
    Flock,
    DestructibleEntity,
    PosterSelectableObject,
    EventControlPoint,
    ActionEffects,
    NewYearTalismanCamera,
    BossLandmine,
    HangarPortalGates,
    ClientSelectableMarathonHangarObject,
    HalloweenHangarTankHeadlight,
    MusicStage,
    LootBoxObject,
    NewYearIciclesIllumination,
    AnimatedScene,
    EventPointsBase,
    ApplicationPoint,
    AttackBomber,
    AnimatedModelObject,
    SE20HeroTank,
    EventPortal,
    GameObjectEntity,
    ClientSelectableHangarsSwitcher,
    ClientSelectableWebLinksOpener,
    ClientSelectableEasterEgg,
    Comp7Lighting,
    CameraTarget,
    HangarDog,
    NewYearJukeboxSelectableObject,
    BasicMine,
    ClientSelectableMusicStageObject,
    PersonalDeathZone,
    AttackArtilleryFort,
    HalloweenHealthPowerup,
    ClientSelectableObject,
    AreaOfEffect,
    SE20ClientSelectableObject,
    GroundLightNewYearSceneObject,
    NewYearPoster,
    ClientSelectableCameraObject,
    FlockExotic,
    OfflineFlag,
    AreaDestructibles,
    HalloweenSupplyDrop,
    HangarBirthdayPoster,
    ClientSelectableRTSLeaderboard,
    TeamBaseRecapturable,
    NewYearToyObject,
    DebugDrawEntity,
    YearHareAffairEntryPoint,
    StaticDeathZone,
    Sector,
    NewYearSelectableObject,
    ClientSelectableFrontLineHangarObject,
    RaceSoundZone,
    PlatoonTank,
    ClientSelectableBlackMarketObject,
    CustomizableSE20SceneObject,
    HangarVehicle,
    ArenaObserverInfo,
    NewYearTalismanEntryObject,
    Account,
    SimulatedVehicle,
    DogBowl,
    ClientSelectableRankedObject,
    NewYearIcicleObject,
    EmptyEntity,
    NewYearVisualObject,
    ClientSelectableMiniGamesHangarObject,
    NewYearCelebrityEntryObject,
    ClientSelectableWotAnniversaryObject,
    OfflineEntity,
    DeathZone,
    MusicStageCameraObject,
    CameraAnchorObject,
    NewYearJukeboxObject,
    ItemPickup,
}

use EntityType::*;

use crate::wot_data::*;
use crate::PacketName;
pub(crate) const DATA_0_9_18_0: crate::wot_data::WotDataForVersion = WotDataForVersion {
    special_formats: SpecialFormat {
        create_avatar: &[
            PropData {
                name:          "name",
                default_value: WotXmlDefaultValue::Str(""),
                datatype:      WotXmlType::String,
                size:          65535,
            },
            PropData {
                name:          "arenaUniqueID",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::UInt64,
                size:          8,
            },
            PropData {
                name:          "arenaTypeID",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::Int32,
                size:          4,
            },
            PropData {
                name:          "arenaBonusType",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::UInt8,
                size:          1,
            },
            PropData {
                name:          "arenaGuiType",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::UInt8,
                size:          1,
            },
            PropData {
                name:          "arenaExtraData",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::Python,
                size:          65535,
            },
            PropData {
                name:          "weatherPresetID",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::UInt8,
                size:          1,
            },
            PropData {
                name:          "denunciationsLeft",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::Int16,
                size:          2,
            },
            PropData {
                name:          "clientCtx",
                default_value: WotXmlDefaultValue::Str(""),
                datatype:      WotXmlType::String,
                size:          65535,
            },
            PropData {
                name:          "tkillIsSuspected",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::UInt8,
                size:          1,
            },
        ],
    },
    packet_map:      phf::phf_map! {
        0x00u32 => PacketName::CreateAvatar,
        0x05u32 => PacketName::CreateEntity,
        0x08u32 => PacketName::EntityMethod,
        0x0Au32 => PacketName::Position,
        0x18u32 => PacketName::GameVersion,
        0x23u32 => PacketName::Chat,
        0x3Du32 => PacketName::CryptoKey,
    },

    entities: &[
        EntityData {
            ty:              Account,
            exposed_props:   &[],
            exposed_methods: &[
                MethodData {
                    name: "onArenaCreated",
                    args: &[],
                    size: 1,
                },
                MethodData {
                    name: "onPrebattleLeft",
                    args: &[],
                    size: 1,
                },
                MethodData {
                    name: "resyncDossiers",
                    args: &[],
                    size: 1,
                },
                MethodData {
                    name: "reloadShop",
                    args: &[],
                    size: 1,
                },
                MethodData {
                    name: "onEnqueued",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onDequeued",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onKickedFromQueue",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onPrebattleJoinFailure",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onKickedFromArena",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onKickedFromPrebattle",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onCenterIsLongDisconnected",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onPrebattleJoined",
                    args: &[WotXmlType::Int32],
                    size: 5,
                },
                MethodData {
                    name: "onUnitCallOk",
                    args: &[WotXmlType::Int32],
                    size: 5,
                },
                MethodData {
                    name: "receiveServerStats",
                    args: &[WotXmlType::FixedDict {
                        inner:      &[
                            ("clusterCCU", WotXmlType::UInt32),
                            ("regionCCU", WotXmlType::UInt32),
                        ],
                        allow_none: false,
                    }],
                    size: 9,
                },
                MethodData {
                    name: "onUnitAssemblerSuccess",
                    args: &[WotXmlType::UInt64, WotXmlType::Int32],
                    size: 13,
                },
                MethodData {
                    name: "onTutorialEnqueued",
                    args: &[WotXmlType::UInt64, WotXmlType::UInt32, WotXmlType::Int32],
                    size: 17,
                },
                MethodData {
                    name: "onChatAction",
                    args: &[WotXmlType::FixedDict {
                        inner:      &[
                            ("requestID", WotXmlType::Int64),
                            ("action", WotXmlType::UInt8),
                            ("actionResponse", WotXmlType::UInt8),
                            ("time", WotXmlType::Float64),
                            ("sentTime", WotXmlType::Float64),
                            ("channel", WotXmlType::Int32),
                            ("originator", WotXmlType::Int64),
                            ("originatorNickName", WotXmlType::String),
                            ("group", WotXmlType::UInt8),
                            ("data", WotXmlType::Python),
                            ("flags", WotXmlType::UInt8),
                        ],
                        allow_none: false,
                    }],
                    size: 65536,
                },
                MethodData {
                    name: "messenger_onActionByServer_chat2",
                    args: &[
                        WotXmlType::Int16,
                        WotXmlType::UInt16,
                        WotXmlType::FixedDict {
                            inner:      &[
                                ("int32Arg1", WotXmlType::Int32),
                                ("int64Arg1", WotXmlType::Int64),
                                ("floatArg1", WotXmlType::Float64),
                                ("strArg1", WotXmlType::String),
                                ("strArg2", WotXmlType::String),
                            ],
                            allow_none: false,
                        },
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "onCmdResponse",
                    args: &[WotXmlType::Int16, WotXmlType::Int16, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onCmdResponseExt",
                    args: &[
                        WotXmlType::Int16,
                        WotXmlType::Int16,
                        WotXmlType::String,
                        WotXmlType::String,
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "receiveClubUpdate",
                    args: &[WotXmlType::Int64, WotXmlType::String, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "receiveClubNotification",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onTokenReceived",
                    args: &[WotXmlType::UInt16, WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "processInvitations",
                    args: &[WotXmlType::Python],
                    size: 65536,
                },
                MethodData {
                    name: "onKickedFromServer",
                    args: &[WotXmlType::String, WotXmlType::UInt8, WotXmlType::UInt32],
                    size: 65536,
                },
                MethodData {
                    name: "onEnqueueFailure",
                    args: &[WotXmlType::UInt8, WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onIGRTypeChanged",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onArenaJoinFailure",
                    args: &[WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "showGUI",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "receiveActiveArenas",
                    args: &[WotXmlType::Array {
                        inner: &WotXmlType::FixedDict {
                            inner:      &[
                                ("id", WotXmlType::Int32),
                                ("typeID", WotXmlType::Int32),
                                ("roundLength", WotXmlType::Int32),
                                ("roundStart", WotXmlType::Float32),
                            ],
                            allow_none: false,
                        },
                        size:  None,
                    }],
                    size: 65536,
                },
                MethodData {
                    name: "receiveQueueInfo",
                    args: &[WotXmlType::Python],
                    size: 65536,
                },
                MethodData {
                    name: "updatePrebattle",
                    args: &[WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "responseFortPublicInfo",
                    args: &[WotXmlType::Int32, WotXmlType::Int8, WotXmlType::Python],
                    size: 65536,
                },
                MethodData {
                    name: "update",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onUnitUpdate",
                    args: &[WotXmlType::UInt64, WotXmlType::String, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onUnitError",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::UInt64,
                        WotXmlType::Int32,
                        WotXmlType::String,
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "onUnitBrowserError",
                    args: &[WotXmlType::Int32, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onUnitBrowserResultsSet",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onUnitBrowserResultsUpdate",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onFortReply",
                    args: &[WotXmlType::UInt64, WotXmlType::Int32, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onFortUpdate",
                    args: &[WotXmlType::String, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onGlobalMapUpdate",
                    args: &[WotXmlType::String, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onGlobalMapReply",
                    args: &[WotXmlType::UInt64, WotXmlType::Int32, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onSendPrebattleInvites",
                    args: &[
                        WotXmlType::Int64,
                        WotXmlType::String,
                        WotXmlType::Int64,
                        WotXmlType::String,
                        WotXmlType::UInt64,
                        WotXmlType::UInt8,
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "onClanInfoReceived",
                    args: &[
                        WotXmlType::Int64,
                        WotXmlType::String,
                        WotXmlType::String,
                        WotXmlType::String,
                        WotXmlType::String,
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "receiveNotification",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
            ],
        },
        EntityData {
            ty:              Avatar,
            exposed_props:   &[
                PropData {
                    name:          "isObserverFPV",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "observerFPVControlMode",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "numOfObservers",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "team",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "isObserverBothTeams",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "isGunLocked",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "ownVehicleGear",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "playerVehicleID",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Int32,
                    size:          4,
                },
                PropData {
                    name:          "ownVehicleAuxPhysicsData",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt64,
                    size:          8,
                },
                PropData {
                    name:          "remoteCameraStrategic",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::FixedDict {
                        inner:      &[("time", WotXmlType::Float64), ("shotPoint", WotXmlType::Vector3)],
                        allow_none: false,
                    },
                    size:          20,
                },
                PropData {
                    name:          "remoteCameraArcade",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::FixedDict {
                        inner:      &[
                            ("time", WotXmlType::Float64),
                            ("relTranslation", WotXmlType::Vector3),
                            ("shotPoint", WotXmlType::Vector3),
                        ],
                        allow_none: false,
                    },
                    size:          32,
                },
                PropData {
                    name:          "remoteCameraSniper",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::FixedDict {
                        inner:      &[
                            ("time", WotXmlType::Float64),
                            ("camMatrixTranslation", WotXmlType::Vector3),
                            ("camMatrixRotation", WotXmlType::Vector3),
                            ("zoom", WotXmlType::UInt8),
                        ],
                        allow_none: false,
                    },
                    size:          33,
                },
                PropData {
                    name:          "remoteCameraArty",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::FixedDict {
                        inner:      &[
                            ("time", WotXmlType::Float64),
                            ("shotPoint", WotXmlType::Vector3),
                            ("translation", WotXmlType::Vector3),
                            ("rotation", WotXmlType::Vector3),
                        ],
                        allow_none: false,
                    },
                    size:          44,
                },
            ],
            exposed_methods: &[
                MethodData {
                    name: "onAutoAimVehicleLost",
                    args: &[],
                    size: 1,
                },
                MethodData {
                    name: "onKickedFromArena",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onRoundFinished",
                    args: &[WotXmlType::Int8, WotXmlType::UInt8],
                    size: 3,
                },
                MethodData {
                    name: "syncVehicleAttrs",
                    args: &[WotXmlType::FixedDict {
                        inner:      &[("circularVisionRadius", WotXmlType::UInt16)],
                        allow_none: false,
                    }],
                    size: 3,
                },
                MethodData {
                    name: "updateResourceAmount",
                    args: &[WotXmlType::UInt8, WotXmlType::UInt32],
                    size: 6,
                },
                MethodData {
                    name: "receiveHorn",
                    args: &[WotXmlType::Int32, WotXmlType::UInt8, WotXmlType::UInt8],
                    size: 7,
                },
                MethodData {
                    name: "updateVehicleOptionalDeviceStatus",
                    args: &[WotXmlType::Int32, WotXmlType::UInt8, WotXmlType::UInt8],
                    size: 7,
                },
                MethodData {
                    name: "onRepairPointAction",
                    args: &[WotXmlType::UInt8, WotXmlType::UInt8, WotXmlType::Float32],
                    size: 7,
                },
                MethodData {
                    name: "updateVehicleHealth",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Int16,
                        WotXmlType::Int8,
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                    ],
                    size: 10,
                },
                MethodData {
                    name: "updateVehicleSetting",
                    args: &[WotXmlType::Int32, WotXmlType::UInt8, WotXmlType::Int32],
                    size: 10,
                },
                MethodData {
                    name: "updateGasAttackState",
                    args: &[WotXmlType::UInt8, WotXmlType::Float32, WotXmlType::Float32],
                    size: 10,
                },
                MethodData {
                    name: "showVehicleDamageInfo",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                        WotXmlType::Int32,
                        WotXmlType::UInt8,
                    ],
                    size: 12,
                },
                MethodData {
                    name: "updateVehicleGunReloadTime",
                    args: &[WotXmlType::Int32, WotXmlType::Float32, WotXmlType::Float32],
                    size: 13,
                },
                MethodData {
                    name: "updateVehicleAmmo",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Int32,
                        WotXmlType::UInt16,
                        WotXmlType::UInt8,
                        WotXmlType::Int16,
                    ],
                    size: 14,
                },
                MethodData {
                    name: "updateVehicleMiscStatus",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::UInt8,
                        WotXmlType::Int32,
                        WotXmlType::Float32,
                    ],
                    size: 14,
                },
                MethodData {
                    name: "onSwitchViewpoint",
                    args: &[WotXmlType::Int32, WotXmlType::Vector3],
                    size: 17,
                },
                MethodData {
                    name: "stopTracer",
                    args: &[WotXmlType::Int32, WotXmlType::Vector3],
                    size: 17,
                },
                MethodData {
                    name: "onCollisionWithVehicle",
                    args: &[WotXmlType::Vector3, WotXmlType::Float32],
                    size: 17,
                },
                MethodData {
                    name: "onFrictionWithVehicle",
                    args: &[WotXmlType::Int32, WotXmlType::Vector3, WotXmlType::UInt8],
                    size: 18,
                },
                MethodData {
                    name: "showOwnVehicleHitDirection",
                    args: &[
                        WotXmlType::Float32,
                        WotXmlType::Int32,
                        WotXmlType::UInt16,
                        WotXmlType::UInt32,
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                        WotXmlType::Int32,
                    ],
                    size: 21,
                },
                MethodData {
                    name: "battleEventsSummary",
                    args: &[WotXmlType::FixedDict {
                        inner:      &[
                            ("damage", WotXmlType::UInt32),
                            ("trackAssist", WotXmlType::UInt32),
                            ("radioAssist", WotXmlType::UInt32),
                            ("stunAssist", WotXmlType::UInt32),
                            ("tankings", WotXmlType::UInt32),
                            ("lastKillerID", WotXmlType::Int32),
                            ("lastDeathReasonID", WotXmlType::UInt8),
                        ],
                        allow_none: false,
                    }],
                    size: 26,
                },
                MethodData {
                    name: "updateGunMarker",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::Float32,
                    ],
                    size: 33,
                },
                MethodData {
                    name: "updateOwnVehiclePosition",
                    args: &[
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                    ],
                    size: 33,
                },
                MethodData {
                    name: "showHittingArea",
                    args: &[
                        WotXmlType::UInt16,
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::Float64,
                    ],
                    size: 35,
                },
                MethodData {
                    name: "showCarpetBombing",
                    args: &[
                        WotXmlType::UInt16,
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::Float64,
                    ],
                    size: 35,
                },
                MethodData {
                    name: "updateTargetingInfo",
                    args: &[
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                    ],
                    size: 37,
                },
                MethodData {
                    name: "showTracer",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Int32,
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                    ],
                    size: 43,
                },
                MethodData {
                    name: "updateBomberTrajectory",
                    args: &[
                        WotXmlType::UInt16,
                        WotXmlType::UInt8,
                        WotXmlType::Float64,
                        WotXmlType::Vector3,
                        WotXmlType::Vector2,
                        WotXmlType::Float64,
                        WotXmlType::Vector3,
                        WotXmlType::Vector2,
                        WotXmlType::UInt8,
                    ],
                    size: 61,
                },
                MethodData {
                    name: "onChatAction",
                    args: &[WotXmlType::FixedDict {
                        inner:      &[
                            ("requestID", WotXmlType::Int64),
                            ("action", WotXmlType::UInt8),
                            ("actionResponse", WotXmlType::UInt8),
                            ("time", WotXmlType::Float64),
                            ("sentTime", WotXmlType::Float64),
                            ("channel", WotXmlType::Int32),
                            ("originator", WotXmlType::Int64),
                            ("originatorNickName", WotXmlType::String),
                            ("group", WotXmlType::UInt8),
                            ("data", WotXmlType::Python),
                            ("flags", WotXmlType::UInt8),
                        ],
                        allow_none: false,
                    }],
                    size: 65536,
                },
                MethodData {
                    name: "messenger_onActionByServer_chat2",
                    args: &[
                        WotXmlType::Int16,
                        WotXmlType::UInt16,
                        WotXmlType::FixedDict {
                            inner:      &[
                                ("int32Arg1", WotXmlType::Int32),
                                ("int64Arg1", WotXmlType::Int64),
                                ("floatArg1", WotXmlType::Float64),
                                ("strArg1", WotXmlType::String),
                                ("strArg2", WotXmlType::String),
                            ],
                            allow_none: false,
                        },
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "onCmdResponse",
                    args: &[WotXmlType::Int16, WotXmlType::Int16, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onCmdResponseExt",
                    args: &[
                        WotXmlType::Int16,
                        WotXmlType::Int16,
                        WotXmlType::String,
                        WotXmlType::String,
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "processInvitations",
                    args: &[WotXmlType::Python],
                    size: 65536,
                },
                MethodData {
                    name: "onTokenReceived",
                    args: &[WotXmlType::UInt16, WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "update",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "onKickedFromServer",
                    args: &[WotXmlType::String, WotXmlType::UInt8, WotXmlType::UInt32],
                    size: 65536,
                },
                MethodData {
                    name: "onIGRTypeChanged",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "receiveAccountStats",
                    args: &[WotXmlType::UInt32, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "redrawVehicleOnRespawn",
                    args: &[WotXmlType::Int32, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "showOtherVehicleDamagedDevices",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt8,
                            size:  None,
                        },
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt8,
                            size:  None,
                        },
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "showShotResults",
                    args: &[WotXmlType::Array {
                        inner: &WotXmlType::UInt64,
                        size:  None,
                    }],
                    size: 65536,
                },
                MethodData {
                    name: "showDevelopmentInfo",
                    args: &[WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "explodeProjectile",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt32,
                            size:  None,
                        },
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "onBattleEvents",
                    args: &[WotXmlType::Array {
                        inner: &WotXmlType::FixedDict {
                            inner:      &[
                                ("eventType", WotXmlType::UInt8),
                                ("targetID", WotXmlType::Int32),
                                ("details", WotXmlType::UInt32),
                                ("count", WotXmlType::UInt16),
                            ],
                            allow_none: false,
                        },
                        size:  None,
                    }],
                    size: 65536,
                },
                MethodData {
                    name: "updateArena",
                    args: &[WotXmlType::UInt8, WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "updatePositions",
                    args: &[
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt8,
                            size:  None,
                        },
                        WotXmlType::Array {
                            inner: &WotXmlType::Int16,
                            size:  None,
                        },
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "receivePhysicsDebugInfo",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "updateCarriedFlagPositions",
                    args: &[
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt8,
                            size:  None,
                        },
                        WotXmlType::Array {
                            inner: &WotXmlType::Int16,
                            size:  None,
                        },
                    ],
                    size: 65536,
                },
                MethodData {
                    name: "receiveNotification",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
                MethodData {
                    name: "updateAvatarPrivateStats",
                    args: &[WotXmlType::String],
                    size: 65536,
                },
            ],
        },
        EntityData {
            ty:              ClientSelectableObject,
            exposed_props:   &[
                PropData {
                    name:          "modelName",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
                PropData {
                    name:          "selectionId",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
                PropData {
                    name:          "mouseOverSoundName",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
                PropData {
                    name:          "clickSoundName",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
            ],
            exposed_methods: &[],
        },
        EntityData {
            ty:              HangarVehicle,
            exposed_props:   &[],
            exposed_methods: &[],
        },
        EntityData {
            ty:              Vehicle,
            exposed_props:   &[
                PropData {
                    name:          "isStrafing",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "physicsMode",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "siegeState",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "isCrewActive",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "gunAnglesPacked",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt16,
                    size:          2,
                },
                PropData {
                    name:          "health",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Int16,
                    size:          2,
                },
                PropData {
                    name:          "engineMode",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt8,
                        size:  Some(2),
                    },
                    size:          2,
                },
                PropData {
                    name:          "steeringAngle",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "stunInfo",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Float64,
                    size:          8,
                },
                PropData {
                    name:          "publicInfo",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::FixedDict {
                        inner:      &[
                            ("name", WotXmlType::String),
                            ("compDescr", WotXmlType::String),
                            ("index", WotXmlType::UInt8),
                            ("team", WotXmlType::UInt8),
                            ("prebattleID", WotXmlType::Int32),
                            ("marksOnGun", WotXmlType::UInt8),
                        ],
                        allow_none: false,
                    },
                    size:          65535,
                },
                PropData {
                    name:          "damageStickers",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt64,
                        size:  None,
                    },
                    size:          65535,
                },
                PropData {
                    name:          "publicStateModifiers",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt8,
                        size:  None,
                    },
                    size:          65535,
                },
            ],
            exposed_methods: &[
                MethodData {
                    name: "showShooting",
                    args: &[WotXmlType::UInt8],
                    size: 2,
                },
                MethodData {
                    name: "onHealthChanged",
                    args: &[WotXmlType::Int16, WotXmlType::Int32, WotXmlType::UInt8],
                    size: 8,
                },
                MethodData {
                    name: "onPushed",
                    args: &[WotXmlType::Float32, WotXmlType::Float32],
                    size: 9,
                },
                MethodData {
                    name: "showAmmoBayEffect",
                    args: &[WotXmlType::UInt8, WotXmlType::Float32, WotXmlType::Float32],
                    size: 10,
                },
                MethodData {
                    name: "showRammingEffect",
                    args: &[WotXmlType::Float32, WotXmlType::Vector3],
                    size: 17,
                },
                MethodData {
                    name: "showDamageFromExplosion",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Vector3,
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                    ],
                    size: 19,
                },
                MethodData {
                    name: "onStaticCollision",
                    args: &[
                        WotXmlType::Float32,
                        WotXmlType::Vector3,
                        WotXmlType::Vector3,
                        WotXmlType::UInt8,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Float32,
                        WotXmlType::Int8,
                        WotXmlType::UInt8,
                    ],
                    size: 44,
                },
                MethodData {
                    name: "showDamageFromShot",
                    args: &[
                        WotXmlType::Int32,
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt64,
                            size:  None,
                        },
                        WotXmlType::UInt8,
                        WotXmlType::UInt8,
                    ],
                    size: 65536,
                },
            ],
        },
        EntityData {
            ty:              AreaDestructibles,
            exposed_props:   &[
                PropData {
                    name:          "destroyedModules",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt16,
                        size:  None,
                    },
                    size:          65535,
                },
                PropData {
                    name:          "destroyedFragiles",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt16,
                        size:  None,
                    },
                    size:          65535,
                },
                PropData {
                    name:          "fallenColumns",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt16,
                        size:  None,
                    },
                    size:          65535,
                },
                PropData {
                    name:          "fallenTrees",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Array {
                        inner: &WotXmlType::UInt32,
                        size:  None,
                    },
                    size:          65535,
                },
            ],
            exposed_methods: &[],
        },
        EntityData {
            ty:              OfflineEntity,
            exposed_props:   &[],
            exposed_methods: &[],
        },
        EntityData {
            ty:              Flock,
            exposed_props:   &[
                PropData {
                    name:          "modelCount",
                    default_value: WotXmlDefaultValue::Int(5),
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "flyAroundCenter",
                    default_value: WotXmlDefaultValue::Int(0),
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "yawSpeed",
                    default_value: WotXmlDefaultValue::Float(1.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "pitchSpeed",
                    default_value: WotXmlDefaultValue::Float(0.002),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "rollSpeed",
                    default_value: WotXmlDefaultValue::Float(0.05),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "animSpeedMin",
                    default_value: WotXmlDefaultValue::Float(1.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "animSpeedMax",
                    default_value: WotXmlDefaultValue::Float(1.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "height",
                    default_value: WotXmlDefaultValue::Float(50.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "radius",
                    default_value: WotXmlDefaultValue::Float(100.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "deadZoneRadius",
                    default_value: WotXmlDefaultValue::Float(0.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "speedAtBottom",
                    default_value: WotXmlDefaultValue::Float(0.5),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "speedAtTop",
                    default_value: WotXmlDefaultValue::Float(0.2),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "decisionTime",
                    default_value: WotXmlDefaultValue::Float(7.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "modelName",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
                PropData {
                    name:          "modelName2",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
            ],
            exposed_methods: &[],
        },
        EntityData {
            ty:              FlockExotic,
            exposed_props:   &[
                PropData {
                    name:          "modelCount",
                    default_value: WotXmlDefaultValue::Int(5),
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "animSpeedMax",
                    default_value: WotXmlDefaultValue::Float(1.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "animSpeedMin",
                    default_value: WotXmlDefaultValue::Float(1.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "speed",
                    default_value: WotXmlDefaultValue::Float(0.2),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "accelerationTime",
                    default_value: WotXmlDefaultValue::Float(5.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "triggerRadius",
                    default_value: WotXmlDefaultValue::Float(20.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "spawnRadius",
                    default_value: WotXmlDefaultValue::Float(5.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "spawnHeight",
                    default_value: WotXmlDefaultValue::Float(5.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "flightRadius",
                    default_value: WotXmlDefaultValue::Float(50.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "flightHeight",
                    default_value: WotXmlDefaultValue::Float(15.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "flightAngleMin",
                    default_value: WotXmlDefaultValue::Float(0.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "flightAngleMax",
                    default_value: WotXmlDefaultValue::Float(360.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "flightOffsetFromOrigin",
                    default_value: WotXmlDefaultValue::Float(0.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "lifeTime",
                    default_value: WotXmlDefaultValue::Float(7.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "respawnTime",
                    default_value: WotXmlDefaultValue::Float(5.0),
                    datatype:      WotXmlType::Float32,
                    size:          4,
                },
                PropData {
                    name:          "initSpeedRandom",
                    default_value: WotXmlDefaultValue::List(&[
                        WotXmlDefaultValue::Float(0.3),
                        WotXmlDefaultValue::Float(0.4),
                    ]),
                    datatype:      WotXmlType::Vector2,
                    size:          8,
                },
                PropData {
                    name:          "speedRandom",
                    default_value: WotXmlDefaultValue::List(&[
                        WotXmlDefaultValue::Float(0.8),
                        WotXmlDefaultValue::Float(1.0),
                    ]),
                    datatype:      WotXmlType::Vector2,
                    size:          8,
                },
                PropData {
                    name:          "explosionRadius",
                    default_value: WotXmlDefaultValue::List(&[
                        WotXmlDefaultValue::Float(10.0),
                        WotXmlDefaultValue::Float(20.0),
                    ]),
                    datatype:      WotXmlType::Vector2,
                    size:          8,
                },
                PropData {
                    name:          "modelName",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
                PropData {
                    name:          "modelName2",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
                PropData {
                    name:          "flightSound",
                    default_value: WotXmlDefaultValue::Str("	ambient_nature_trigger_soaring_birds	"),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
            ],
            exposed_methods: &[],
        },
        EntityData {
            ty:              Login,
            exposed_props:   &[],
            exposed_methods: &[
                MethodData {
                    name: "onKickedFromServer",
                    args: &[WotXmlType::Int32],
                    size: 5,
                },
                MethodData {
                    name: "receiveLoginQueueNumber",
                    args: &[WotXmlType::UInt64],
                    size: 9,
                },
            ],
        },
        EntityData {
            ty:              DetachedTurret,
            exposed_props:   &[
                PropData {
                    name:          "isUnderWater",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "isCollidingWithWorld",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::UInt8,
                    size:          1,
                },
                PropData {
                    name:          "vehicleID",
                    default_value: WotXmlDefaultValue::Null,
                    datatype:      WotXmlType::Int32,
                    size:          4,
                },
                PropData {
                    name:          "vehicleCompDescr",
                    default_value: WotXmlDefaultValue::Str(""),
                    datatype:      WotXmlType::String,
                    size:          65535,
                },
            ],
            exposed_methods: &[
                MethodData {
                    name: "onStaticCollision",
                    args: &[WotXmlType::Float32, WotXmlType::Vector3, WotXmlType::Vector3],
                    size: 29,
                },
                MethodData {
                    name: "showDamageFromShot",
                    args: &[
                        WotXmlType::Array {
                            inner: &WotXmlType::UInt64,
                            size:  None,
                        },
                        WotXmlType::UInt8,
                    ],
                    size: 65536,
                },
            ],
        },
        EntityData {
            ty:              OfflineFlag,
            exposed_props:   &[PropData {
                name:          "flagID",
                default_value: WotXmlDefaultValue::Null,
                datatype:      WotXmlType::Int32,
                size:          4,
            }],
            exposed_methods: &[],
        },
    ],
};

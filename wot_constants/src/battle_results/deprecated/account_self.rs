use crate::battle_results::field_types::{FieldArenaType, ResultField};
use crate::battle_results::ResultFieldType;

pub const ACCOUNT_SELF: [ResultField; 34] = [
    ResultField("rankChange", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("avatarAmmo", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("avatarDamageEventList", "<type 'set'>", "set([])", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("accountDBID", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::AccountSelf),
    ResultField("team", "<type 'int'>", "1", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("clanDBID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("fortClanDBIDs", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("winnerIfDraw", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("isPrematureLeave", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("watchedBattleToTheEnd", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("vseBattleResults", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("squadBonusInfo", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("progressiveReward", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("eligibleForCrystalRewards", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("activeRents", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("recruitsIDs", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("recruiterID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("fareTeamXPPosition", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("questsProgress", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinDicts", ResultFieldType::AccountSelf),
    ResultField("PM2Progress", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("dogTags", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("eventCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventGold", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventCrystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventEventCoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventBpcoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("credits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("xp", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("freeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("crystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("goldBankGain", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
];
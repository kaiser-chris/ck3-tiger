use std::sync::LazyLock;

use crate::helpers::TigerHashSet;

pub(crate) static BUILTIN_MACROS_IMPERATOR: LazyLock<TigerHashSet<&'static str>> =
    LazyLock::new(|| BUILTIN_MACROS.iter().copied().collect());

// LAST UPDATED IMPERATOR VERSION 2.0.4
// The table entries were collected by analyzing tiger's own output.
const BUILTIN_MACROS: &[&str] = &[
    "ABILITY",
    "A_BLOCK",
    "ACCEPT",
    "ACTION",
    "ACTIVATE_EFFECT",
    "ACTIVATEEFFECT",
    "ACTIVITY",
    "ACTOR",
    "ADDREMOVE",
    "ADJ",
    "ADJECTIVE",
    "AE",
    "AGE",
    "AGENDA",
    "AGENDA_DESC",
    "AGENDA_TITLE",
    "AGRESSOR",
    "AI_MODIFIERS",
    "AIREASON",
    "ALMOST",
    "ALPHA",
    "AMBITION",
    "AMOUNT",
    "APPROVAL",
    "AREA",
    "ASCORE",
    "ASSASSIN",
    "ASSIMILATE",
    "ATTACKER",
    "ATTRITION",
    "BACKEND",
    "BASE_NODE_ID",
    "BASEVAL",
    "BEFORE",
    "BENEFIT",
    "BLUE",
    "BONUS",
    "BONUSEFFECT",
    "BREACHBONUS",
    "BREAKDOWN",
    "BUILDING",
    "BUILDING_NAME",
    "CAN_SUPPORT_UP_TO",
    "CAPACITY",
    "CAPITAL",
    "CAPTURE",
    "CAPVALUE",
    "CASH",
    "CASUS_BELLI",
    "CATEGORY_NAME",
    "CATEGORY_SLOT",
    "CENTER_X",
    "CENTRE",
    "CHANCE",
    "CHANGE",
    "CHANGES",
    "CHAR",
    "CHAR01",
    "CHAR02",
    "CHAR2",
    "CHARACTER",
    "CHARACTERNAME",
    "CHARISMA",
    "CHARNAME",
    "CHILD",
    "CITIES",
    "CITYNAME",
    "CLAIM",
    "CLAN",
    "COHORTS",
    "COMBAT",
    "COMMANDERS",
    "COMMENT",
    "COMPARATOR",
    "COMPARISON",
    "COMPLETION",
    "COMPOSITION",
    "CONSTRUCTION_LEADERS",
    "CONTROLLER",
    "CONTROLLER_TAG",
    "CONVERT",
    "COST",
    "COST_BREAKDOWN",
    "COSTO",
    "COST_WHY",
    "COUNT",
    "COUNTRYMODIFIER",
    "COUNTRYSIDE",
    "CRITERIA",
    "CULTUERE",
    "CULTURENAME",
    "CUR",
    "CURR",
    "CURRENT",
    "CURRENT_VALUE",
    "DAMAGE",
    "DATA",
    "DATE_MAX",
    "DATE_MIN",
    "D_BLOCK",
    "DECAY",
    "DECISIONNAME",
    "DEFECT",
    "DEFENDER",
    "DELAY",
    "DEMOTE",
    "DEPENDENCIES",
    "DESC",
    "DESC_KEY",
    "DETAILS",
    "DICE",
    "DIR",
    "DIRECTION",
    "DIR_LONG",
    "DISTANCE",
    "DISTINCTION",
    "DIV",
    "DLC",
    "DSCORE",
    "DURATION",
    "EACH",
    "EFECT",
    "EFF",
    "EFFECTIVENESS",
    "EFFECTIVE_VAL",
    "EFFECT_NAME",
    "EFFECT_PRESTIGE",
    "EFFECTS",
    "EFFICIENCY",
    "ELIGIBLE",
    "EMPLOYER",
    "ENEMY",
    "ENEMY_COUNTRY",
    "ENEMY_NAME",
    "ENEMY_STRENGTH",
    "ENEMY_UNITS",
    "ERROR",
    "ERRORS",
    "EVENT",
    "EXAMPLE",
    "EXHAUSTION",
    "EXPIRE",
    "EXPLANATION",
    "EXPORT",
    "EXP_VAL",
    "EXTRA",
    "FACTOR",
    "FAILED",
    "FAMILY_FEMALE",
    "FAMILY_MALE",
    "FEMALE_ORDER",
    "FINESSE",
    "FIRST",
    "FIRST_DETAILS",
    "FIRST_FAMILY",
    "FLAG",
    "FLANK",
    "FLAVOR",
    "FLIPPER",
    "FOG",
    "FOLDER",
    "FOR",
    "FORT",
    "FRIENDLY_UNITS",
    "FROMGOODS",
    "FROM_MODIFIERS",
    "FROMPROV",
    "GAIN",
    "GENERAL",
    "GOLD_VALUE",
    "GOOD",
    "GOOD_NAME",
    "GOODS",
    "GOVERNOR",
    "GOVERNORS",
    "GOVERNORSHIP",
    "GOVFORM",
    "GRAND",
    "GREATWORK",
    "GREEN",
    "GROUP",
    "GW_NAME",
    "HAPPINESS",
    "HEALTH",
    "HEIR",
    "HELPERS",
    "HERITAGE",
    "HIT",
    "HOLY_SITE_FALLEN",
    "HOME_PORT",
    "HOW",
    "ICON",
    "ID",
    "IDEA",
    "IMPACT",
    "INCDEC",
    "INCOMING",
    "INFLUENCE",
    "INFO",
    "INSULT1",
    "INSULT2",
    "INTERACTION",
    "INVENTION",
    "INVERSIÓN",
    "INVESTMENT",
    "INVVAL",
    "KEY",
    "KEY_LOC",
    "LAST_REASON",
    "LAW",
    "LAYER",
    "LAYOUT_NODE_ID",
    "LEFT",
    "LIEGE",
    "LIMIT",
    "LIST",
    "LOCATION",
    "LOCNAME",
    "LONGNAME",
    "LOSER",
    "LOSS",
    "MAN",
    "MARTIAL",
    "MATERIAL",
    "MAX",
    "MAXINFO",
    "MAXVALUE",
    "MEN",
    "MESSAGE",
    "MESSENGER",
    "MINUMUM",
    "MINUTES",
    "MINVALUE",
    "MISSION",
    "MOD",
    "MODEL_NAME",
    "MODIFIER",
    "MODIFIER_DESC",
    "MODIFIER_SUMMARY",
    "MODS",
    "MODULE",
    "MONARCHNAME",
    "MONARCHTITLE",
    "MONTHLY_COST",
    "MONTHLY_INCOME",
    "MONTHS",
    "MOR",
    "MOURN",
    "MUL",
    "MULT",
    "NAME",
    "NAME_FEMALE",
    "NAME_MALE",
    "NAMES",
    "NEED",
    "NEW_EFFECT",
    "NEW_NAME",
    "NEW_RANK",
    "NEXT",
    "NICKNAME",
    "NOT",
    "NUMBER",
    "OBJ",
    "OBJECT",
    "OF",
    "OFFICE",
    "OLD",
    "OLD_EFFECT",
    "OLD_NAME",
    "OLD_RANK",
    "OPERATOR",
    "OPINION",
    "OPRESSOR",
    "OPTION",
    "ORDER",
    "ORIG",
    "ORIG_IDEA",
    "OTHER",
    "OTHERRESULT",
    "OTHER_WORK",
    "OUREFFECT",
    "OUR_LOSSES",
    "OUTGOING",
    "OVERLORD",
    "OWNED",
    "OWNER",
    "OWNERSHIP",
    "OWNER_TAG",
    "PARTY",
    "PARTYADJ",
    "PARTY_INFO",
    "PARTYNAME",
    "PENALTY",
    "PERC",
    "PERCENT",
    "PERCENTAGE",
    "PERCENTAGE_LOSS",
    "PHASE",
    "PING",
    "PI_VALUE",
    "PLAYER",
    "PLAYER_MODIFIERS",
    "PLAYER_TYPE",
    "POLICY",
    "POP",
    "POP_BREAKDOWN",
    "POPS",
    "POPTYPE",
    "POPTYPENAME",
    "POPULARITY",
    "PORT_LEVEL",
    "POSITION",
    "POTENTIAL_AMOUNT",
    "POTENTIAL_VAL",
    "POWER",
    "PRESTIGE",
    "PREVENTERS",
    "PRICE",
    "PRIMARY",
    "PRIORITY",
    "PROGRESS",
    "PROMOTE",
    "PRONOUN",
    "PROV",
    "PROV_ID",
    "PROVINCENAME",
    "PROV_NAME",
    "QUEUED",
    "RAISED_AMOUNT",
    "RANGE",
    "RANK",
    "RANKTHEM",
    "RANKUS",
    "RATIO",
    "REASON",
    "REASONS",
    "REBEL",
    "RECIPIENT",
    "RECIPIENT_TITLE",
    "RED",
    "REGIMENTS",
    "REGION",
    "REGNALNUMBER",
    "REINFORCEMENTS",
    "REJECTION",
    "RELATION01",
    "RELATION02",
    "REQUIRED",
    "REQUIREMENTS",
    "RESDESC",
    "RESEARCHERS",
    "RESERVE_CENTRE",
    "RESERVE_FLANK",
    "RESPONSE",
    "RESULT",
    "REWARD",
    "RIGHT",
    "RISE",
    "ROMAN",
    "RP",
    "RULE",
    "RULER",
    "RULERTITLE",
    "SAVE_NAME",
    "SCORE",
    "SEATS",
    "SECOND",
    "SECOND_DETAILS",
    "SECOND_FAMILY",
    "SECT",
    "SEX",
    "SHIPS",
    "SHORTCUT",
    "SHORTDESC",
    "SHORT_IMPACT",
    "SHORT_TITLE",
    "SIZE",
    "SKILL",
    "SKILL_VAL",
    "SLAVE_INFO",
    "SPECIAL",
    "SPECIFIC",
    "SPEED",
    "STANCE",
    "STATE_MODIFIERS",
    "STATUS",
    "STATUS_OTHER",
    "STATUS_SHORT",
    "STAY",
    "STRENGTH",
    "STRENGTH_LOSS",
    "STRING",
    "SUBJECT_TYPE",
    "SUBUNIT_ID",
    "SUBUNIT_NAME",
    "SUPPORT",
    "TACTIC",
    "TAG",
    "TARGET",
    "TARGET_COUNTRY",
    "TARGET_DESC",
    "TARGETLIST",
    "TARGET_NAME",
    "TARGET_NUM",
    "TASK",
    "TASKS",
    "TECH",
    "TEMP",
    "TEMPLATE_ID",
    "TERMS",
    "TERRAIN",
    "TEXT",
    "THEIRLOST",
    "THEIRNUM",
    "THEIRSHIP",
    "THEM",
    "THRESHOLD",
    "TIER",
    "TIME",
    "TITLE",
    "TOFROM",
    "TOGOODS",
    "TOPROV",
    "TOTAL_POP_CONSUMPTION",
    "TRADEGOOD_KEY",
    "TRADEGOOD_NAME",
    "TRADEGOODNAME",
    "TRADITION",
    "TRAIT",
    "TREASURE",
    "TRIGGER",
    "TRUCE_COUNTRY",
    "TRUTH",
    "TYPE_DESC",
    "UNIT",
    "UNITS",
    "UNITS_TYPE",
    "UNREST",
    "UNTIL",
    "US",
    "USED",
    "USER",
    "USLOSS",
    "USNUM",
    "VAL",
    "VALUE",
    "VASSAL",
    "VASSAL_ACTION",
    "VASSALACTION",
    "VERSUS",
    "VISUAL_STRENGTH",
    "WARGOAL",
    "WARSCORE",
    "WEIGHT",
    "WG",
    "WHAT",
    "WHEN",
    "WHERE",
    "WHICH",
    "WHO",
    "WHY",
    "WIDTH",
    "WIDTH_FROM_TERRAIN",
    "WINNER",
    "WOMAN",
    "WONDER",
    "WORK",
    "X",
    "Y",
    "YEAR",
    "YEARS",
    "YOU_HAVE",
    "ZEAL",
];

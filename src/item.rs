use std::fmt::{Display, Formatter};
use strum_macros::IntoStaticStr;

/// "items" are all the things that can be looked up in string-indexed databases.
/// There is some overlap with scopes, but the difference is that scopes are runtime values
/// while items are always strings.
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoStaticStr, Hash, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum Item {
    Accessory,
    AccessoryTag,
    AccessoryVariation,
    AccessoryVariationLayout,
    AccessoryVariationTextures,
    AccoladeCategory,
    AccoladeIcon,
    AccoladeName,
    AccoladeParameter,
    AccoladeType,
    ActivityIntent,
    ActivityLocale,
    ActivityOption,
    ActivityOptionCategory,
    ActivityPhase,
    ActivityState,
    ActivityType,
    Amenity,
    ArtifactFeature,
    ArtifactFeatureGroup,
    ArtifactHistory,
    ArtifactRarity,
    ArtifactSlot,
    ArtifactSlotType,
    ArtifactTemplate,
    ArtifactType,
    ArtifactVisual,
    Asset,
    BlendShape,
    Bookmark,
    BookmarkGroup,
    BookmarkPortrait,
    Building,
    BuildingFlag,
    BuildingGfx,
    CasusBelli,
    CasusBelliGroup,
    Catalyst,
    Character,
    CharacterBackground,
    CharacterTemplate,
    ClothingGfx,
    Coa,
    CoaGfx,
    CoaColorList,
    CoaColoredEmblemList,
    CoaDynamicDefinition,
    CoaPatternList,
    CoaTemplate,
    CoaTemplateList,
    CoaTexturedEmblemList,
    CouncilPosition,
    CouncilTask,
    CourtPosition,
    CourtPositionCategory,
    CourtSceneCulture,
    CourtSceneGroup,
    CourtSceneRole,
    CourtSceneSetting,
    CourtType,
    Culture,
    CultureEra,
    CultureParameter,
    CulturePillar,
    CultureTradition,
    CustomLocalization,
    DangerType,
    DeathReason,
    Decision,
    Define,
    DiarchyMandate,
    DiarchyParameter,
    DiarchyType,
    Dlc,
    DlcFeature,
    Dna,
    Doctrine,
    DoctrineParameter,
    Dynasty,
    DynastyLegacy,
    DynastyPerk,
    EffectLocalization,
    Entity,
    Environment,
    Ethnicity,
    Event,
    EventBackground,
    EventNamespace,
    EventTheme,
    EventTransition,
    Faction,
    Faith,
    FaithIcon,
    File,
    Focus,
    GameConcept,
    GameRule,
    GameRuleSetting,
    GeneAgePreset,
    GeneAttribute,
    GeneCategory,
    GeneticConstraint,
    GovernmentType,
    GovernmentFlag,
    GraphicalFaith,
    GuestInviteRule,
    GuestSubset,
    Holding,
    HoldingFlag,
    HolySite,
    HolySiteFlag,
    Hook,
    House,
    ImportantAction,
    Innovation,
    InnovationFlag,
    Inspiration,
    Interaction,
    InteractionCategory,
    Language,
    Law,
    LawFlag,
    Lifestyle,
    Localization,
    MapMode,
    MemoryCategory,
    MemoryType,
    MenAtArms,
    MenAtArmsBase,
    Modifier,
    ModifierFormat,
    Music,
    NamedColor,
    NameList,
    Nickname,
    OnAction,
    OpinionModifier,
    Pdxmesh,
    Perk,
    PerkTree,
    PointOfInterest,
    PoolSelector,
    PortraitAnimation,
    PortraitCamera,
    PortraitModifierGroup,
    PortraitModifierPack,
    PrisonType,
    Province,
    PulseAction,
    Region,
    Relation,
    RelationFlag,
    Religion,
    ReligionFamily,
    RewardItem,
    Scheme,
    ScriptedAnimation,
    ScriptedEffect,
    ScriptedGui,
    ScriptedList,
    ScriptedModifier,
    ScriptedRule,
    ScriptedTrigger,
    ScriptValue,
    Secret,
    Sexuality,
    Skill,
    Sound,
    SpecialBuilding,
    SpecialGuest,
    Story,
    Struggle,
    StrugglePhase,
    StrugglePhaseParameter,
    Suggestion,
    Terrain,
    TextureFile,
    Title,
    TitleHistory,
    TitleHistoryType,
    TitleLaw,
    TitleLawFlag,
    Trait,
    TraitCategory,
    TraitFlag,
    TraitTrack,
    TravelOption,
    TravelPlanModifier,
    TriggerLocalization,
    UnitGfx,
    VassalContract,
    VassalContractFlag,
    VassalObligationLevel,
    VassalStance,
}

use crate::item::Item::*;

impl Item {
    pub fn path(self) -> &'static str {
        #[allow(clippy::match_same_arms)]
        match self {
            Accessory => "gfx/portraits/accessories/",
            AccessoryTag => "gfx/portraits/accessories/",
            AccessoryVariation => "gfx/portraits/accessory_variations/",
            AccessoryVariationLayout => "gfx/portraits/accessory_variations/",
            AccessoryVariationTextures => "gfx/portraits/accessory_variations/",
            AccoladeCategory => "common/accolade_types/",
            AccoladeIcon => "common/accolade_icons/",
            AccoladeName => "common/accolade_names/",
            AccoladeParameter => "common/accolade_types/",
            AccoladeType => "common/accolade_types/",
            ActivityIntent => "common/activities/intents/",
            ActivityLocale => "common/activities/activity_locales/",
            ActivityOption => "common/activities/activity_types/",
            ActivityOptionCategory => "common/activities/activity_types/",
            ActivityPhase => "common/activities/activity_types/",
            ActivityState => "",
            ActivityType => "common/activities/activity_types/",
            Amenity => "common/court_amenities/",
            ArtifactFeature => "common/artifacts/features/",
            ArtifactFeatureGroup => "common/artifacts/feature_groups/",
            ArtifactHistory => "",
            ArtifactRarity => "",
            ArtifactSlot => "common/artifacts/slots/",
            ArtifactSlotType => "common/artifacts/slots/",
            ArtifactTemplate => "common/artifacts/templates/",
            ArtifactType => "common/artifacts/types/",
            ArtifactVisual => "common/artifacts/visuals/",
            Asset => "gfx/models/",
            BlendShape => "gfx/models/",
            Bookmark => "common/bookmarks/bookmarks/",
            BookmarkGroup => "common/bookmarks/groups/",
            BookmarkPortrait => "common/bookmark_portraits/",
            Building => "common/buildings/",
            BuildingFlag => "common/buildings/",
            BuildingGfx => "common/culture/cultures/",
            CasusBelli => "common/casus_belli_types/",
            CasusBelliGroup => "common/casus_belli_groups/",
            Catalyst => "common/struggle/catalysts/",
            Character => "history/characters/",
            CharacterBackground => "common/character_backgrounds/",
            CharacterTemplate => "common/scripted_character_templates/",
            ClothingGfx => "common/culture/cultures/",
            Coa => "common/coat_of_arms/coat_of_arms/",
            CoaGfx => "common/culture/cultures/",
            CoaColorList => "common/coat_of_arms/template_lists/",
            CoaColoredEmblemList => "common/coat_of_arms/template_lists/",
            CoaDynamicDefinition => "common/coat_of_arms/dynamic_definitions/",
            CoaPatternList => "common/coat_of_arms/template_lists/",
            CoaTemplate => "common/coat_of_arms/coat_of_arms/",
            CoaTemplateList => "common/coat_of_arms/template_lists/",
            CoaTexturedEmblemList => "common/coat_of_arms/template_lists/",
            CouncilPosition => "common/council_positions/",
            CouncilTask => "common/council_tasks/",
            CourtPosition => "common/court_positions/types/",
            CourtPositionCategory => "common/court_positions/categories/",
            CourtSceneCulture => "gfx/court_scene/scene_cultures/",
            CourtSceneGroup => "gfx/court_scene/character_groups/",
            CourtSceneRole => "gfx/court_scene/character_roles/",
            CourtSceneSetting => "gfx/court_scene/scene_settings/",
            CourtType => "common/court_types/",
            Culture => "common/culture/cultures/",
            CultureEra => "common/culture/eras/",
            CultureParameter => "common/culture/",
            CulturePillar => "common/culture/pillars/",
            CultureTradition => "common/culture/traditions/",
            CustomLocalization => "common/customizable_localization/",
            DangerType => "",
            DeathReason => "common/deathreasons/",
            Decision => "common/decisions/",
            Define => "common/defines/",
            DiarchyMandate => "common/diarchies/diarchy_mandates/",
            DiarchyParameter => "common/diarchies/diarchy_types/",
            DiarchyType => "common/diarchies/diarchy_types/",
            Dlc => "",
            DlcFeature => "",
            Dna => "common/dna_data/",
            Doctrine => "common/religion/doctrines/",
            DoctrineParameter => "common/religion/doctrines/",
            Dynasty => "common/dynasties/",
            DynastyLegacy => "common/dynasty_legacies/",
            DynastyPerk => "common/dynasty_perks/",
            EffectLocalization => "common/effect_localization/",
            Ethnicity => "common/ethnicities/",
            Entity => "gfx/models/",
            Environment => "gfx/portraits/environments/",
            Event => "events/",
            EventBackground => "common/event_backgrounds/",
            EventNamespace => "events/",
            EventTheme => "common/event_themes/",
            EventTransition => "common/event_transitions/",
            Faith => "common/religion/religions/",
            FaithIcon => "common/religion/religions/",
            Faction => "common/factions/",
            File => "",
            Focus => "common/focuses/",
            GameConcept => "common/game_concepts/",
            GameRule => "common/game_rules/",
            GameRuleSetting => "common/game_rules/",
            GeneAgePreset => "common/genes/",
            GeneAttribute => "common/genes/",
            GeneCategory => "common/genes/",
            GeneticConstraint => "common/traits/",
            GovernmentType => "common/governments/",
            GovernmentFlag => "common/governments/",
            GraphicalFaith => "common/religion/religions/",
            GuestInviteRule => "common/activities/guest_invite_rules/",
            GuestSubset => "common/activities/activity_types/",
            Holding => "common/holdings/",
            HoldingFlag => "common/holdings/",
            HolySite => "common/religion/holy_sites/",
            HolySiteFlag => "common/religion/holy_sites/",
            Hook => "common/hook_types/",
            House => "common/dynasty_houses/",
            ImportantAction => "common/important_actions/",
            Innovation => "common/culture/innovations/",
            InnovationFlag => "common/culture/innovations/",
            Inspiration => "common/inspirations/",
            Interaction => "common/character_interactions/",
            InteractionCategory => "common/character_interaction_categories/",
            Language => "common/culture/pillars/",
            Law => "common/laws/",
            LawFlag => "common/laws/",
            Lifestyle => "common/lifestyles/",
            Localization => "localization/",
            MapMode => "gfx/map/map_modes/",
            MemoryCategory => "common/character_memory_types/",
            MemoryType => "common/character_memory_types/",
            MenAtArms => "common/men_at_arms_types/",
            MenAtArmsBase => "common/men_at_arms_types/",
            Modifier => "common/modifiers/",
            ModifierFormat => "common/modifier_definition_formats/",
            Music => "music/",
            NamedColor => "common/named_colors/",
            NameList => "common/culture/name_lists/",
            Nickname => "common/nicknames/",
            OnAction => "common/on_action/",
            OpinionModifier => "common/opinion_modifiers/",
            Pdxmesh => "gfx/models/",
            Perk => "common/lifestyle_perks/",
            PerkTree => "common/lifestyle_perks/",
            PointOfInterest => "common/travel/point_of_interest_types/",
            PoolSelector => "common/pool_character_selectors/",
            PortraitAnimation => "gfx/portraits/portrait_animations/",
            PortraitCamera => "gfx/portraits/cameras/",
            PortraitModifierGroup => "gfx/portraits/portrait_modifiers/",
            PortraitModifierPack => "gfx/portraits/portrait_animations/",
            PrisonType => "",
            Province => "map_data/definition.csv",
            PulseAction => "common/activities/pulse_actions/",
            Region => "map_data/geographical_regions/",
            Relation => "common/scripted_relations/",
            RelationFlag => "common/scripted_relations/",
            Religion => "common/religion/religions/",
            ReligionFamily => "common/religion/religion_families/",
            RewardItem => "",
            Scheme => "common/schemes/",
            ScriptedAnimation => "common/scripted_animations/",
            ScriptedEffect => "common/scripted_effects/",
            ScriptedGui => "common/scripted_guis/",
            ScriptedList => "common/scripted_lists/",
            ScriptedModifier => "common/scripted_modifiers/",
            ScriptedRule => "common/scripted_rules/",
            ScriptedTrigger => "common/scripted_triggers/",
            ScriptValue => "common/script_values/",
            Secret => "common/secret_types/",
            Sexuality => "",
            Skill => "",
            Sound => "sound/GUIDs.txt",
            SpecialBuilding => "common/buildings/",
            SpecialGuest => "common/activities/activity_types/",
            Story => "common/story_cycle/",
            Struggle => "common/struggle/struggles/",
            StrugglePhase => "common/struggle/struggles/",
            StrugglePhaseParameter => "common/struggle/struggles/",
            Suggestion => "common/suggestions/",
            Terrain => "common/terrain_types/",
            TextureFile => "gfx/models/",
            Title => "common/landed_titles/",
            TitleHistory => "history/titles/",
            TitleHistoryType => "",
            TitleLaw => "common/laws/",
            TitleLawFlag => "common/laws/",
            Trait => "common/traits/",
            TraitCategory => "",
            TraitFlag => "common/traits/",
            TraitTrack => "common/traits/",
            TravelOption => "common/travel/travel_options/",
            TravelPlanModifier => "",
            TriggerLocalization => "common/trigger_localization/",
            UnitGfx => "common/culture/cultures/",
            VassalContract => "common/vassal_contracts/",
            VassalContractFlag => "common/vassal_contracts/",
            VassalObligationLevel => "common/vassal_contracts/",
            VassalStance => "common/vassal_stances/",
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let s: &'static str = self.into();
        write!(f, "{}", s.replace('_', " "))
    }
}

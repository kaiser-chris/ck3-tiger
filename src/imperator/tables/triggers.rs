use once_cell::sync::Lazy;

use crate::everything::Everything;
use crate::helpers::TigerHashMap;
use crate::item::Item;
use crate::report::{warn, ErrorKey};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::trigger::Trigger;

use Trigger::*;

pub fn scope_trigger(name: &Token, data: &Everything) -> Option<(Scopes, Trigger)> {
    let name_lc = name.as_str().to_ascii_lowercase();

    // TODO: binary search might be faster
    if let Some((from, trigger)) = TRIGGER_MAP.get(&*name_lc) {
        return Some((*from, *trigger));
    }
    if let Some(conviction) = name_lc.strip_suffix("_conviction") {
        data.verify_exists_implied(Item::PartyType, conviction, name);
        return Some((Scopes::Character, Trigger::CompareValue));
    }
    if let Some(support) = name_lc.strip_suffix("_support") {
        data.verify_exists_implied(Item::PartyType, support, name);
        return Some((Scopes::Country, Trigger::CompareValue));
    }
    if let Some(happiness) = name_lc.strip_suffix("_happiness") {
        data.verify_exists_implied(Item::PopType, happiness, name);
        return Some((Scopes::Province, Trigger::CompareValue));
    }
    if let Some(part) = name.as_str().strip_prefix("num_of_") {
        if data.item_exists(Item::Building, part) {
            return Some((Scopes::Province, Trigger::CompareValue));
        }
        if data.item_exists(Item::PopType, part) {
            return Some((Scopes::Province.union(Scopes::Country), Trigger::CompareValue));
        }
        if !data.item_exists(Item::Building, part) && !data.item_exists(Item::PopType, part) {
            let msg = format!("could not find any {part}");
            let info = "Possible valid options would be: num_of_$POPTYPE$ or num_of_$BUILDING$";
            warn(ErrorKey::MissingItem).msg(msg).info(info).loc(name).push();
        }
    }
    // This one is weird...the trigger is just Item::TechnologyTable with no suffix or prefix.
    if data.item_exists(Item::TechnologyTable, name.as_str()) {
        return Some((Scopes::Country, Trigger::CompareValue));
    }
    None
}

static TRIGGER_MAP: Lazy<TigerHashMap<&'static str, (Scopes, Trigger)>> = Lazy::new(|| {
    let mut hash = TigerHashMap::default();
    for (from, s, trigger) in TRIGGER {
        hash.insert(*s, (*from, *trigger));
    }
    hash
});

/// LAST UPDATED IMPERATOR VERSION 2.0.4
/// See `triggers.log` from the game data dumps
/// A key ends with '(' if it is the version that takes a parenthesized argument in script.
const TRIGGER: &[(Scopes, &str, Trigger)] = &[
    (
        Scopes::State,
        "can_create_trade_route",
        Block(&[("target", Scope(Scopes::State)), ("goods", Item(Item::TradeGood))]),
    ),
    (Scopes::State, "has_capital_bonus_for_trade_good", Item(Item::TradeGood)),
    (Scopes::State, "has_capital_surplus", Boolean),
    (Scopes::State, "is_automated_trading", Boolean),
    (Scopes::Province, "distance_from", ScopeOrItem(Scopes::Province, Item::Province)),
    (
        Scopes::Country,
        "can_unlock_invention",
        Block(&[
            ("invention", Item(Item::Invention)),
            ("free", Boolean),
            ("ignore_dependencies", Boolean),
        ]),
    ),
    (Scopes::None, "debug_log", UncheckedValue),
    (Scopes::None, "debug_log_details", UncheckedValue),
    // State Triggers
    (Scopes::State, "can_change_governor_policy", Boolean),
    (Scopes::State, "can_import_trade_good", Boolean),
    (Scopes::State, "has_any_great_work_state", Boolean),
    (Scopes::State, "has_governor", Boolean),
    (Scopes::State, "has_state_food", CompareValue),
    (Scopes::State, "has_state_food_capacity", CompareValue),
    (Scopes::State, "has_state_modifier", Item(Item::Modifier)),
    (Scopes::State, "incoming_trade_routes", CompareValue),
    (Scopes::State, "is_capital_state", Boolean),
    (Scopes::State, "outgoing_trade_routes", CompareValue),
    (Scopes::State, "state_commerce_income", CompareValue),
    (Scopes::State, "state_level_loyalty", CompareValue),
    (Scopes::State, "state_monthly_food_income", CompareValue),
    (
        Scopes::State,
        "trade_good_exports",
        Block(&[("target", Item(Item::TradeGood)), ("value", CompareValue)]),
    ),
    (
        Scopes::State,
        "trade_good_imports",
        Block(&[("target", Item(Item::TradeGood)), ("value", CompareValue)]),
    ),
    (
        Scopes::State,
        "trade_good_surplus",
        Block(&[("target", Item(Item::TradeGood)), ("value", CompareValue)]),
    ),
    (Scopes::State, "trade_routes", CompareValue),
    (Scopes::State, "unused_trade_routes", CompareValue),
    // Unit Triggers
    (Scopes::Unit, "days_since_last_unit_victory", CompareValue),
    (Scopes::Unit, "experience_percentage", CompareValue),
    (Scopes::Unit, "food_percentage", CompareValue),
    (Scopes::Unit, "has_reduced_roadbuilding_cost", Boolean),
    (Scopes::Unit, "has_commander", Boolean),
    (Scopes::Unit, "has_siege_impact", Boolean),
    (Scopes::Unit, "has_unit_modifier", Item(Item::Modifier)),
    (Scopes::Unit, "in_combat", Boolean),
    (Scopes::Unit, "in_siege", Boolean),
    (Scopes::Unit, "is_army", Boolean),
    (Scopes::Unit, "is_carrying_troops", Boolean),
    (Scopes::Unit, "in_siege", Boolean),
    (Scopes::Unit, "is_dominant_unit", Boolean),
    (Scopes::Unit, "is_exiled", Boolean),
    (Scopes::Unit, "is_idle", Boolean),
    (Scopes::Unit, "is_moving", Boolean),
    (Scopes::Unit, "is_navy", Boolean),
    (Scopes::Unit, "is_unit_ability_used", Item(Item::UnitAbility)),
    (Scopes::Unit, "is_unit_locked", Boolean),
    (Scopes::Unit, "morale_percentage", CompareValue),
    (Scopes::Unit, "num_of_loyal_cohorts", CompareValue),
    (Scopes::Unit, "num_of_migrants", CompareValue),
    (Scopes::Unit, "strength_percentage", CompareValue),
    (Scopes::Unit, "unit_size", CompareValue),
    (Scopes::Unit, "unit_size_rank", CompareValue),
    (Scopes::Unit, "unit_size_rank_percentage", CompareValue),
    (Scopes::Unit, "unit_threat", CompareValue),
    // Great Work Triggers
    (Scopes::GreatWork, "great_work_any_material", Item(Item::GreatWorkMaterial)),
    (Scopes::GreatWork, "great_work_builder", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::GreatWork, "great_work_category", Item(Item::GreatWorkCategory)),
    (Scopes::GreatWork, "great_work_min_effect_tier", CompareValue),
    (Scopes::GreatWork, "great_work_only_material", Item(Item::GreatWorkMaterial)),
    (Scopes::GreatWork, "is_ancient_wonder", Boolean),
    // Character Triggers
    (Scopes::Character, "age", CompareValue),
    (Scopes::Character, "can_add_entire_loyalty_bonus", Item(Item::Loyalty)),
    (Scopes::Character, "can_get_friends", Boolean),
    (Scopes::Character, "can_get_rivals", Boolean),
    (Scopes::Character, "can_hold_office", Item(Item::Office)),
    (Scopes::Character, "character_experience", CompareValue),
    (Scopes::Character, "charisma", CompareValue),
    (Scopes::Character, "corruption", CompareValue),
    (Scopes::Character, "current_party_conviction", CompareValue),
    (Scopes::Character, "days_since_last_victory", CompareValue),
    (Scopes::Character, "fertility", CompareValue),
    (Scopes::Character, "finesse", CompareValue),
    (Scopes::Character, "from_ruler_family", Boolean),
    (Scopes::Character, "has_ambition", Item(Item::Ambition)),
    (Scopes::Character, "has_any_office", Boolean),
    (Scopes::Character, "has_character_modifier", Item(Item::Modifier)),
    (Scopes::Character, "has_culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (Scopes::Character, "has_culture_group", ScopeOrItem(Scopes::CultureGroup, Item::CultureGroup)),
    (Scopes::Character, "has_father", Boolean),
    (Scopes::Character, "has_holding_in", ScopeOrItem(Scopes::Province, Item::Province)),
    (Scopes::Character, "has_job", Boolean),
    (Scopes::Character, "has_loyalty", Item(Item::Loyalty)),
    (Scopes::Character, "has_mother", Boolean),
    (Scopes::Character, "has_nickname", Boolean),
    (Scopes::Character, "has_office", Item(Item::Office)),
    (Scopes::Character, "has_religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
    (Scopes::Character, "has_same_culture_group_as", Scope(Scopes::Character)),
    (Scopes::Character, "has_same_family", Scope(Scopes::Character)),
    (Scopes::Character, "has_same_religion_as", Scope(Scopes::Character)),
    (Scopes::Character, "has_tech_office", Boolean),
    (Scopes::Character, "has_tech_office_of", Item(Item::TechnologyTable)),
    (Scopes::Character, "has_trait", Item(Item::CharacterTrait)),
    (Scopes::Character, "has_triggered_character_modifier", Item(Item::Modifier)),
    (Scopes::Character, "health", CompareValue),
    (Scopes::Character, "highest_skill", Choice(&["martial", "finesse", "charisma", "zeal"])),
    (Scopes::Character, "in_command", Boolean),
    (Scopes::Character, "is_admiral", Boolean),
    (Scopes::Character, "is_adult", Boolean),
    (Scopes::Character, "is_alive", Boolean),
    (Scopes::Character, "is_at_location", ScopeOrItem(Scopes::Province, Item::Province)),
    (Scopes::Character, "is_at_same_location", Scope(Scopes::Character)),
    (Scopes::Character, "is_banished", Boolean),
    (Scopes::Character, "is_bastard", Boolean),
    (Scopes::Character, "is_child_of", Scope(Scopes::Character)),
    (Scopes::Character, "is_clan_chief", Boolean),
    (Scopes::Character, "is_close_relative", Scope(Scopes::Character)),
    (Scopes::Character, "is_co_ruler", Boolean),
    (Scopes::Character, "is_courtier", Boolean),
    (Scopes::Character, "is_deified", Boolean),
    (Scopes::Character, "is_female", Boolean),
    (Scopes::Character, "is_friend", Scope(Scopes::Character)),
    (Scopes::Character, "is_general", Boolean),
    (Scopes::Character, "is_governor", Boolean),
    (Scopes::Character, "is_head_of_family", Boolean),
    (Scopes::Character, "is_leader_of_party", Item(Item::PartyType)),
    (Scopes::Character, "is_leader_of_party_type", Item(Item::PartyType)),
    (Scopes::Character, "is_male", Boolean),
    (Scopes::Character, "is_married", Boolean),
    (Scopes::Character, "is_mercenary", Boolean),
    (Scopes::Character, "is_minor_character", Boolean),
    (Scopes::Character, "is_parent_of", Scope(Scopes::Character)),
    (Scopes::Character, "is_party_leader", Boolean),
    (Scopes::Character, "is_pregnant", Boolean),
    (Scopes::Character, "is_pretender", Boolean),
    (Scopes::Character, "is_previous_ruler", Boolean),
    (Scopes::Character, "is_primary_heir", Boolean),
    (Scopes::Character, "is_rival", Scope(Scopes::Character)),
    (Scopes::Character, "is_ruler", Boolean),
    (Scopes::Character, "is_same_gender", Scope(Scopes::Character)),
    (Scopes::Character, "is_same_party_as", Scope(Scopes::Character)),
    (Scopes::Character, "is_sibling_of", Scope(Scopes::Character)),
    (Scopes::Character, "is_spouse_of", Scope(Scopes::Character)),
    (Scopes::Character, "is_successor", Boolean),
    (Scopes::Character, "loyalty", CompareValue),
    (Scopes::Character, "martial", CompareValue),
    (Scopes::Character, "num_character_treasures", CompareValue),
    (Scopes::Character, "num_holdings_owned", CompareValue),
    (Scopes::Character, "num_loyal_cohorts", CompareValue),
    (Scopes::Character, "num_loyal_veterans", CompareValue),
    (Scopes::Character, "num_of_children", CompareValue),
    (Scopes::Character, "num_of_friends", CompareValue),
    (Scopes::Character, "num_of_rivals", CompareValue),
    (Scopes::Character, "num_of_supporters", CompareValue),
    (Scopes::Character, "number_of_health_traits", CompareValue),
    (Scopes::Character, "number_of_military_traits", CompareValue),
    (Scopes::Character, "number_of_personality_traits", CompareValue),
    (Scopes::Character, "number_of_status_traits", CompareValue),
    (Scopes::Character, "number_of_traits", CompareValue),
    (Scopes::Character, "total_power_base", CompareValue),
    (Scopes::Character, "party", Scope(Scopes::Party)),
    (Scopes::Character, "party_type", Item(Item::PartyType)),
    (Scopes::Character, "popularity", CompareValue),
    (Scopes::Character, "power_base", CompareValue),
    (Scopes::Character, "prisoner", Boolean),
    (Scopes::Character, "prominence", CompareValue),
    (Scopes::Character, "relative_power_base", CompareValue),
    (Scopes::Character, "title_importance", CompareValue),
    (Scopes::Character, "wealth", CompareValue),
    (Scopes::Character, "zeal", CompareValue),
    (Scopes::Character, "has_same_culture_as", Scope(Scopes::Character)),
    // Pop Triggers
    (Scopes::Pop, "pop_can_move", Boolean),
    (Scopes::Pop, "pop_culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (Scopes::Pop, "pop_culture_group", ScopeOrItem(Scopes::CultureGroup, Item::CultureGroup)),
    (Scopes::Pop, "pop_hapiness", CompareValue),
    (Scopes::Pop, "pop_religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
    (Scopes::Pop, "pop_type", Item(Item::PopType)),
    (Scopes::Pop, "is_pop_type_right", Item(Item::PopType)),
    // Country Triggers
    (
        Scopes::Country.union(Scopes::Character).union(Scopes::Province),
        "treasure_count",
        CompareValue,
    ),
    (Scopes::Country, "biggest_party", ScopeOrItem(Scopes::Party, Item::PartyType)),
    (Scopes::Country, "alliance_with", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "can_activate", Scope(Scopes::Deity)),
    (Scopes::Country, "can_change_idea", Item(Item::Idea)),
    (Scopes::Country, "can_pay_price", Item(Item::Price)),
    (Scopes::Country, "centralization", CompareValue),
    (Scopes::Country, "civil_war_with", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "country_culture_group", Item(Item::CultureGroup)),
    (Scopes::Country, "country_population", CompareValue),
    (
        Scopes::Country,
        "country_trade_good_exports",
        Block(&[("target", Item(Item::TradeGood)), ("value", CompareValue)]),
    ),
    (
        Scopes::Country,
        "country_trade_good_imports",
        Block(&[("target", Item(Item::TradeGood)), ("value", CompareValue)]),
    ),
    (Scopes::Country, "cultural_unity", CompareValue),
    (Scopes::Country, "days_since_last_war", CompareValue),
    (Scopes::Country, "diplomatic_stance", Item(Item::DiplomaticStance)),
    (Scopes::Country, "distress_level", CompareValue),
    (Scopes::Country, "exports_to", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "gender_equality", Boolean),
    (Scopes::Country, "government", Item(Item::GovernmentType)),
    (Scopes::Country, "has_aggressive_expansion", CompareValue),
    (Scopes::Country, "has_any_great_work_country", Boolean),
    (Scopes::Country, "has_any_omen", Boolean),
    (Scopes::Country, "has_civil_war", Boolean),
    (Scopes::Country, "has_co_ruler_government", Boolean),
    (Scopes::Country, "has_coasts", Boolean),
    (Scopes::Country, "has_completed_mission", Item(Item::Mission)),
    (Scopes::Country, "has_completed_mission_task", Item(Item::MissionTask)),
    (Scopes::Country, "has_country_great_work_effect", Item(Item::GreatWorkEffect)),
    (Scopes::Country, "has_country_modifier", Item(Item::Modifier)),
    (Scopes::Country, "has_deity_in_pantheon", Scope(Scopes::Deity)),
    (Scopes::Country, "has_high_economic_policy", Item(Item::EconomicPolicy)),
    (Scopes::Country, "has_land", Boolean),
    (Scopes::Country, "has_law", Item(Item::Law)),
    (Scopes::Country, "has_low_economic_policy", Item(Item::EconomicPolicy)),
    (Scopes::Country, "has_mid_economic_policy", Item(Item::EconomicPolicy)),
    (Scopes::Country, "has_military_access", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "has_military_bonus", Item(Item::MilitaryTradition)),
    (Scopes::Country, "has_monthly_balance", CompareValue),
    (Scopes::Country, "has_monthly_income", CompareValue),
    (
        Scopes::Country,
        "has_opinion",
        Block(&[
            ("modifier", Item(Item::Opinion)),
            ("target", ScopeOrItem(Scopes::Country, Item::Localization)),
        ]),
    ),
    (Scopes::Country, "has_party_type", Item(Item::PartyType)),
    (Scopes::Country, "has_primary_heir", Boolean),
    (Scopes::Country, "has_senate_approval", CompareValue),
    (Scopes::Country, "has_subject_loyalty", CompareValue),
    (Scopes::Country, "has_this_omen", ScopeOrItem(Scopes::Deity, Item::Deity)),
    (Scopes::Country, "has_truce_with", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "has_war_exhaustion", CompareValue),
    (Scopes::Country, "has_subject_loyalty", CompareValue),
    (Scopes::Country, "healthy_economy_percentage", CompareValue),
    (Scopes::Country, "heritage", Item(Item::Heritage)),
    (Scopes::Country, "idea", Item(Item::Idea)),
    (Scopes::Country, "imports_from", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "in_diplomatic_range", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "invention", Item(Item::Invention)),
    (Scopes::Country, "is_ai", Boolean),
    (Scopes::Country, "is_antagonist", Boolean),
    (Scopes::Country, "is_guaranteed_by", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "is_monarchy", Boolean),
    (Scopes::Country, "is_monotheist_deity", Scope(Scopes::Deity)),
    (Scopes::Country, "is_republic", Boolean),
    (Scopes::Country, "is_subject", Boolean),
    (Scopes::Country, "is_overlord", Boolean),
    (Scopes::Country, "is_subject_of", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "is_subject_type", Item(Item::SubjectType)),
    (Scopes::Country, "is_tradition_tree_allowed", Item(Item::MilitaryTraditionTree)),
    (Scopes::Country, "is_tribal", Boolean),
    (Scopes::Country, "is_tutorial_active", Boolean),
    (Scopes::Country, "legitimacy", CompareValue),
    (Scopes::Country, "manpower", CompareValue),
    (Scopes::Country, "manpower_percentage", CompareValue),
    (Scopes::Country, "max_diplomatic_relations", CompareValue),
    (Scopes::Country, "max_manpower", CompareValue),
    (Scopes::Country, "migration_strategy", CompareValue),
    (Scopes::Country, "military_experience", CompareValue),
    (Scopes::Country, "months_to_war", CompareValue),
    (Scopes::Country, "naval_dominance", CompareValue),
    (Scopes::Country, "non_loyal_power_base", CompareValue),
    (Scopes::Country, "num_active_relations", CompareValue),
    (Scopes::Country, "num_allowed_families", CompareValue),
    (Scopes::Country, "num_of_cities", CompareValue),
    (Scopes::Country, "num_of_civic_ideas", CompareValue),
    (Scopes::Country, "num_of_cohorts", CompareValue),
    (Scopes::Country, "num_of_controlled_cities", CompareValue),
    (Scopes::Country, "num_of_deified_rulers_in_pantheon", CompareValue),
    (Scopes::Country, "num_of_families", CompareValue),
    (Scopes::Country, "num_of_military_ideas", CompareValue),
    (Scopes::Country, "num_of_oratory_ideas", CompareValue),
    (Scopes::Country, "num_of_ports", CompareValue),
    (Scopes::Country, "num_of_provinces", CompareValue),
    (Scopes::Country, "num_of_religious_ideas", CompareValue),
    (Scopes::Country, "num_of_ships", CompareValue),
    (
        Scopes::Country,
        "num_of_unit_type",
        Block(&[("type", Item(Item::Unit)), ("value", CompareValue)]),
    ),
    (Scopes::Country, "office_is_empty", Item(Item::Office)),
    (
        Scopes::Country,
        "opinion",
        Block(&[
            ("target", ScopeOrItem(Scopes::Country, Item::Localization)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::Country, "owns", ScopeOrItem(Scopes::Province, Item::Province)),
    (Scopes::Country, "owns_area", ScopeOrItem(Scopes::Area, Item::Area)),
    (Scopes::Country, "owns_or_subject_owns", ScopeOrItem(Scopes::Province, Item::Province)),
    (Scopes::Country, "owns_or_subject_owns_area", ScopeOrItem(Scopes::Area, Item::Area)),
    (Scopes::Country, "owns_or_subject_owns_region", ScopeOrItem(Scopes::Region, Item::Region)),
    (Scopes::Country, "owns_region", ScopeOrItem(Scopes::Region, Item::Region)),
    (Scopes::Country, "percentage_characters_below_max_loyalty", CompareValue),
    (Scopes::Country, "political_influence", CompareValue),
    (Scopes::Country, "possible_holdings", CompareValue),
    (Scopes::Country, "primary_culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (Scopes::Country, "rank", CompareValue),
    (Scopes::Country, "religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
    (Scopes::Country, "religious_unity", CompareValue),
    (Scopes::Country, "safety", CompareValue),
    (Scopes::Country, "stability", CompareValue),
    (Scopes::Country, "tag", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Country, "threat_in_owned_land", CompareValue),
    (Scopes::Country, "total_holdings", CompareValue),
    (Scopes::Country, "towards_civil_war", Boolean),
    (Scopes::Country, "treasury", CompareValue),
    (Scopes::Country, "tyranny", CompareValue),
    (Scopes::Country, "war", Boolean),
    (Scopes::Country, "war_with", ScopeOrItem(Scopes::Country, Item::Localization)),
    (
        Scopes::Country,
        "culture_pops_in_country",
        Block(&[("target", Scope(Scopes::CountryCulture)), ("value", CompareValue)]),
    ),
    (Scopes::Country, "is_monotheist_religion", Boolean),
    // Subunit triggers
    (Scopes::SubUnit, "cohort_food_consumption", CompareValue),
    (Scopes::SubUnit, "cohort_food_storage_capacity", CompareValue),
    (Scopes::SubUnit, "has_personal_loyalty", Boolean),
    (Scopes::SubUnit, "is_cohort", Boolean),
    (Scopes::SubUnit, "is_migration", Boolean),
    (Scopes::SubUnit, "is_ship", Boolean),
    (Scopes::SubUnit, "ship_category", Choice(&["light", "medium", "heavy"])),
    (Scopes::SubUnit, "sub_unit_type", Item(Item::Unit)),
    (Scopes::SubUnit, "subunit_morale_percentage", CompareValue),
    (Scopes::SubUnit, "subunit_strength_percentage", CompareValue),
    // Unit triggers
    (
        Scopes::Unit.union(Scopes::Character).union(Scopes::Governorship),
        "has_legion_trigger",
        Boolean,
    ),
    // Party triggers
    (Scopes::Party, "has_active_agenda", Boolean),
    (Scopes::Party, "has_power_percentage", CompareValue),
    (Scopes::Party, "is_party_type", Item(Item::PartyType)),
    (Scopes::Party, "party_approval", CompareValue),
    // Family triggers
    (Scopes::Family, "is_grateful", Boolean),
    (Scopes::Family, "is_scorned", Boolean),
    (Scopes::Family, "num_of_expected_jobs", CompareValue),
    (Scopes::Family, "num_of_jobs", CompareValue),
    (Scopes::Family, "num_of_members", CompareValue),
    (Scopes::Family, "prestige", CompareValue),
    // Legion triggers
    (Scopes::Legion, "can_add_commander", Boolean),
    (Scopes::Legion, "commander_count", CompareValue),
    (Scopes::Legion, "distinction_count", CompareValue),
    (Scopes::Legion, "has_distinction", Item(Item::LegionDistinction)),
    (Scopes::Legion, "unit_count", CompareValue),
    // Siege triggers
    (Scopes::Siege, "has_breach", Boolean),
    // War triggers
    (Scopes::War, "is_civil_war", Boolean),
    (Scopes::War, "is_war_leader", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::War, "is_war_over", Boolean),
    (
        Scopes::War,
        "war_score",
        Block(&[
            ("target", ScopeOrItem(Scopes::Country, Item::Localization)),
            ("value", CompareValue),
        ]),
    ),
    // Deity triggers
    (Scopes::Deity, "deity_religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
    (Scopes::Deity, "has_holy_site", Boolean),
    // TODO - These 2 should be country modifiers from ModifKinds::Country
    (Scopes::Deity, "has_active_modifier", UncheckedValue),
    (Scopes::Deity, "has_passive_modifier", UncheckedValue),
    // Province triggers
    (Scopes::Province, "ai_wants_road", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "can_build_building", Item(Item::Building)),
    (Scopes::Province, "can_have_port", Boolean),
    // TODO - This should be "Special" and only be usable in files inside common/unit_abilities
    (Scopes::Province, "can_use_unit_ability", Item(Item::UnitAbility)),
    (Scopes::Province, "civilization_value", CompareValue),
    (Scopes::Province, "control_range", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "distance_to_migration_target", CompareValue),
    (Scopes::Province, "dominant_province_culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (Scopes::Province, "dominant_province_religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
    (Scopes::Province, "fort_level", CompareValue),
    (Scopes::Province, "free_building_slots", CompareValue),
    (Scopes::Province, "governor_policy", Item(Item::GovernorPolicy)),
    (
        Scopes::Province,
        "great_work_locator_is_free",
        Choice(&["primary_great_work", "secondary_great_work", "great_work"]),
    ),
    // TODO - ancient wonders are defined in setup/main by great works with "ancient_wonder = yes"
    (Scopes::Province, "has_ancient_wonder", UncheckedValue),
    (Scopes::Province, "has_building", Item(Item::Building)),
    (Scopes::Province, "has_city_status", Boolean),
    (Scopes::Province, "has_construction", Boolean),
    (Scopes::Province, "has_enemy_army", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "has_enemy_navy", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "has_great_work", Boolean),
    (Scopes::Province, "has_minor_river", Boolean),
    (Scopes::Province, "has_owner", Boolean),
    (Scopes::Province, "has_province_modifier", Item(Item::Modifier)),
    (Scopes::Province, "has_province_rank", Item(Item::ProvinceRank)),
    (Scopes::Province, "has_road_towards", ScopeOrItem(Scopes::Province, Item::Province)),
    (Scopes::Province, "has_siege", Boolean),
    (Scopes::Province, "has_specific_construction", Boolean),
    (Scopes::Province, "has_winter", Boolean),
    (Scopes::Province, "is_adjacent_to_major_river", Boolean),
    (Scopes::Province, "is_capital", Boolean),
    (Scopes::Province, "is_coastal", Boolean),
    (Scopes::Province, "is_colonizer", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "is_core_of", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "is_holy_site", Boolean),
    (Scopes::Province, "is_importing_trade_good", Item(Item::TradeGood)),
    (Scopes::Province, "is_in_area", ScopeOrItem(Scopes::Province, Item::Area)),
    (Scopes::Province, "is_in_region", ScopeOrItem(Scopes::Province, Item::Region)),
    (Scopes::Province, "is_inhabitable", Boolean),
    (Scopes::Province, "is_model_shown", UncheckedValue),
    (Scopes::Province, "is_neighbor", ScopeOrItem(Scopes::Province, Item::Province)),
    (Scopes::Province, "is_port", Boolean),
    (Scopes::Province, "is_previous_controller", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "is_previous_owner", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "is_sea", Boolean),
    (Scopes::Province, "is_state_capital", Boolean),
    (Scopes::Province, "is_uninhabitable", Boolean),
    (Scopes::Province, "num_goods_produced", CompareValue),
    (Scopes::Province, "num_of_total_building_slots", CompareValue),
    (Scopes::Province, "num_of_used_building_slots", CompareValue),
    (Scopes::Province, "num_other_religion", CompareValue),
    (Scopes::Province, "num_foreign_culture", CompareValue),
    (Scopes::Province, "num_province_treasures", CompareValue),
    (Scopes::Province, "owned_or_subject_owned", ScopeOrItem(Scopes::Country, Item::Localization)),
    (Scopes::Province, "population_cap", CompareValue),
    (Scopes::Province, "province_dominant_culture_group", Item(Item::CultureGroup)),
    (Scopes::Province, "province_id", CompareValue),
    (Scopes::Province, "province_income", CompareValue),
    (Scopes::Province, "province_manpower_income", CompareValue),
    (Scopes::Province, "province_tax_income", CompareValue),
    (Scopes::Province, "province_unrest", CompareValue),
    (Scopes::Province, "state_loyalty", CompareValue),
    (Scopes::Province, "terrain", Item(Item::Terrain)),
    (Scopes::Province, "total_population", CompareValue),
    (Scopes::Province, "trade_goods", Item(Item::TradeGood)),
    // Country Culture triggers
    (Scopes::CountryCulture, "country_culture_pop_count", CompareValue),
    (Scopes::CountryCulture, "has_country_culture_modifier", Item(Item::Modifier)),
    (Scopes::CountryCulture, "has_pop_type_right", Item(Item::PopType)),
    (Scopes::CountryCulture, "integration_progress", CompareValue),
    (Scopes::CountryCulture, "is_integrated", Boolean),
    (Scopes::CountryCulture, "is_culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
    // None scope triggers
    (Scopes::all_but_none(), "add_to_temporary_list", Special),
    (Scopes::None, "all_false", Control),
    (Scopes::None, "always", Boolean),
    (Scopes::None, "and", Control),
    (Scopes::None, "assert_if", Block(&[("limit", Control), ("?text", UncheckedValue)])),
    (Scopes::None, "assert_read", UncheckedValue),
    (Scopes::None, "calc_true_if", Control),
    (Scopes::None, "current_date", CompareDate),
    (Scopes::None, "custom_tooltip", Special),
    (Scopes::None, "debug_only", Boolean),
    (Scopes::None, "exists", Special),
    (Scopes::None, "game_start_date", CompareDate),
    (Scopes::None, "gender_rules", Boolean),
    (
        Scopes::None,
        "global_variable_list_size",
        Block(&[("name", UncheckedValue), ("value", CompareValue)]),
    ),
    (Scopes::None, "has_agenda", UncheckedValue),
    (Scopes::None, "has_dlc", Item(Item::Dlc)),
    (Scopes::None, "has_global_variable", UncheckedValue),
    (Scopes::None, "has_global_variable_list", UncheckedValue),
    (Scopes::None, "has_local_variable", UncheckedValue),
    (Scopes::None, "has_local_variable_list", UncheckedValue),
    (Scopes::None, "has_variable", UncheckedValue),
    (Scopes::None, "has_variable_list", UncheckedValue),
    (Scopes::None, "is_dynamic_tag", Boolean),
    (Scopes::None, "is_in_list", Special),
    (Scopes::None, "is_dynamic_tag", Boolean),
    (Scopes::None, "is_iron_man", Boolean),
    (Scopes::None, "is_target_alive", Scope(Scopes::Character)),
    (
        Scopes::None,
        "is_target_in_global_variable_list",
        Block(&[("name", UncheckedValue), ("*target", ScopeOkThis(Scopes::all_but_none()))]),
    ),
    (
        Scopes::None,
        "is_target_in_local_variable_list",
        Block(&[("name", UncheckedValue), ("*target", ScopeOkThis(Scopes::all_but_none()))]),
    ),
    (
        Scopes::None,
        "is_target_in_variable_list",
        Block(&[("name", UncheckedValue), ("*target", ScopeOkThis(Scopes::all_but_none()))]),
    ),
    (Scopes::None, "list_size", Block(&[("name", UncheckedValue), ("value", CompareValue)])),
    (
        Scopes::None,
        "local_variable_list_size",
        Block(&[("name", UncheckedValue), ("value", CompareValue)]),
    ),
    (Scopes::None, "nand", Control),
    (Scopes::None, "nor", Control),
    (Scopes::None, "not", Control),
    (Scopes::None, "or", Control),
    (
        Scopes::None,
        "religion_pops_in_country",
        Block(&[
            ("target", ScopeOrItem(Scopes::Religion, Item::Religion)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::all_but_none(), "save_temporary_scope_as", Special),
    (Scopes::None, "switch", Special),
    (Scopes::None, "target_is_valid_character", Scope(Scopes::Character)),
    (Scopes::None, "trigger_else", Control),
    (Scopes::None, "trigger_else_if", Control),
    (Scopes::None, "trigger_if", Control),
    (
        Scopes::None,
        "variable_list_size",
        Block(&[("name", UncheckedValue), ("value", CompareValue)]),
    ),
    (Scopes::None, "weighted_calc_true_if", Special),
    (Scopes::None, "in_color_list", UncheckedValue),
    (Scopes::None, "is_color", UncheckedValue),
];

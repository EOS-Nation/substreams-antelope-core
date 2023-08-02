pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ContributionT {
        pub id: Name,
        pub value: Float64,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct GlobalsRow {
        pub season_id: Uint16,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub grant_fee: Uint64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub bounty_fee: Uint64,
        pub login_contract: Name,
        pub fee_account: Name,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct GrantsRow {
        pub id: Name,
        #[serde(rename = "type")]
        pub type_: Name,
        pub author_user_id: Name,
        pub funding_account: Name,
        pub accepted_tokens: Vec<SymbolCode>,
        pub status: Name,
        pub created_at: TimePointSec,
        pub updated_at: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MatchRow {
        pub grant_id: Name,
        pub round_id: Uint16,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub total_users: Uint64,
        pub sum_value: Float64,
        pub sum_boost: Float64,
        pub sum_sqrt: Float64,
        pub square: Float64,
        pub updated_at: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RoundsRow {
        pub round_id: Uint16,
        pub description: String,
        pub season_id: Uint16,
        pub grant_ids: Vec<Name>,
        pub user_ids: Vec<Name>,
        pub donated_tokens: Vec<ExtendedAsset>,
        pub match_value: Float64,
        pub sum_value: Float64,
        pub sum_boost: Float64,
        pub sum_square: Float64,
        pub created_at: TimePointSec,
        pub updated_at: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct SeasonsRow {
        pub season_id: Uint16,
        pub description: String,
        pub round_ids: Vec<Uint16>,
        pub match_value: Float64,
        pub start_at: TimePointSec,
        pub end_at: TimePointSec,
        pub submission_start_at: TimePointSec,
        pub submission_end_at: TimePointSec,
        pub created_at: TimePointSec,
        pub updated_at: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatusRow {
        pub counters: Vec<Uint32>,
        pub last_updated: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TokensRow {
        pub sym: Symbol,
        pub contract: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub min_amount: Uint64,
        pub oracle_contract: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub oracle_id: Uint64,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TransfersRow {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub transfer_id: Uint64,
        pub from: Name,
        pub to: Name,
        pub ext_quantity: ExtendedAsset,
        pub fee: Asset,
        pub memo: String,
        pub user_id: Name,
        pub season_id: Uint16,
        pub round_id: Uint16,
        pub project_type: Name,
        pub project_id: Name,
        pub value: Float64,
        pub trx_id: Checksum256,
        pub created_at: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct UsersRow {
        pub user_id: Name,
        pub multiplier: Float64,
        pub value: Float64,
        pub boost: Float64,
        pub contributions: Vec<ContributionT>,
        pub updated_at: TimePointSec,
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Cleartable {
        pub table_name: Name,
        pub round_id: Uint16,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub max_rows: Uint64,
    }
    impl substreams_antelope::action::Action for Cleartable {
        const NAME: &'static str = "cleartable";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Collapse {
        pub user_ids: Vec<Name>,
        pub user_id: Name,
        pub round_id: Uint16,
    }
    impl substreams_antelope::action::Action for Collapse {
        const NAME: &'static str = "collapse";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deltoken {
        pub symcode: SymbolCode,
    }
    impl substreams_antelope::action::Action for Deltoken {
        const NAME: &'static str = "deltoken";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Joinround {
        pub grant_id: Name,
        pub round_id: Uint16,
    }
    impl substreams_antelope::action::Action for Joinround {
        const NAME: &'static str = "joinround";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Removeuser {
        pub user_ids: Vec<Name>,
        pub round_id: Uint16,
    }
    impl substreams_antelope::action::Action for Removeuser {
        const NAME: &'static str = "removeuser";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setconfig {
        pub season_id: Uint16,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub grant_fee: Uint64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub bounty_fee: Uint64,
        pub login_contract: Name,
        pub fee_account: Name,
    }
    impl substreams_antelope::action::Action for Setconfig {
        const NAME: &'static str = "setconfig";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setfunding {
        pub grant_id: Name,
        pub user_id: Name,
    }
    impl substreams_antelope::action::Action for Setfunding {
        const NAME: &'static str = "setfunding";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setgrant {
        pub author_id: Name,
        pub project_id: Name,
        pub funding_account: Name,
        pub accepted_tokens: Vec<SymbolCode>,
    }
    impl substreams_antelope::action::Action for Setgrant {
        const NAME: &'static str = "setgrant";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setgrantid {
        pub grant_id: Name,
        pub new_grant_id: Name,
    }
    impl substreams_antelope::action::Action for Setgrantid {
        const NAME: &'static str = "setgrantid";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setproject {
        pub author_id: Name,
        pub project_type: Name,
        pub project_id: Name,
        pub funding_account: Name,
        pub accepted_tokens: Vec<SymbolCode>,
    }
    impl substreams_antelope::action::Action for Setproject {
        const NAME: &'static str = "setproject";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setround {
        pub round_id: Uint16,
        pub season_id: Uint16,
        pub description: String,
        pub match_value: Float64,
    }
    impl substreams_antelope::action::Action for Setround {
        const NAME: &'static str = "setround";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setseason {
        pub season_id: Uint16,
        pub start_at: TimePointSec,
        pub end_at: TimePointSec,
        pub submission_start_at: TimePointSec,
        pub submission_end_at: TimePointSec,
        pub description: String,
        pub match_value: Float64,
    }
    impl substreams_antelope::action::Action for Setseason {
        const NAME: &'static str = "setseason";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setstate {
        pub project_id: Name,
        pub state: Name,
    }
    impl substreams_antelope::action::Action for Setstate {
        const NAME: &'static str = "setstate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Token {
        pub sym: Symbol,
        pub contract: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub min_amount: Uint64,
        pub oracle_contract: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub oracle_id: Uint64,
    }
    impl substreams_antelope::action::Action for Token {
        const NAME: &'static str = "token";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Unjoinround {
        pub grant_id: Name,
        pub round_id: Uint16,
    }
    impl substreams_antelope::action::Action for Unjoinround {
        const NAME: &'static str = "unjoinround";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Value {
        #[serde(rename = "in")]
        pub in_: ExtendedAsset,
    }
    impl substreams_antelope::action::Action for Value {
        const NAME: &'static str = "value";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
}
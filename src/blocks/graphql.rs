use self::blocks_query::BlockQueryInput;
use chrono::Utc;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<Utc>;
type Long = i32;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/mina-explorer.graphql",
    query_path = "graphql/queries/blocks.graphql",
    response_derives = "Serialize,PartialEq,Debug,Clone",
    skip_serializing_none
)]
pub struct BlocksQuery;

#[allow(clippy::derivable_impls)]
impl Default for blocks_query::BlocksQueryBlocks {
    fn default() -> Self {
        blocks_query::BlocksQueryBlocks {
            block_height: None,
            date_time: None,
            state_hash: None,
            canonical: None,
            transactions: None,
            creator_account: None,
            snark_jobs: None,
            protocol_state: None,
            winner_account: None,
            snark_fees: None,
            tx_fees: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for blocks_query::BlocksQueryBlocksTransactions {
    fn default() -> Self {
        blocks_query::BlocksQueryBlocksTransactions {
            coinbase: None,
            coinbase_receiver_account: None,
            user_commands: None,
            fee_transfer: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for BlockQueryInput {
    fn default() -> Self {
        BlockQueryInput {
            creator_nin: None,
            state_hash_lte: None,
            canonical_ne: None,
            date_time_lt: None,
            snark_jobs: None,
            creator_ne: None,
            received_time: None,
            block_height_lte: None,
            state_hash_field_gte: None,
            received_time_in: None,
            block_height: None,
            state_hash_field_exists: None,
            block_height_nin: None,
            creator_gt: None,
            state_hash_gte: None,
            state_hash_lt: None,
            creator_gte: None,
            protocol_state: None,
            canonical_exists: None,
            date_time_nin: None,
            creator_lte: None,
            creator_account: None,
            state_hash_field_lt: None,
            creator_in: None,
            state_hash_ne: None,
            received_time_ne: None,
            creator: None,
            state_hash_field_lte: None,
            date_time_lte: None,
            date_time_exists: None,
            state_hash_field_gt: None,
            date_time: None,
            date_time_gt: None,
            winner_account_exists: None,
            received_time_gte: None,
            protocol_state_exists: None,
            state_hash_exists: None,
            canonical: None,
            creator_exists: None,
            received_time_lte: None,
            block_height_exists: None,
            state_hash_field_ne: None,
            winner_account: None,
            or: None,
            state_hash_field: None,
            received_time_lt: None,
            transactions: None,
            date_time_gte: None,
            and: None,
            creator_account_exists: None,
            block_height_in: None,
            received_time_nin: None,
            snark_jobs_nin: None,
            date_time_in: None,
            snark_jobs_in: None,
            block_height_lt: None,
            state_hash_field_in: None,
            block_height_ne: None,
            transactions_exists: None,
            creator_lt: None,
            received_time_exists: None,
            block_height_gt: None,
            state_hash_nin: None,
            state_hash_field_nin: None,
            date_time_ne: None,
            state_hash_in: None,
            state_hash: None,
            block_height_gte: None,
            received_time_gt: None,
            snark_jobs_exists: None,
            state_hash_gt: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for blocks_query::BlockCreatorAccountQueryInput {
    fn default() -> Self {
        blocks_query::BlockCreatorAccountQueryInput {
            public_key_exists: None,
            public_key_ne: None,
            public_key_lte: None,
            or: None,
            public_key_gte: None,
            public_key_lt: None,
            public_key_in: None,
            and: None,
            public_key: None,
            public_key_nin: None,
            public_key_gt: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for blocks_query::BlockProtocolStateQueryInput {
    fn default() -> Self {
        blocks_query::BlockProtocolStateQueryInput {
            previous_state_hash_exists: None,
            blockchain_state_exists: None,
            consensus_state: None,
            previous_state_hash_ne: None,
            consensus_state_exists: None,
            previous_state_hash_nin: None,
            previous_state_hash_lt: None,
            or: None,
            previous_state_hash_lte: None,
            blockchain_state: None,
            previous_state_hash_gte: None,
            previous_state_hash_gt: None,
            previous_state_hash_in: None,
            and: None,
            previous_state_hash: None,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for blocks_query::BlockProtocolStateConsensusStateQueryInput {
    fn default() -> Self {
        blocks_query::BlockProtocolStateConsensusStateQueryInput {
            slot_ne: None,
            block_height_nin: None,
            min_window_density_gt: None,
            block_height_in: None,
            slot_since_genesis_gte: None,
            epoch_count_ne: None,
            slot_lt: None,
            epoch_exists: None,
            or: None,
            blockchain_length_gt: None,
            min_window_density_ne: None,
            total_currency_ne: None,
            slot_since_genesis_lte: None,
            slot_exists: None,
            next_epoch_data: None,
            block_height: None,
            total_currency_gt: None,
            epoch_lt: None,
            epoch_count_gt: None,
            epoch_ne: None,
            blockchain_length_lte: None,
            slot_lte: None,
            slot_nin: None,
            blockchain_length_nin: None,
            slot_in: None,
            min_window_density_gte: None,
            and: None,
            epoch_gt: None,
            slot_since_genesis_nin: None,
            slot_since_genesis_exists: None,
            has_ancestor_in_same_checkpoint_window_exists: None,
            total_currency_lt: None,
            staking_epoch_data: None,
            slot_since_genesis_ne: None,
            slot_gte: None,
            slot: None,
            next_epoch_data_exists: None,
            min_window_density_lt: None,
            blockchain_length_exists: None,
            has_ancestor_in_same_checkpoint_window: None,
            blockchain_length_gte: None,
            epoch: None,
            last_vrf_output_lte: None,
            min_window_density_exists: None,
            epoch_count_nin: None,
            block_height_lte: None,
            total_currency_nin: None,
            block_height_exists: None,
            epoch_count_gte: None,
            blockchain_length_ne: None,
            total_currency_lte: None,
            slot_since_genesis_in: None,
            total_currency_gte: None,
            epoch_nin: None,
            min_window_density_lte: None,
            epoch_count_lte: None,
            slot_gt: None,
            slot_since_genesis_gt: None,
            has_ancestor_in_same_checkpoint_window_ne: None,
            min_window_density_in: None,
            total_currency_in: None,
            total_currency_exists: None,
            min_window_density: None,
            min_window_density_nin: None,
            epoch_gte: None,
            last_vrf_output_gt: None,
            block_height_gte: None,
            blockchain_length_lt: None,
            block_height_gt: None,
            last_vrf_output_nin: None,
            epoch_count: None,
            blockchain_length: None,
            last_vrf_output_exists: None,
            epoch_count_exists: None,
            last_vrf_output_in: None,
            epoch_count_in: None,
            last_vrf_output_ne: None,
            block_height_lt: None,
            slot_since_genesis_lt: None,
            epoch_in: None,
            block_height_ne: None,
            last_vrf_output: None,
            blockchain_length_in: None,
            last_vrf_output_gte: None,
            staking_epoch_data_exists: None,
            epoch_count_lt: None,
            slot_since_genesis: None,
            epoch_lte: None,
            last_vrf_output_lt: None,
            total_currency: None,
        }
    }
}

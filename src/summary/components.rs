use crate::{
    blocks::components::UniqueBlocksProducersSummaryItem,
    common::{components::*, functions::*},
    summary::models::*,
};
use leptos::*;

#[component]
pub fn SummaryGrid(summary: Option<BlockchainSummary>) -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-5 auto-rows-min gap-4 p-4 pt-0">
            <h2 class="h-0 w-0 overflow-hidden absolute">"Summary"</h2>
            <SummaryItem
                id="epoch"
                label="Epoch"
                value=summary.as_ref().map(|s| format_number(s.epoch))
            />
            <UniqueBlocksProducersSummaryItem/>
            <SummaryItem
                id="globalSlot"
                label="Global Slot"
                value=summary.as_ref().map(|s| format_number(s.global_slot))
            />
            <SummaryItem
                id="blockchainLength"
                label="Block Height"
                value=summary.as_ref().map(|s| format_number(s.blockchain_length))
            />
            <SummaryItem
                id="totalCurrency"
                label="Total Currency"
                value=summary
                    .as_ref()
                    .map(|s| {
                        format_mina(s.tot_currency().to_string())
                            .split('.')
                            .collect::<Vec<_>>()[0]
                            .to_string()
                    })
            />

            <SummaryItem
                id="circulatingSupply"
                label="Circulating Supply"
                value=summary
                    .as_ref()
                    .map(|s| {
                        format_mina(s.circ_supply().to_string())
                            .split('.')
                            .collect::<Vec<_>>()[0]
                            .to_string()
                    })
            />

            <SummaryItem
                id="totalNumBlocks"
                label="Total Blocks"
                value=summary.as_ref().map(|s| format_number(s.total_num_blocks))
            />
            <SummaryItem
                id="totalUserCommands"
                label="Total User  Txn"
                value=summary.as_ref().map(|s| format_number(s.total_num_user_commands))
            />
            <SummaryItem
                id="totalIntCommands"
                label="Total Internal Txn"
                value=summary.as_ref().map(|s| format_number(s.total_num_internal_commands))
            />
            <SummaryItem
                id="totalSnarks"
                label="Total SNARKs"
                value=summary.as_ref().map(|s| format_number(s.total_num_snarks))
            />
        </section>
    }
}
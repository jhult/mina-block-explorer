use leptos::*;

use super::functions::*;
use super::models::*;
use crate::transactions::components::TransactionsSubsection;

#[component]
pub fn AccountDialog(path_base: String, account: AccountSummary) -> impl IntoView {
    let id = account.public_key.clone();
    let summary_items = get_summary_items(account.clone());
    let public_key = account.public_key.clone();
    
    view! {
        <dialog id="accountdialog" class="z-20 w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <section>
                <div class="flex justify-between">
                    <h2 class="text-bold text-xl">"Account Overview"</h2>
                    <button id="closedialog">
                        <a href=path_base>X</a>
                    </button>
                </div>
                <AccountSummarySubsection summary_items=summary_items public_key=account.public_key username=account.username />
            </section>
            <TransactionsSubsection limit=3 account_id=public_key />
            <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                <button id="viewmore" class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg">
                    <a href={format!("/accounts/{}", id)}>"View all details"</a>
                </button>
            </div>
        </dialog>
    }.into_view()
}


#[component]
pub fn AccountSummarySubsection(summary_items: Vec<(String, String, bool)>, username: String, public_key: String) -> impl IntoView {
    view! {
        <div class="@lg:grid @lg:grid-cols-[10rem_5rem_auto_10rem] @lg:grid-rows-[2.5rem_2.5rem] @lg:gap-x-[2rem] @lg:h-auto flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
            <div class="@lg:col-start-2 @lg:col-end-3 @lg:row-start-1 @lg:row-end-2 w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%]">
                <img src="/assets/img/account_balance_wallet.svg" alt="account balance wallet logo"/>
            </div>
            <div class="@lg:col-start-3 text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden">
                {public_key}
            </div>
            <div class="@lg:col-start-3 @lg:row-start-2 text-slate-400 text-sm">
                "Username: "{username}
            </div>
        </div>
        <div class="@lg:grid @lg:grid-cols-[10rem_auto_10rem] bg-white rounded-xl flex flex-col items-stretch mt-8 p-4">
            {summary_items.into_iter()
                .map(|(label, value, has_pill)| view! {
                    <OverviewEntry label=label.to_owned() value=value.to_owned() has_pill=has_pill />
                })
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn OverviewEntry(label: String, value: String, has_pill: bool) -> impl IntoView {
    let value_class_str_base = "py-1 my-1 text-sm";

    let value_class_str = match has_pill {
        true => format!("{} {}",value_class_str_base.to_owned(),"p-1 rounded-full bg-light-granola-orange"),
        false => format!("{} {}",value_class_str_base.to_owned(),"w-3/4 text-ellipsis overflow-hidden"),
    };

    view! {
        <div class="@lg:col-start-2 @lg:col-end-3 flex flex-col items-start md:flex-row md:items-baseline md:justify-start">
            <span class="w-1/4 text-slate-400 text-sm whitespace-nowrap">{label}:</span>
            <span class=value_class_str>{value}</span>
        </div>
    }
}
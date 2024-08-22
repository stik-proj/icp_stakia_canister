use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    symbol: String,
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_args: TransferArg = TransferArg {
        memo: None,
        amount: args.amount,
        from_subaccount: None,
        fee: None,
        to: args.to_account,
        created_at_time: None,
    };

    // Replace with the appropriate Principal ID for the token based on the symbol
    let ledger_canister_id = get_ledger_canister_id(&args.symbol)?;

    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        ledger_canister_id,
        "icrc1_transfer",
        (transfer_args,),
    )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

// Function to map symbol to the correct ledger canister ID
fn get_ledger_canister_id(symbol: &str) -> Result<Principal, String> {
    match symbol {
        "ICP" => Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai")
            .map_err(|_| "Invalid Principal for ICP".to_string()),
        // Add more symbols and their corresponding canister IDs here
        _ => Err("Unsupported token symbol".to_string()),
    }
}

ic_cdk::export_candid!();

//! A program that computes DeFi metrics (ICR, liquidation threshold, LTV) for a BTC-collateralized
//! loan system, committing the results as public values in a zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use alloy_sol_types::SolValue;
use fibonacci_lib::{
  PublicValuesAddress,
};

pub fn main() {
    // Read inputs from zkVM
    // let collateral_amount = sp1_zkvm::io::read::<u32>(); // BTC collateral (in units, e.g., scaled)
    // let debt_amount = sp1_zkvm::io::read::<u32>();       // Debt amount (in USD, scaled)
    // let btc_price_usd = sp1_zkvm::io::read::<u32>();     // BTC price in USD
    // // let min_icr = sp1_zkvm::io::read::<u32>();           // Minimum ICR required (e.g., 150 for 150%)

    let user_address = sp1_zkvm::io::read::<[u8; 20]>(); // User address (20 bytes)

    // // Compute ICR and collateral value in USD
    // let (icr, collateral_usd_value) = calculate_icr(collateral_amount, debt_amount, btc_price_usd);

    // Compute liquidation threshold
    // let liquidation_threshold = calculate_liquidation_threshold(collateral_amount, btc_price_usd, min_icr);

    // // Compute real-time LTV
    // let real_time_ltv = real_time_ltv(debt_amount, collateral_amount, btc_price_usd);

    // Encode public values into PublicValuesStruct
    let public_values = PublicValuesAddress {
        user_address: user_address.into(),
    };

    // ABI-encode the public values
    // let bytes = PublicValuesIcr::abi_encode(&public_values);
    let bytes = public_values.abi_encode();

    // Commit the encoded public values to the zkVM
    sp1_zkvm::io::commit_slice(&bytes);
}
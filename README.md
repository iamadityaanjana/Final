open index.html to navigate from beggining

**Description**
A Decentralized Real-Estate Crowdfunding platform based on MANTRA blockchain that allows users to own fractionalized parts of real-estate properties using a variety of options, not limited to NFTs, Crpto etc.

**Smart Contract Deployment**

# Guide to deploying a basic smart contract on the MANTRA Hongbai testnet 
#User intervention required

#WSL Recommeneded


#STEP1 : Install Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
#Select Options

rustup default stable

cargo version


#STEP2: Add wasm target
rustup target add wasm32-unknown-unknown


#STEP3: Install cargo-generate package
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script


#STEP4: Get boilerplate code ready 
# Specify your project name in place of <PROJECT_NAME>
cargo generate --git https://github.com/CosmWasm/cw-template.git --name <PROJECT_NAME>

cd <PROJECT_NAME>

STEP5: Run the test Command to confirm the installation
cargo generate --git https://github.com/CosmWasm/cw-template.git --name first-token

cd ./first-token

#NANO Navigation and Commands
#Ctrl+X to save
#Enter Y to confirm
#Press enter to save and exit

#Paste the following after editing the file using nano

#File 1

#[dependencies]
#cw20-base = {  version = "0.13.2", features =["library"] }

nano Cargo.toml


#File 2

#use schemars::JsonSchema;
#use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#pub struct MigrateMsg {}

nano src/msg.rs


#File 3

#pub mod contract;
#pub mod msg;

nano src/lib.rs


#File 4

#[cfg(not(feature = "library"))]
#use cosmwasm_std::entry_point;
#use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
#use cw20_base::ContractError;
#use cw20_base::enumerable::{query_all_allowances, query_all_accounts};
#use cw20_base::msg::{QueryMsg,ExecuteMsg};

#use crate::msg::MigrateMsg;
#use cw2::set_contract_version;
#use cw20_base::allowances::{
    execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance, execute_burn_from,
};
#use cw20_base::contract::{
    execute_mint, execute_send, execute_transfer, execute_update_marketing,
    execute_upload_logo, query_balance, query_token_info, query_minter, query_download_logo, query_marketing_info, execute_burn,
};

// version info for migration info
#const CONTRACT_NAME: &str = "crates.io:cw20-token";
#const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
#pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: cw20_base::msg::InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    /* Execute the instantiate method from cw_20_base as the code from that
    library is already battle tested we do not have to re-write the full
    functionality: https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base*/
    Ok(cw20_base::contract::instantiate(deps, env, info, msg)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
#pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, cw20_base::ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            execute_transfer(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => execute_send(deps, env, info, contract, amount, msg),
        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, env, info, recipient, amount),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::BurnFrom { owner, amount } => execute_burn_from(deps, env, info, owner, amount),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => execute_send_from(deps, env, info, owner, contract, amount, msg),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => execute_update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
#pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        /* Default methods from CW20 Standard with no modifications:
        https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base */
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&query_all_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
#pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
} 

nano src/contract.rs


#STEP5 : Download Pre-requisite

#Move up directory to root
#cd ../../

#CosmWasm Library
#sudo wget -P /usr/lib https://github.com/CosmWasm/wasmvm/releases/download/v1.3.0/libwasmvm.x86_64.so 

#Mantra CLI
#Download CLI
curl -LO https://github.com/MANTRA-Finance/public/raw/main/mantrachain-testnet/mantrachaind-linux-amd64.zip
#UnZIP CLI
unzip mantrachaind-linux-amd64.zip

#Test command
mantrachaind

#Global Declaration
sudo mv mantrachaind /usr/bin


#STEP6: Setting up enviornment

#Paste following into mantrachaind-cli.env using nano

export CHAIN_ID="mantra-hongbai-1"
export TESTNET_NAME="mantra-hongbai"
export FEE_DENOM="uom"
export STAKE_DENOM="uom"
export BECH32_HRP="wasm"
export WASMD_VERSION="v0.27.0"
export CONFIG_DIR=".mantrachaind"
export BINARY="mantrachaind"

export COSMJS_VERSION="v0.28.1"
export GENESIS_URL="https://<location-to-be-provided>/config/genesis.json"

export RPC="https://rpc.hongbai.mantrachain.io:443"
export FAUCET="https://faucet.hongbai.mantrachain.io"

export NODE=(--node $RPC)
export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.25uom --gas auto --gas-adjustment 1.3)

nano mantrachaind-cli.env

source mantrachaind-cli.env

#Setup Wallets

mantrachaind keys add wallet
mantrachaind keys add wallet2

#Request testnet tokens
#Faucet
https://faucet.hongbai.mantrachain.io/

#Use above created wallet addresses


#Downloading and Compiling a Smart Contract

#Setting up Contract
# Download the repository
git clone https://github.com/InterWasm/cw-contracts
cd cw-contracts
git checkout main
cd contracts/nameservice

# compile the wasm contract with stable toolchain
rustup default stable
cargo wasm

#Setting up Docker
#Not necessary, but is preferred to reduce gas costs
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11


#Deploying and Interacting with the Smart Contract
#Relatively small code, may wotk
#Sure shot code

#See list of codes uploaded to the network
mantrachaind query wasm list-code --node https://rpc.hongbai.mantrachain.io:443

#

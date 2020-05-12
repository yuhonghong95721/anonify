use std::{sync::Arc, thread, time};
use failure::Error;
use log::debug;
use anonify_host::dispatcher::get_state;
use anonify_event_watcher::{
    BlockNumDB,
    traits::*,
};
use anonify_runtime::{U64, Approved};
use app::{approve, transfer, construct, transfer_from, mint, burn};
use actix_web::{
    web,
    HttpResponse,
};
use anyhow::anyhow;
use crate::Server;

const DEFAULT_SEND_GAS: u64 = 3_000_000;

pub fn handle_deploy<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::deploy::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    debug!("Starting deploy a contract...");

    let deployer_addr = server.dispatcher.get_account(0)?;
    let contract_addr = server.dispatcher
        .deploy(&deployer_addr)?;

    debug!("Contract address: {:?}", &contract_addr);
    server.dispatcher.set_contract_addr(&contract_addr, &server.abi_path)?;

    Ok(HttpResponse::Ok().json(api::deploy::post::Response(contract_addr)))
}

pub fn handle_register<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::register::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let signer = server.dispatcher.get_account(0)?;
    let dispatcher = &server.dispatcher;
    let t0 = std::time::SystemTime::now();
    let receipt = dispatcher.register(
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;
    println!("t0: {:?}", t0);

    Ok(HttpResponse::Ok().json(api::register::post::Response(receipt)))
}

pub fn handle_init_state<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::init_state::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let access_right = req.into_access_right()?;
    let signer = server.dispatcher.get_account(0)?;
    let total_supply = U64::from_raw(req.total_supply);
    let init_state = construct{ total_supply };

    let receipt = server.dispatcher.state_transition(
        access_right,
        init_state,
        req.state_id,
        "construct",
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;

    Ok(HttpResponse::Ok().json(api::init_state::post::Response(receipt)))
}

pub fn handle_transfer<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::transfer::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let access_right = req.into_access_right()?;
    let signer = server.dispatcher.get_account(0)?;
    let amount = U64::from_raw(req.amount);
    let recipient = req.target;
    let transfer_state = transfer{ amount, recipient };

    let t2 = std::time::SystemTime::now();
    let receipt = server.dispatcher.state_transition(
        access_right,
        transfer_state,
        req.state_id,
        "transfer",
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;
    println!("t2: {:?}", t2);

    Ok(HttpResponse::Ok().json(api::transfer::post::Response(receipt)))
}

pub fn handle_approve<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::approve::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let access_right = req.into_access_right()?;
    let signer = server.dispatcher.get_account(0)?;
    let amount = U64::from_raw(req.amount);
    let spender = req.target;
    let approve_state = approve { amount, spender };

    let receipt = server.dispatcher.state_transition(
        access_right,
        approve_state,
        req.state_id,
        "approve",
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;

    Ok(HttpResponse::Ok().json(api::approve::post::Response(receipt)))
}

pub fn handle_mint<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::mint::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let access_right = req.into_access_right()?;
    let signer = server.dispatcher.get_account(0)?;
    let amount = U64::from_raw(req.amount);
    let recipient = req.target;
    let minting_state = mint{ amount, recipient };

    let receipt = server.dispatcher.state_transition(
        access_right,
        minting_state,
        req.state_id,
        "mint",
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;

    Ok(HttpResponse::Ok().json(api::mint::post::Response(receipt)))
}

pub fn handle_burn<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::burn::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let access_right = req.into_access_right()?;
    let signer = server.dispatcher.get_account(0)?;
    let amount = U64::from_raw(req.amount);
    let burn_state = burn{ amount };

    let receipt = server.dispatcher.state_transition(
        access_right,
        burn_state,
        req.state_id,
        "burn",
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;

    Ok(HttpResponse::Ok().json(api::burn::post::Response(receipt)))
}

pub fn handle_transfer_from<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::transfer_from::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let access_right = req.into_access_right()?;
    let signer = server.dispatcher.get_account(0)?;
    let amount = U64::from_raw(req.amount);
    let owner = req.owner;
    let recipient = req.target;
    let transferred_from_state = transfer_from { owner, recipient, amount };

    let receipt = server.dispatcher.state_transition(
        access_right,
        transferred_from_state,
        req.state_id,
        "transfer_from",
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;

    Ok(HttpResponse::Ok().json(api::transfer_from::post::Response(receipt)))
}

pub fn handle_key_rotation<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::key_rotation::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    let signer = server.dispatcher.get_account(0)?;
    let receipt = server.dispatcher.handshake(
        signer,
        DEFAULT_SEND_GAS,
        &req.contract_addr,
        &server.abi_path,
    )?;

    Ok(HttpResponse::Ok().json(api::key_rotation::post::Response(receipt)))
}

/// Fetch events from blockchain nodes manually, and then get the balance of the address approved by the owner from enclave.
pub fn handle_allowance<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::allowance::get::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    server.dispatcher.block_on_event(&req.contract_addr, &server.abi_path)?;

    let access_right = req.into_access_right()?;
    let owner_approved = get_state::<Approved>(&access_right, server.eid, "Approved")?;
    let approved_amount = owner_approved.allowance(&req.spender).unwrap();
    // TODO: stop using unwrap when switching from failure to anyhow.

    Ok(HttpResponse::Ok().json(api::allowance::get::Response((*approved_amount).as_raw())))
}

/// Fetch events from blockchain nodes manually, and then get balance of the address from enclave.
pub fn handle_balance_of<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::state::get::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    server.dispatcher.block_on_event(&req.contract_addr, &server.abi_path)?;

    let access_right = req.into_access_right()?;
    let state = get_state::<U64>(&access_right, server.eid, "Balance")?;

    Ok(HttpResponse::Ok().json(api::state::get::Response(state.as_raw())))
}

pub fn handle_start_polling<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::state::start_polling::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer + Send + Sync + 'static,
        S: Sender + Send + Sync + 'static,
        W: Watcher<WatcherDB=DB> + Send + Sync + 'static,
        DB: BlockNumDB + Send + Sync + 'static,
{
    let _ = thread::spawn(move || {
        loop {
            server.dispatcher.block_on_event(&req.contract_addr, &server.abi_path).unwrap();
            debug!("event fetched...");
            thread::sleep(time::Duration::from_secs(3));
        }
    });

    Ok(HttpResponse::Ok().finish())
}

pub fn handle_set_contract_addr<D, S, W, DB>(
    server: web::Data<Arc<Server<D, S, W, DB>>>,
    req: web::Json<api::contract_addr::post::Request>,
) -> Result<HttpResponse, Error>
    where
        D: Deployer,
        S: Sender,
        W: Watcher<WatcherDB=DB>,
        DB: BlockNumDB,
{
    debug!("Starting set a contract address...");

    debug!("Contract address: {:?}", &req.contract_addr);
    server.dispatcher.set_contract_addr(&req.contract_addr, &server.abi_path)?;

    Ok(HttpResponse::Ok().finish())
}

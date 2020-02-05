use anonify_types::{RawRegisterTx, RawInitStateTx, traits::RawEnclaveTx};
use anonify_common::{State, UserAddress, Ciphertext};
use crate::{
    attestation::{Report, ReportSig, AttestationService},
    error::Result,
    quote::{EnclaveContext, ENCLAVE_CONTEXT},
    bridges::ocalls::save_to_host_memory,
    state::{LockParam, UserState},
    crypto::SYMMETRIC_KEY,
};

pub trait EnclaveTx: Sized {
    type R: RawEnclaveTx;

    fn into_raw(self) -> Result<Self::R>;
 }

#[derive(Debug, Clone)]
pub struct RegisterTx {
    report: Report,
    report_sig: ReportSig,
    // AddHandShake
}

impl EnclaveTx for RegisterTx {
    type R = RawRegisterTx;

    fn into_raw(self) -> Result<Self::R> {
        let report = save_to_host_memory(&self.report.as_bytes())? as *const u8;
        let report_sig = save_to_host_memory(&self.report_sig.as_bytes())? as *const u8;

        Ok(RawRegisterTx {
            report,
            report_sig,
        })
    }
}

impl RegisterTx {
    pub fn new(report: Report, report_sig: ReportSig) -> Self {
        RegisterTx {
            report,
            report_sig,
        }
    }

    pub fn construct(
        host: &str,
        path: &str,
        ias_api_key: &str,
        ctx: &EnclaveContext,
    ) -> Result<Self> {
        let service = AttestationService::new(host, path);
        let quote = ctx.get_quote()?;
        let (report, report_sig) = service.get_report_and_sig_new(&quote, ias_api_key)?;

        Ok(RegisterTx {
            report,
            report_sig,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InitStateTx {
    state_id: u64,
    ciphertext: Ciphertext,
    lock_param: LockParam,
    enclave_sig: secp256k1::Signature,
}

impl InitStateTx {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn construct<S: State>(
        state_id: u64,
        params: &[u8],
        user_address: UserAddress,
        enclave_ctx: &EnclaveContext,
    ) -> Result<Self> {
        let params = S::from_bytes(params)?;
        let init_state = UserState::<S, _>::init(user_address, params)
            .expect("Failed to initialize state.");
        let lock_param = *init_state.lock_param();
        let ciphertext = init_state.encrypt(&SYMMETRIC_KEY)
            .expect("Failed to encrypt init state.");
        let enclave_sig = enclave_ctx.sign(&lock_param)?;

        Ok(InitStateTx {
            state_id,
            ciphertext,
            lock_param,
            enclave_sig,
        })
    }
}

impl EnclaveTx for InitStateTx {
    type R = RawInitStateTx;

    fn into_raw(self) -> Result<Self::R> {
        let ciphertext = save_to_host_memory(&self.ciphertext.as_bytes())? as *const u8;
        let lock_param = save_to_host_memory(&self.lock_param.as_bytes())? as *const u8;
        let enclave_sig = save_to_host_memory(&self.enclave_sig.serialize())? as *const u8;

        Ok(RawInitStateTx {
            state_id: self.state_id,
            ciphertext,
            lock_param,
            enclave_sig,
        })
    }
}

// #[derive(Debug, Clone)]
// pub struct StateTransitionTx {
//     ciphertexts: Vec<Ciphertext>,
//     lock_params: Vec<LockParam>,
//     enclave_sig: secp256k1::Signature,
//     blc_num: u64,
//     state_hash: Vec<u8>,
// }


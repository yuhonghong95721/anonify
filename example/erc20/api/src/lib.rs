use std::fmt;
use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use rand::Rng;
use ed25519_dalek::{Keypair, Signature, PublicKey, SignatureError, SIGNATURE_LENGTH, PUBLIC_KEY_LENGTH};
use anonify_common::{AccessRight, UserAddress};
use anonify_runtime::State;

// ----------------------
//  GET and POST types
// ----------------------

pub mod deploy {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                rng: &mut R
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod join_group {
    pub mod post {
        use super::super::*;

        #[derive(Clone, Deserialize, Serialize, Debug)]
        pub struct Request {
            pub contract_addr: String,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod init_state {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub total_supply: u64,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                total_supply: u64,
                state_id: u64,
                contract_addr: String,
                rng: &mut R
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    total_supply,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, total_supply: {:?}, state_id: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.total_supply, self.state_id, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod transfer {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub target: UserAddress,
            pub amount: u64,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                amount: u64,
                state_id: u64,
                target: UserAddress,
                contract_addr: String,
                rng: &mut R,
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    target,
                    amount,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, target: {:?}, amount: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.target, self.amount, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod approve {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub target: UserAddress,
            pub amount: u64,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                amount: u64,
                state_id: u64,
                target: UserAddress,
                contract_addr: String,
                rng: &mut R,
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    target,
                    amount,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, target: {:?}, amount: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.target, self.amount, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod transfer_from {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub owner: UserAddress,
            pub target: UserAddress,
            pub amount: u64,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                amount: u64,
                state_id: u64,
                owner: UserAddress,
                target: UserAddress,
                contract_addr: String,
                rng: &mut R,
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    owner,
                    target,
                    amount,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, owner: {:?}, target: {:?}, amount: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.owner, self.target, self.amount, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod mint {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub target: UserAddress,
            pub amount: u64,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                amount: u64,
                state_id: u64,
                target: UserAddress,
                contract_addr: String,
                rng: &mut R,
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    target,
                    amount,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, target: {:?}, amount: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.target, self.amount, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod burn {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub amount: u64,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                amount: u64,
                state_id: u64,
                contract_addr: String,
                rng: &mut R,
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    amount,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, amount: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.amount, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod key_rotation {
    pub mod post {
        use super::super::*;

        #[derive(Clone, Deserialize, Serialize, Debug)]
        pub struct Request {
            pub contract_addr: String,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response(pub String);
    }
}

pub mod allowance {
    pub mod get {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub spender: UserAddress,
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                contract_addr: String,
                spender: UserAddress,
                state_id: u64,
                rng: &mut R
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    spender,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, spender: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.spender, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response<S: State>(pub S);
    }
}

pub mod state {
    pub mod get {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
            pub state_id: u64,
            pub contract_addr: String,
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                contract_addr: String,
                state_id: u64,
                rng: &mut R
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                    state_id,
                    contract_addr,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }

        impl fmt::Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Request {{ sig: {:?}, pubkey: {:?}, challenge: {:?}, contract address: {:?} }}",
                    &self.sig[..], self.pubkey, self.challenge, self.contract_addr
                )
            }
        }

        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Response<S: State>(pub S);
    }

    pub mod start_sync_bc {
        use super::super::*;

        #[derive(Clone, Deserialize, Serialize, Debug)]
        pub struct Request {
            pub contract_addr: String,
        }

        impl Request {
            pub fn new(contract_addr: String) -> Self {
                Request { contract_addr }
            }
        }
    }
}

pub mod contract_addr {
    pub mod post {
        use super::super::*;

        #[derive(Clone, Deserialize, Serialize, Debug)]
        pub struct Request {
            pub contract_addr: String,
        }

        impl Request {
            pub fn new(contract_addr: String) -> Self {
                Request { contract_addr }
            }
        }
    }
}

pub mod register_notification {
    pub mod post {
        use super::super::*;
        big_array! { BigArray; }

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Request {
            #[serde(with = "BigArray")]
            pub sig: [u8; SIGNATURE_LENGTH],
            pub pubkey: [u8; PUBLIC_KEY_LENGTH],
            pub challenge: [u8; 32],
        }

        impl Request {
            pub fn new<R: Rng>(
                keypair: &Keypair,
                rng: &mut R
            ) -> Self {
                let challenge: [u8; 32] = rng.gen();
                let sig = keypair.sign(&challenge[..]);
                assert!(keypair.verify(&challenge, &sig).is_ok());

                Request {
                    sig: sig.to_bytes(),
                    pubkey: keypair.public.to_bytes(),
                    challenge,
                }
            }

            pub fn into_access_right(&self) -> Result<AccessRight, SignatureError> {
                let sig = Signature::from_bytes(&self.sig)?;
                let pubkey = PublicKey::from_bytes(&self.pubkey)?;

                Ok(AccessRight::new(sig, pubkey, self.challenge))
            }
        }
    }
}

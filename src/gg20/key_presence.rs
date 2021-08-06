//! This module handles the key_presence gRPC.
//! Request includes [proto::message_in::Data::KeyPresenceRequest] struct and encrypted recovery info.
//! The recovery info is decrypted by party's mnemonic seed and saved in the KvStore.

use super::{proto, service::Gg20Service};
use crate::TofndError;
use tracing::info;

impl Gg20Service {
    pub(super) async fn handle_key_presence(
        &mut self,
        request: proto::KeyPresenceRequest,
    ) -> Result<proto::key_presence_response::Response, TofndError> {
        // check if mnemonic is available
        let _ = self.seed().await?;

        // try to get party info related to session id
        match self.shares_kv.get(&request.key_uid).await {
            Ok(_) => {
                info!(
                    "Found session-id {} in kv store during key presence check",
                    request.key_uid
                );
                Ok(proto::key_presence_response::Response::Present)
            }
            Err(_) => {
                info!(
                    "Did not find session-id {} in kv store during key presence check",
                    request.key_uid
                );
                Ok(proto::key_presence_response::Response::Absent)
            }
        }
    }
}
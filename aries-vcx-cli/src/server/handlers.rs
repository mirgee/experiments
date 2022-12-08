use std::sync::RwLockReadGuard;

use anyhow::anyhow;

use aries_vcx_agent::{
    aries_vcx::{messages::a2a::A2AMessage, utils::encryption_envelope::EncryptionEnvelope},
    Agent,
};

// TODO: Should return internal error type
pub async fn handle_message(agent: RwLockReadGuard<'_, Agent>, payload: Vec<u8>) -> anyhow::Result<()> {
    let (message, sender_vk) = EncryptionEnvelope::anon_unpack(&agent.profile().inject_wallet(), payload)
        .await
        .map_err(|err| anyhow!("Failed to unpack message: {}", err))?;
    let sender_vk = sender_vk.ok_or(anyhow!("Received anoncrypted message"))?;
    info!("Received message: {:?}", message);
    let connection_ids = agent
        .connections()
        .get_by_their_vk(&sender_vk)
        .map_err(|_| anyhow!("Failed to unpack message"))?;
    let connection_id = connection_ids.last();
    // TODO: User should be able to decide how to react manually
    match message {
        A2AMessage::ConnectionRequest(request) => {
            let tid = request.get_thread_id();
            agent
                .connections()
                .accept_request(&tid, request)
                .await
                .map_err(|_| anyhow!(""))?;
            agent
                .connections()
                .send_response(&tid)
                .await
                .map_err(|_| anyhow!(""))?;
        }
        A2AMessage::ConnectionResponse(response) => {
            let tid = response.get_thread_id();
            agent
                .connections()
                .accept_response(&tid, response)
                .await
                .map_err(|_| anyhow!(""))?;
            agent
                .connections()
                .send_ack(&tid)
                .await
                .map_err(|_| anyhow!(""))?;
        }
        A2AMessage::CredentialProposal(proposal) => {
            if connection_ids.len() == 1 {
                agent
                    .issuer()
                    .accept_proposal(connection_id.unwrap(), &proposal)
                    .await
                    .map_err(|_| anyhow!(""))?;
            } else {
                return Err(anyhow!("Found multiple or no connections by verkey {}", sender_vk));
            }
        }
        A2AMessage::CredentialOffer(offer) => {
            if connection_ids.len() == 1 {
                agent
                    .holder()
                    .create_from_offer(connection_id.unwrap(), offer)
                    .map_err(|_| anyhow!(""))?;
            } else {
                return Err(anyhow!("Found multiple or no connections by verkey {}", sender_vk));
            }
        }
        A2AMessage::CredentialRequest(request) => {
            agent
                .issuer()
                .process_credential_request(&request.get_thread_id(), request)
                .map_err(|_| anyhow!(""))?;
        }
        A2AMessage::Credential(credential) => {
            agent
                .holder()
                .process_credential(&credential.get_thread_id(), credential).await
                .map_err(|_| anyhow!(""))?;
        }
        A2AMessage::PresentationRequest(request) => {
            if connection_ids.len() == 1 {
                agent
                    .prover()
                    .create_from_request(connection_id.unwrap(), request)
                    .map_err(|_| anyhow!(""))?;
            } else {
                return Err(anyhow!("Found multiple or no connections by verkey {}", sender_vk));
            }
        }
        A2AMessage::Presentation(presentation) => {
            agent
                .verifier()
                .verify_presentation(&presentation.get_thread_id(), presentation)
                .await
                .map_err(|_| anyhow!(""))?;
        }
        A2AMessage::PresentationAck(ack) => {
            agent
                .prover()
                .process_presentation_ack(&ack.get_thread_id(), ack)
                .map_err(|_| anyhow!(""))?;
        }
        m @ _ => warn!("Received message of unexpected type: {:?}", m)
    };
    Ok(())
}

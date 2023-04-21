use std::time::Duration;
use std::{num::NonZeroUsize, thread};

use aries_vcx::{
    common::ledger::{
        service_didsov::{DidSovServiceType, EndpointDidSov},
        transactions::write_endpoint,
    },
    utils::devsetup::SetupProfile,
};
use did_resolver::{
    did_parser::ParsedDIDUrl,
    traits::resolvable::{resolution_options::DIDResolutionOptions, DIDResolvable},
};
use did_resolver_sov::resolution::DIDSovResolver;

#[tokio::test]
async fn write_service_on_ledger_and_resolve_did_doc() {
    SetupProfile::run(|init| async move {
        let did = format!("did:sov:{}", init.institution_did);
        let endpoint = EndpointDidSov::create()
            .set_service_endpoint("http://localhost:8080".to_string())
            .set_routing_keys(Some(vec!["key1".to_string(), "key2".to_string()]))
            .set_types(Some(vec![DidSovServiceType::Endpoint]));
        write_endpoint(&init.profile, &init.institution_did, &endpoint)
            .await
            .unwrap();
        thread::sleep(Duration::from_millis(50));
        let mut resolver =
            DIDSovResolver::new(init.profile.inject_ledger(), NonZeroUsize::new(10).unwrap());
        let did_doc = resolver
            .resolve(
                ParsedDIDUrl::parse(did.clone()).unwrap(),
                DIDResolutionOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(did_doc.did_document().id().to_string(), did);
    })
    .await;
}

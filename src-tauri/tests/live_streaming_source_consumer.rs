//! Real published-source acceptance for the streaming-source consumer.
//!
//! This test uses the production trust root and the published default
//! distribution URL. It refreshes the signed index, downloads and extracts the
//! pinned Node runtime archive for the host platform, installs the latest
//! NetEase release, starts its worker, and asks it to describe itself.
//!
//! It is ignored by default because it needs network access and downloads tens
//! of megabytes. Run it explicitly on a real host:
//!
//! ```text
//! cargo test -q --test live_streaming_source_consumer -- --ignored --nocapture
//! ```
//!
//! On Windows this is the acceptance path for #579: the pinned runtime entry is
//! declared with `/` while the archive extractor works on `Path`, and the
//! worker cannot start unless `node.exe` was extracted and sealed.

use openkara_lib::source_host::{HostConfig, SourceHost, SourceState};
use serde_json::json;

#[tokio::test]
#[ignore = "needs network access to the published signed source index and pinned Node runtime"]
async fn published_source_installs_and_starts_with_production_trust() {
    let root = tempfile::tempdir().unwrap();
    let config = HostConfig::production(
        root.path().to_path_buf(),
        root.path().to_path_buf(),
        env!("CARGO_PKG_VERSION").into(),
    );
    let host = SourceHost::new(config).await.unwrap();

    let refreshed = host.refresh().await.unwrap();
    let netease = refreshed
        .iter()
        .find(|snapshot| snapshot.source_id == "netease")
        .expect("published signed index must describe the NetEase source");
    assert_eq!(netease.state, SourceState::NotInstalled);
    let available = netease
        .available_version
        .clone()
        .expect("published signed index must offer a NetEase release");

    let installed = host.install("netease", None).await.unwrap();
    assert_eq!(
        installed.installed_version.as_deref(),
        Some(available.as_str())
    );
    assert!(installed.runtime_ready);

    host.set_enabled("netease", true, false).await.unwrap();
    let description = host
        .request("netease", "describe", json!({}))
        .await
        .unwrap();
    println!("PUBLISHED SOURCE ACCEPTANCE: {description}");

    assert_eq!(description["name"], "NetEase Cloud Music");
    assert!(description["capabilities"]
        .as_array()
        .is_some_and(|capabilities| capabilities.iter().any(|value| value == "search")));
    assert_eq!(description["session"]["state"], "signed_out");
}

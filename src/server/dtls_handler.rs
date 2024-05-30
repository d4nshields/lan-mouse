use anyhow::Result;
use webrtc::dtls::config::Config;
use webrtc::dtls::conn::DTLSConn;
use webrtc::dtls::Error;
use webrtc::dtls::state::State;
use webrtc::dtls::handshake::HandshakeState;
use webrtc::ice_transport::RTCIceTransport;
use webrtc::RTCIceTransportPolicy;
use webrtc::ice_transport::ice_gatherer::RTCIceGatherer;
use webrtc::ice_transport::ice_role::RTCIceRole;
use webrtc::api::setting_engine::SettingEngine;
use webrtc::util::Conn;
use std::sync::Arc;
use tokio::net::UdpSocket;
use webrtc::ice::url::Url;

pub struct DtlsHandler {
    dtls_conn: DTLSConn,
}

impl DtlsHandler {
    pub async fn new(socket: UdpSocket, is_client: bool) -> Result<Self> {
        let dtls_config = Config {
            certificates: vec![], // Load your certificates here
            ..Default::default()
        };

        // Initialize ICE gatherer with the appropriate arguments
        let validated_servers = vec![Url::parse("stun:stun.l.google.com:19302").unwrap()];
        let gather_policy = RTCIceTransportPolicy::All;
        let setting_engine = Arc::new(SettingEngine::default());

        let ice_gatherer = RTCIceGatherer::new(validated_servers, gather_policy, setting_engine).await?;
        let ice_transport = RTCIceTransport::new(Arc::new(ice_gatherer));

        let conn: Arc<dyn Conn + Send + Sync> = Arc::new(socket);

        let dtls_conn = DTLSConn::new(conn, dtls_config, is_client, None).await?;

        Ok(DtlsHandler { dtls_conn })
    }

    pub async fn handshake(&mut self) -> Result<()> {
        let state = HandshakeState::default();
        self.dtls_conn.handshake(state).await?;
        Ok(())
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.dtls_conn.encrypt(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.dtls_conn.decrypt(data)
    }
}

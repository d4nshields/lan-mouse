use anyhow::Result;
use webrtc::dtls::config::Config;
use webrtc::dtls::conn::DTLSConn;
use webrtc::dtls::Error;
use webrtc::dtls::state::State;
use webrtc::ice_transport::ice_gatherer::RTCIceGatherer;
use webrtc::ice_transport::RTCIceTransport;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::ice_transport::
use webrtc::ice_transport::ice_role::RTCIceRole;
use std::sync::Arc;
use tokio::net::UdpSocket;
use webrtc::util::Conn;

pub struct DtlsHandler {
    dtls_conn: DTLSConn,
}

impl DtlsHandler {
    pub async fn new(socket: UdpSocket, is_client: bool) -> Result<Self> {
        let dtls_config = Config {
            certificates: vec![], // Load your certificates here
            ..Default::default()
        };

        // Initialize ICE gatherer
        let ice_server = RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        };
        let ice_gatherer = RTCIceGatherer::new(ice_server).await?;

        // Initialize ICE transport
        let ice_transport = RTCIceTransport::new(Arc::new(ice_gatherer));

        let conn: Arc<dyn Conn + Send + Sync> = Arc::new(socket);

        let dtls_conn = DTLSConn::new(conn, dtls_config, is_client, None).await?;

        Ok(DtlsHandler { dtls_conn })
    }

    pub async fn handshake(&self) -> Result<()> {
        self.dtls_conn.handshake().await?;
        Ok(())
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.dtls_conn.encrypt(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.dtls_conn.decrypt(data)
    }
}

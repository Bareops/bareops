use std::io::{Read, Write};

use ssh2::Session;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Error)]
pub(crate) enum SSHError {
    #[error("failed to allocate ssh channel to communicte with target: `{0}]`")]
    SSHChannel(ssh2::Error),
    #[error("failed to write request out via ssh: `{0}]`")]
    WritingRequest(std::io::Error),
    #[error("failed to read response from ssh: `{0}`")]
    ReadResponse(std::io::Error),
    #[error("failed to send reply on channel")]
    SendReply(),
}

pub(crate) struct SSHResponse {
    pub(crate) data: Vec<u8>,
}

pub(crate) struct SSHRequest {
    data: Vec<u8>,
    reply_chan: oneshot::Sender<SSHResponse>,
}

pub(crate) struct SSHConn {
    sess: Session,
    recv: mpsc::Receiver<SSHRequest>,
}

const SSH_CHAN_BUF_SIZE: usize = 128;

impl SSHConn {
    pub(crate) fn new(sess: Session) -> (Self, mpsc::Sender<SSHRequest>) {
        let (tx, rx) = mpsc::channel(SSH_CHAN_BUF_SIZE);
        (Self { sess, recv: rx }, tx)
    }

    pub(crate) fn recv_loop(&mut self) -> Result<(), SSHError> {
        let mut ssh_chan = self.sess.channel_session().map_err(SSHError::SSHChannel)?;
        while let Some(req) = self.recv.blocking_recv() {
            ssh_chan
                .write_all(&req.data)
                .map_err(SSHError::WritingRequest)?;

            let mut resp = Vec::new();
            let _read = ssh_chan
                .read_to_end(&mut resp)
                .map_err(SSHError::ReadResponse)?;

            req.reply_chan
                .send(SSHResponse { data: resp })
                .map_err(|_t| SSHError::SendReply())?;
        }
        Ok(())
    }
}

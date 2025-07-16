use crate::models::CandleSnapshotBody;
use tokio::sync::oneshot;

use crate::client;

pub type Response<T> = oneshot::Sender<T>;

pub enum ClientCommand {
    GetCandle {
        req: CandleSnapshotBody,
        resp: Response<client::Response>,
    },
}

pub enum DbCommand {
    WriteCandleData,
}

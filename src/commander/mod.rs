pub mod commands;

pub use commands::*;

//use futures::future;
//use tokio::{
//    sync::{mpsc, oneshot},
//    task::JoinHandle,
//};
//
//use crate::{client::client_manager::ClientManager, db::db_manager::DbManager};
//
//#[derive(Debug)]
//enum InfoCommand {
//    Get {
//        body: String,
//        resp_tx: Responder<String>,
//    },
//}
//
//#[derive(Debug)]
//enum DbCommand {
//    Write {
//        payload: String,
//        resp_tx: Responder<String>,
//    },
//}
//
//type Responder<T> = oneshot::Sender<T>;
//
//pub struct Manager {}
//
//impl Manager {
//    pub async fn sync() {
//        let workers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
//
//        let (h_tx, mut h_rx) = mpsc::channel::<InfoCommand>(10);
//        let (d_tx, mut d_rx) = mpsc::channel::<DbCommand>(10);
//
//        let handle_info_client_manager = tokio::spawn(async move {
//            println!("log - client manager: created client manager!");
//            let info_client = ClientManager {
//                name: "ClientManager Test".to_string(),
//            };
//
//            println!("log - client manager: waiting from worker");
//            while let Some(resp) = h_rx.recv().await {
//                println!("recieved request from worker");
//                use InfoCommand::*;
//
//                let _ = match resp {
//                    Get { body, resp_tx } => {
//                        let response = info_client.get(&body).await;
//                        resp_tx.send(response)
//                    }
//                };
//            }
//            println!("log - client manager: closing this manager");
//        });
//
//        let handle_db_client_manager = tokio::spawn(async move {
//            println!("log - db manager: created client manager!");
//            let db_manager = DbManager {
//                name: "DbManager Test".to_string(),
//            };
//
//            println!("log - db manager: waiting from worker");
//            while let Some(resp) = d_rx.recv().await {
//                println!("recieved request from worker");
//                use DbCommand::*;
//
//                let _ = match resp {
//                    Write { payload, resp_tx } => {
//                        let response = db_manager.write(&payload).await;
//                        resp_tx.send(response)
//                    }
//                };
//            }
//            println!("log - db manager: closing this manager");
//        });
//
//        let mut info_workers_handle: Vec<JoinHandle<()>> = Vec::new();
//
//        // Info workers
//        for worker in workers {
//            let w_tx = h_tx.clone();
//            let dw_tx = d_tx.clone();
//
//            let worker_hansdle = tokio::spawn(async move {
//                println!("worker node {worker} woriking...");
//                let (resp_tx, resp_rx) = oneshot::channel();
//                let cmd = InfoCommand::Get {
//                    body: format!("worker: {worker}"),
//                    resp_tx,
//                };
//
//                println!("worker node {worker} send request to client");
//                w_tx.send(cmd).await.unwrap();
//
//                let res = resp_rx.await.unwrap();
//                println!("woker node {worker} recieved from client: {res}");
//
//                let (db_resp_tx, db_resp_rx) = oneshot::channel();
//                let cmd = DbCommand::Write {
//                    payload: res,
//                    resp_tx: db_resp_tx,
//                };
//
//                println!("worker node {worker} send request to db");
//                dw_tx.send(cmd).await.unwrap();
//
//                let res = db_resp_rx.await.unwrap();
//                println!("worker node {worker} recieved from db: {res}");
//            });
//
//            info_workers_handle.push(worker_hansdle);
//        }
//
//        drop(h_tx);
//        drop(d_tx);
//
//        handle_info_client_manager.await.unwrap();
//        handle_db_client_manager.await.unwrap();
//        future::join_all(info_workers_handle).await;
//    }
//}

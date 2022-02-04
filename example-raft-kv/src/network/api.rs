use actix_web::HttpResponse;
use actix_web::post;
use actix_web::web;
use actix_web::web::Data;
use actix_web::Responder;
use openraft::raft::ClientWriteRequest;
use openraft::raft::EntryPayload;
use serde::Serialize;
use web::Json;

use crate::app::ExampleApp;
use crate::store::ExampleRequest;

/**
 * Application API
 *
 * This is where you place your application, you can use the example below to create your
 * API. The current implementation:
 *
 *  - `POST - /write` saves a value in a key and sync the nodes.
 *  - `GET - /read` attempt to find a value from a given key.
 */
#[post("/write")]
pub async fn write(app: Data<ExampleApp>, req: Json<ExampleRequest>) -> actix_web::Result<impl Responder> {
    // Redirect this call to the leader, this is required because learners are not allowed to write data to
    // other nodes. 
    if let Some(result) = redirect_to_leader_if_required(&app, &req).await {
        return result;
    }
    let request = ClientWriteRequest::new(EntryPayload::Normal(req.0));
    let response = app.raft.client_write(request).await;
    Ok(if response.is_ok() { 
        HttpResponse::Ok().finish()
    } else { 
        HttpResponse::BadRequest().body("Failed write number")
    })
}

async fn redirect_to_leader_if_required<T: Serialize>(
    app: &Data<ExampleApp>,
    req: &Json<T>
) -> Option<actix_web::Result<HttpResponse>> {

    let metrics = app.raft.metrics();
    let metrics = metrics.borrow();

    if let Some(current_leader) = metrics.current_leader {

        if metrics.id == current_leader {
            return None;
        }

        let state_machine = app.store.state_machine.read().await;
        let nodes = state_machine.nodes.clone();
        let result = match nodes.get(&current_leader) {
            Some(ip) => {
                Ok(app.client
                    .post(format!("http://{}", ip))
                    .json(&req)
                    .send()
                    .await
                    .map(|_| HttpResponse::Ok().finish())
                    .map_err(|_| HttpResponse::BadRequest().body("Failed to connect to leader node"))
                    .into_ok_or_err())
            }
            None => Ok(HttpResponse::BadRequest().body("Failed to add users"))
        };
        
        return Some(result);
    }
    None
}

#[post("/read")]
pub async fn read(app: Data<ExampleApp>, req: Json<String>) -> actix_web::Result<impl Responder> {
    let state_machine = app.store.state_machine.read().await;
    let key = req.0;
    let value = state_machine.data.get(&key).cloned();
    Ok(Json(value.unwrap_or_default()))
}

use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn create_router() -> Router {
    Router::new().route("/sources/:id", get(get_source_by_id))
    // .route("/sources", post(create_source))
    // .route("/sources", get(get_sources))
    // .route("/sources/:id", delete(delete_source_by_id))
    // .route("/sources/:id", put(update_source_by_id))
}

async fn get_source_by_id(
    user: TokenUser,
    Path(id): Path<String>,
) -> Result<Json<PublicCat>, Error> {
    let cat_id = to_object_id(id)?;
    let cat = Cat::find_one(doc! { "_id": cat_id, "user": &user.id }, None)
        .await?
        .map(PublicCat::from);

    let cat = match cat {
        Some(cat) => cat,
        None => {
            debug!("Cat not found, returning 404 status code");
            return Err(Error::not_found());
        }
    };

    debug!("Returning cat");
    Ok(Json(cat))
}

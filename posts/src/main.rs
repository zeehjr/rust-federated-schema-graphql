use async_graphql::http::GQLResponse;
use async_graphql::{
    EmptyMutation, EmptySubscription, QueryBuilder, Schema, ID,
};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

mod models;

// type PostsSchema = Schema<models::QueryRoot, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    let schema = Schema::new(models::Query, EmptyMutation, EmptySubscription);

    warp::serve(
        graphql(schema).and_then(|(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        }),
    )
    .run(([0, 0, 0, 0], 5052))
    .await;
}
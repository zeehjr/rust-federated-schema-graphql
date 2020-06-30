use async_graphql::{Object, SimpleObject, ID};

#[SimpleObject]
pub struct Post {
  id: ID,
  title: String,
  text: String
}

pub struct User {
  id: ID
}

#[Object(extends)]
impl User {
  #[field(external)]
  async fn id(&self) -> &ID {
    &self.id
  }

  async fn posts(&self) -> Vec<Post> {
    vec![
      Post {
        id: ID::from(0),
        title: "Hey".to_string(),
        text: "Post content".to_string()
      }
    ]
  }
}

pub struct Query;

#[Object(extends)]
impl Query {
  #[entity]
  async fn find_user_by_id(&self, id: ID) -> User {
      User { id }
  }
}
use async_graphql::{Object, SimpleObject, ID};

#[SimpleObject]
pub struct User {
  id: ID,
  name: String,
  email: String
}

pub struct Query;

#[Object(extends)]
impl Query {
  async fn users(&self) -> Vec<User> {
    vec![
      User {
        id: ID::from(0),
        name: "User one".to_string(),
        email: "user_one@domain.com".to_string()
      },
      User {
        id: ID::from(1),
        name: "User two".to_string(),
        email: "user_two@domain.com".to_string()
      }
    ]
  }

  #[entity]
  async fn find_user_by_id(&self, id: ID) -> User {
    let string_id = id.to_string();
    User {
      id: ID::from(&string_id),
      name: format!("{:?}", &string_id),
      email: format!("{:?}@domain.com", &string_id)
    }
  }
}
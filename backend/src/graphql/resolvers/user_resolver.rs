// Title: user_resolver
// Created by sorryu
// Date: 2024-11-11
// Description: User-related GraphQL resolver.

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-28 | Create UserQuery and UserMutation | sorryu
2024-11-29 | Add users function to get all users for UserQuery | sorryu
2024-11-29 | Add create_users function to create user in database for UserMutations | sorryu
2024-11-30 | Use Context to get database pool and Change map_err function to match syntax | sorryu

*/

use async_graphql::{ Context, Object, Result, Error as GqlError };
use log::error;

use crate::{
    services::auth_service::{ get_all_users, insert_user },
    models::user::{ UserData, UserResponse, UserRequest },
    utils::db_pool::DbPool, // type DbPool = Pool<Postgres>
};


#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserResponse>, GqlError> {
        let pool = match ctx.data::<DbPool>() {
            Ok(db_pool) => db_pool,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new("Database pool is not available in the GraphQL context."));
            }
        };
        
        let users_data = match get_all_users(pool).await {
            Ok(users_data) => users_data,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new(format!("Failed to fetch users from database: {}", err)));
            }
        };

        let users_response = users_data.into_iter().map(UserResponse::from).collect();
        Ok(users_response)
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, user_request: UserRequest) -> Result<UserResponse, GqlError> {
        let pool = match ctx.data::<DbPool>() {
            Ok(db_pool) => db_pool,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new("Database pool is not available in the GraphQl context."))?;
            }
        };
        
        let user_data: UserData = match user_request.try_into() {
            Ok(user_data) => user_data,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new(format!("Invalid Input: {}", err)));
            }
        };
        
        insert_user(pool, &user_data).await.map_err(|err| GqlError::new(format!("Failed to insert user: {}", err)))?;

        Ok(UserResponse::from(user_data))
    }
}
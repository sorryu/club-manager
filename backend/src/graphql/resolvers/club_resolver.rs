// Title: club_resolver
// Created by sorryu
// Date: 2024-11-11
// Description: Club-related GraphQL resolver.

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-29 | Create ClubQuery and ClubMutation | sorryu
2024-11-29 | Add clubs function to get all clubs for ClubQuery and Add create_clubs function to create clubs in database for ClubMutation | sorryu
2024-11-30 | Use Context to get database pool and Change map_err function to match syntax | sorryu

*/

use async_graphql::{ Context, Object, Result, Error as GqlError };
use log::error;

use crate::{
    services::club_service::{ get_all_clubs, insert_club },
    models::club::{ ClubData, ClubResponse, ClubRequest, convert_to_response },
    utils::db_pool::DbPool, // type DbPool = Pool<Postgres>
};

#[derive(Default)]
pub struct ClubQuery;

#[Object]
impl ClubQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<Option<ClubResponse>>, GqlError> {
        let pool = match ctx.data::<DbPool>() {
            Ok(db_pool) => db_pool,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new("Database pool is not available in the GraphQL context."));
            }
        };

        let clubs_data = match get_all_clubs(pool).await {
            Ok(clubs_data) => clubs_data,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new(format!("Failed to fetch users from database: {}", err)));
            }
        };

        let mut clubs_response: Vec<Option<ClubResponse>> = Vec::new();

        for club_data in clubs_data {
            clubs_response
                .push(convert_to_response(club_data, pool).await);
        }

        Ok(clubs_response)
    }
}

#[derive(Default)]
pub struct ClubMutation;

#[Object]
impl ClubMutation {
    async fn create_club(&self, ctx: &Context<'_>, club_request: ClubRequest) -> Result<ClubResponse, GqlError> {
        let pool = match ctx.data::<DbPool>() {
            Ok(db_pool) => db_pool,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new("Database pool is not available in the GraphQl context."));
            }
        };

        let club_data: ClubData = match club_request.try_into() {
            Ok(club_data) => club_data,
            Err(err) => {
                error!("Error: {:?}", err);
                return Err(GqlError::new(format!("Invalid Input: {}", err)));
            }
        };

        insert_club(pool, &club_data).await
            .map_err(|err| GqlError::new(format!("Failed to insert user: {}", err)))?;

        Ok(club_data.to_response(pool).await?)
    }
}
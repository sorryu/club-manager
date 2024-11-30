// Title: schema
// Created by sorryu
// Date: 2024-11-11
// Description: Define GraphQL schema related to users and clubs

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-28 | Create QueryRoot, MutationRoot and Merged to Schema | sorryu
2024-11-29 | Unified multiple structures into QueryRoot and MutationRoot structures, respectively | sorryu

*/

use async_graphql::{ Schema, MergedObject };
use crate::graphql::resolvers as rsv;
use rsv::{
    user_resolver::{ UserQuery, UserMutation },
    club_resolver::{ ClubQuery, ClubMutation },
};

// Unified Shema
#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, ClubQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation, ClubMutation);

// Final Schema
pub type AppSchema = Schema<QueryRoot, MutationRoot, ()>;
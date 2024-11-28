// Title: schema
// Created by sorryu
// Date: 2024-11-11
// Description: Define GraphQL schema related to users and clubs

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-28 | Create QueryRoot, MutationRoot and Merged to Schema | sorryu

*/

use async_graphql::{ Schema, MergedObject };
use crate::graphql::resolvers as rsv;
use rsv::{
    club_resolver,
    user_resolver
};

// User
#[derive(MergedObject, Default)]
pub struct UserQueryRoot; // To be mapped with resolvers from `user_resolver::UserQuery`

#[derive(MergedObject, Default)]
pub struct UserMutationRoot; // To be mapped with resolvers from `user_resolver::UserMutation`

// Club
#[derive(MergedObject, Default)]
pub struct ClubQueryRoot; // To be mapped with resolvers from `club_resolver::ClubQuery`

#[derive(MergedObject, Default)]
pub struct ClubMutationRoot; // To be mapped with resolvers from `club_resolver::ClubMutation`

// Unified Shema
#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQueryRoot, ClubQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutationRoot, ClubMutationRoot);

// Final Schema
pub type AppSchema = Schema<QueryRoot, MutationRoot, ()>;
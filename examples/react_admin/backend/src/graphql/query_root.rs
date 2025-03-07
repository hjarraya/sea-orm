use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};

lazy_static::lazy_static! { static ref CONTEXT: BuilderContext = BuilderContext::default(); }

pub fn schema(
    database: DatabaseConnection,
    depth: usize,
    complexity: usize,
) -> Result<Schema, SchemaError> {
    // Builder of Seaography query root
    let builder = Builder::new(&CONTEXT, database.clone());
    // Register SeaORM entities
    let builder = crate::models::_entities::register_entity_modules(builder);
    // Configure async GraphQL limits
    let schema = builder
        .schema_builder()
        // The depth is the number of nesting levels of the field
        .limit_depth(depth)
        // The complexity is the number of fields in the query
        .limit_complexity(complexity);
    // Finish up with including SeaORM database connection
    schema.data(database).finish()
}

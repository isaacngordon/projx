use crate::{client::schema, WaveAppClient};
use cynic::{GraphQlResponse, QueryBuilder};

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
struct Query {
    pub businesses: Option<BusinessConnection>,
}

#[derive(cynic::QueryFragment, Debug)]
struct BusinessConnection {
    pub edges: Vec<BusinessEdge>,
}

#[derive(cynic::QueryFragment, Debug)]
struct BusinessEdge {
    pub node: Option<Business>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Business {
    pub id: cynic::Id,
    pub name: String,
}

pub async fn get_businesses(client: &WaveAppClient) -> crate::error::Result<Vec<Business>> {
    let query = Query::build(());
    let res: GraphQlResponse<Query> = client.query_raw(query).await?;
    Ok(res
        .data
        .unwrap()
        .businesses
        .unwrap()
        .edges
        .into_iter()
        .filter_map(|edge| edge.node)
        .collect())
}

#[cfg(test)]
mod query_tests {
    use super::Query;
    use crate::{client::WaveAppClient, queries::businesses};
    use cynic::{GraphQlResponse, QueryBuilder};

    /// Build a WaveAppPayload from a Query and print it out
    #[tokio::test]
    async fn test_build_query() {
        let query = Query::build(());
        let client = WaveAppClient::default();
        let query_result: crate::error::Result<GraphQlResponse<Query>> =
            client.query_raw(query).await;
        println!("{:?}", query_result);
        assert!(query_result.is_ok());
    }

    /// Test get_businesses
    #[tokio::test]
    async fn test_get_businesses() {
        let client = WaveAppClient::default();
        let buz = businesses::get_businesses(&client).await;
        println!("{:?}", buz);
        assert!(buz.is_ok());
    }
}

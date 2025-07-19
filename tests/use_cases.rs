use std::sync::Arc;

use bipa::models::{LightningNodes, LightningNodesView};
use bipa::repository::in_memory::{InMemoryNodesRepository, MockMempoolAPIRepository};
use bipa::repository::{MempoolAPIRepository, NodesRepository};
use bipa::use_cases::fetch_last_nodes::FetchLastNodes;
use bipa::use_cases::get_last_nodes::GetLastNodes;
use tokio::sync::RwLock;

fn mock_api_repository(
    mock_api_responses: impl IntoIterator<Item = LightningNodes>,
) -> Arc<dyn MempoolAPIRepository> {
    Arc::new(MockMempoolAPIRepository(
        mock_api_responses
            .into_iter()
            .collect::<Vec<LightningNodes>>(),
    ))
}

fn mock_db_repository(
    mock_data: impl IntoIterator<Item = LightningNodesView>,
) -> Arc<dyn NodesRepository> {
    Arc::new(InMemoryNodesRepository(Arc::new(RwLock::new(
        mock_data.into_iter().collect(),
    ))))
}

#[tokio::test]
async fn test_basic_flow() {
    let api_node = LightningNodes {
        publicKey: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".into(),
        alias: "ACINQ".into(),
        capacity: 36010516297,
        firstSeen: 1522941222,
    };

    let mock_api = mock_api_repository([api_node.clone()]);
    let mock_db = mock_db_repository([]);

    let get_use_case = GetLastNodes {
        nodes_repository: mock_db.clone(),
    };
    assert_eq!(get_use_case.exec().await.unwrap().len(), 0);

    let fetch_use_case = FetchLastNodes {
        mempool_api_repository: mock_api,
        nodes_repository: mock_db.clone(),
    };

    let fetched_nodes = fetch_use_case.exec().await.unwrap();

    assert_eq!(fetched_nodes.len(), 1);
    assert_eq!(fetched_nodes[0], api_node);

    let nodes = GetLastNodes {
        nodes_repository: mock_db.clone(),
    }
    .exec()
    .await
    .unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].publicKey, api_node.publicKey);
    assert_eq!(nodes[0].alias, api_node.alias);
}


#[tokio::test]
async fn test_transform_values() {
    let api_nodes = [
        LightningNodes {
            publicKey: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".into(),
            alias: "ACINQ".into(),
            capacity: 36010516297,
            firstSeen: 1522941222,
        },
        LightningNodes {
            publicKey: "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226".into(),
            alias: "WalletOfSatoshi.com".into(),
            capacity: 15464503162,
            firstSeen: 1601429940,
        },
        LightningNodes {
            publicKey: "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226".into(),
            alias: "WalletOfSatoshi.com".into(),
            capacity: 0,
            firstSeen: 1601429940,
        },
    ];

    let mock_api = mock_api_repository(api_nodes.clone());
    let mock_db = mock_db_repository([]);

    let fetch_use_case = FetchLastNodes {
        mempool_api_repository: mock_api,
        nodes_repository: mock_db.clone(),
    };

    fetch_use_case.exec().await.unwrap();

    let final_nodes = GetLastNodes {
        nodes_repository: mock_db.clone(),
    }
    .exec()
    .await
    .unwrap();

    let expected_final_nodes = [
        LightningNodesView {
            publicKey: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".into(),
            alias: "ACINQ".into(),
            capacity: "360.10516297".into(),
            firstSeen: "2018-04-05T15:13:42Z".into(),
        },
        LightningNodesView {
            publicKey: "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226".into(),
            alias: "WalletOfSatoshi.com".into(),
            capacity: "154.64503162".into(),
            firstSeen: "2020-09-30T01:39:00Z".into(),
        },
        LightningNodesView {
            publicKey: "035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226".into(),
            alias: "WalletOfSatoshi.com".into(),
            capacity: "0".into(),
            firstSeen: "2020-09-30T01:39:00Z".into(),
        },
    ];

    assert_eq!(expected_final_nodes, final_nodes.as_slice());
}

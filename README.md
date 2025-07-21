# Lightnight Nodes API

Check it out here:  ```http://bipa.talison.dev/nodes``` (only http)

## Endpoints

### Essentials

| Method | Endpoint        | Description                 |
|--------|------------------|-----------------------------|
| GET    | `/nodes`     | Retrieve last nodes saved in Database            |
| PUT    | `/nodes`     | Fetch nodes in Mempool API and update as the last nodes in Database  (for testing purposes)              |

### Health

| Method | Endpoint               | Description                      |
|--------|------------------------|----------------------------------|
| GET   | `/healthz`          |    provide API health informations         |
| GET   | `/ready`           |  Used to check if the API is ready (for k8s probes)        |


---

## Build tools, Libraries & versions used

- **Language**: Rust 1.88
- **Database**: Redis 7.2
- **DevOps**: Gitlab CI/CD + Docker
- **Deploy**: Google Cloud + Kubernetes

#### Main Crates (*versions used in Cargo.toml*)

| Name | Version        |
|--------|------------------|
| Tokio    | Runtime for provide asynchronous Rust |
| Axum   | Framework to build the API |
| bb8    | Redis connection pool     |
| reqwest    | Integrate with Mempool API via HTTP  |
| Tracing    | Log & Tracing  |
| anyhow    | Easily and flexibly error handling |
| chrono    | Deal with dates  |
| async-trait    | Enable traits with async functions be used with `dyn`     |


---
# How it works?

The app has two modules independent: The API and the Worker. By default they 
are both enabled and they run in parallel. As a monolithic service.

![](https://res.cloudinary.com/dfjn94vg8/image/upload/v1753065109/Untitled-2024-06-06-1125_s1qawf.png)

However, you can disable one of these modules using the features flag.

```sh
cargo build --features disable_api
# or
cargo build --features disable_worker
```

Splitting the modules into separate services. Allowing a CQRS architecture.

![alt text](https://res.cloudinary.com/dfjn94vg8/image/upload/v1753065109/Untitled-2024-06-06-11d25_wsd53j.png)

With this approach, you can scale the api as the request volume grows. The worker, however should always be running as a single instance.

The Worker runs in a loop fetching the last nodes from the Mempool API and updating the database. The interval is configurable in the `.env` file. With `WORKER_INTERVAL_SECONDS` variable.

The API is a simple REST API that returns the last nodes saved in the database. The `PUT /nodes` make the same thing of the worker, it's used for testing purposes.

### Why Redis?

Given the requirements and description of the challenge, the database would be used to store
only the more recent information about Mempool API. Instead a history of nodes, with complex
query operations, or relations that could require a SQL database. The database looks like a 
cache store for the last nodes. Also as perfomance was a chosed priority for this project, Redis was chosen.

But, what about persistence? 

Even though Redis runs in-memory, it still [provide ways to store data in a persistent way](https://redis.io/docs/latest/operate/oss_and_stack/management/persistence/). 
The prefered way for this project is the point-in-time snapshots, or append-only log. 
However, if immediately persistence was needed, some SQL with ACID properties could be used with
addition to Redis.

## Steps to run the app

You need a redis instance running in your local machine.

You can run with Docker, running the container in Gitlab Container Registry, setting the `REDIS_URL` environment variable.
```sh
docker run -it -p 8080:8080 -e REDIS_URL='...' registry.gitlab.com/talis-fb/lightning-nodes-api/lightning-nodes:latest
```

There are images with api or worker disabled
```sh
docker run -it --rm -p 8080:8080 -e REDIS_URL='...' registry.gitlab.com/talis-fb/lightning-nodes-api/lightning-nodes:api-only
# or
docker run -it --rm -p 8080:8080 -e REDIS_URL='...' registry.gitlab.com/talis-fb/lightning-nodes-api/lightning-nodes:worker-only
```

You can check the environment variables availables to set in container and their default values in the `.env.default` file.

## What was the reason for your focus? What problems were you trying to solve?

I tried to focus in problemn this application could faced in the real world, if it was a real project. Be extensable, great to make new features, scalable, and performant.

I seek to use async and concurrency features to make the app performant and redis to provide faster queries. And I inspired in Clean Architecture.

I make the setup for a CQRS architecture. With the API and the Worker running in parallel. Enabled for cloud-native deployments. 


## How long did you spend on this project?
I took 5 days (1 day for planning, 3 days for implementation, 1 day for deployment).

| Date | Tasks |
|--- | --- |
|16 July          | Planning, searching for tools and ecossystem and making the architecture |
| 17 July - 19 July | Implementing |
| 20 July | Deploying and finishing |


## Did you make any trade-offs for this project? What would you have done differently with more time?

I didn't priorize testing. I still created some basic tests for main use cases. But I'd like to make integrations tests and boundary tests.

I'd like to make perfomance tests too. With focus on the API availability with high intensive load. How my async and concurrency implementation are working in real world scenarios.



## What do you think is the weakest part of your project?
As I commented above. Once I decided to use a NoSQL database, the application does not have a immediate persistence. Even if little, the data can be lost. Redis can crash and the point-in-time snapshot didn't runned in time. In worst scenario, the saved data in database would be delayed but some seconds.

Also, the service only store the last nodes in the database. Not a history of nodes. That's lightweight and simple, but it could be improved.

A option would be use a SQL database to store a history of nodes. Make the worker saving in both databases in the same transaction.

![](https://res.cloudinary.com/dfjn94vg8/image/upload/v1753065109/Untitled-2024-06ds-06-1125_f99owu.png)


Also, the nodes data is not stored as a binary format. Instead, it's stored as a string JSON.

This implys in some overhead to serialize and deserialize the data. In high intensive scenarios, this could be a bottleneck.

```rs
async fn append_nodes(&self, nodes: Vec<LightningNodesView>) -> anyhow::Result<()> {
    let mut conn = self.connection_pool.get().await?;

    // TODO: Use a binary format instead
    let json_nodes = serde_json::to_string(&nodes)?;

    conn.set::<&str, &str, ()>(REDIS_NODES_KEY, &json_nodes)
        .await?;

    Ok(())
}
```

## Deployment & CI/CD
The project is deployed in a GCP Kubernetes cluster. With pipelines to build and deploy docker images
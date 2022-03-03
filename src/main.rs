use scienceobjectsdb_rust_api::sciobjectsdb::sciobjsdb::api::storage::models::v1::Metadata;
use scienceobjectsdb_rust_api::sciobjectsdb::sciobjsdb::api::storage::services::v1::{CreateDatasetRequest, CreateDownloadLinkRequest, CreateObjectGroupRequest, CreateObjectRequest};
use scienceobjectsdb_rust_api::sciobjectsdb::sciobjsdb::api::storage::services::v1::dataset_objects_service_client::DatasetObjectsServiceClient;
use scienceobjectsdb_rust_api::sciobjectsdb::sciobjsdb::api::storage::services::v1::dataset_service_client::DatasetServiceClient;
use scienceobjectsdb_rust_api::sciobjectsdb::sciobjsdb::api::storage::services::v1::object_load_service_client::ObjectLoadServiceClient;
use tonic::metadata::{AsciiMetadataValue, AsciiMetadataKey};
use tonic::{Request, Status};
use tonic::service::Interceptor;
use tonic::transport::Endpoint;

const ENDPOINT: &str = "https://api.scienceobjectsdb.nfdi-dev.gi.denbi.de/swagger-ui/";
// Insert your project id here
const PROJECT_ID: &str = "TODO";
// Inser your API token here
const API_TOKEN: &str = "TODO";

/// The interceptor appends the `API_TOKEN` to each request
#[derive(Clone, Debug)]
struct APITokenInterceptor {
    key: AsciiMetadataKey,
    token: AsciiMetadataValue,
}

impl APITokenInterceptor {
    fn new(token: &'static str) -> APITokenInterceptor {
        let key = AsciiMetadataKey::from_static("api_token");
        let value = AsciiMetadataValue::from_static(token);
        APITokenInterceptor { key, token: value }
    }
}

impl Interceptor for APITokenInterceptor {
    // Append the API token to the given request
    fn call(&mut self, mut request: Request<()>) -> std::result::Result<Request<()>, Status> {
        request
            .metadata_mut()
            .append(self.key.clone(), self.token.clone());
        Ok(request)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a channel
    let channel = Endpoint::from_static(ENDPOINT).connect().await?;

    // Create an interceptor instance
    let interceptor = APITokenInterceptor::new(API_TOKEN);
    // Dataset client
    let mut dataset_client =
        DatasetServiceClient::with_interceptor(channel.clone(), interceptor.clone());
    // ObjectGroup client
    let mut object_group_client =
        DatasetObjectsServiceClient::with_interceptor(channel.clone(), interceptor.clone());
    // Object client
    let mut object_client = ObjectLoadServiceClient::with_interceptor(channel, interceptor);

    // Create a dataset
    let dataset_id = dataset_client
        .create_dataset(CreateDatasetRequest {
            name: "Example DataSet".to_string(),
            description: "An example dataset".to_string(),
            // The project id
            project_id: PROJECT_ID.to_string(),
            // Labels are simple key/value pairs that will later help to categorize the data
            labels: vec![],
            // Any additional meta-data (e.g., text,json) can be added to the data-set
            // Every meta datum has a unique key for later retrieval
            metadata: vec![Metadata {
                // The unique key
                key: "example".to_string(),
                // Again, labels can be assigned
                labels: vec![],
                // This is the actual meta-data which is simply a bound of bytes
                metadata: "{ \"purpose\": \"demo\" }".to_string().into_bytes(),
                ..Default::default()
            }],
        })
        .await?
        .into_inner()
        .id;

    // Create an ObjectGroup
    let object_group = object_group_client
        .create_object_group(CreateObjectGroupRequest {
            name: "Example ObjectGroup".to_string(),
            description: "An example ObjectGroup.".to_string(),
            // The dataset id
            dataset_id,
            // Again we can assign labels and meta-data
            labels: vec![],
            metadata: vec![],
            // Here, we define the objects (files) that are uploaded lateron.
            objects: vec![CreateObjectRequest {
                filename: "example.txt".to_string(),
                filetype: "txt".to_string(),
                // Again we can assign labels and meta-data
                labels: vec![],
                metadata: vec![],
                // The length of the file in bytes
                content_len: 7,
                // Ignored for now
                origin: None,
            }],
            // Whether to return the object upload-links after creating the ObjectGroup
            include_object_link: true,
            // Timestamp of creation
            generated: None,
            // A user defined uuid that is used to identify requests in chunked workloads - ignore for now
            uuid: "".to_string(),
        })
        .await?
        .into_inner();

    // Get upload link (they are in the same order as the objects were given in the previous call
    let upload_link = object_group.object_links.first().unwrap();

    // Upload the data via a PUT request
    let http_client = reqwest::Client::new();
    let upload_response = http_client
        .put(upload_link.link.as_str())
        .body("Example".to_string())
        .header("content-length", 7_i64)
        .send()
        .await?;

    println!("Upload finished - Status {}", upload_response.status());

    // And then download it again.
    // Request the link with the corresponding object id.
    let download_link = object_client
        .create_download_link(CreateDownloadLinkRequest {
            id: upload_link.object_id.clone(),
            // We could request a range of bytes if we wanted to
            range: None,
        })
        .await?
        .into_inner();

    // Perform the download
    let download_response = http_client
        .get(download_link.download_link.as_str())
        .send()
        .await?
        .error_for_status()?;

    println!("Upload finished - Status {}", download_response.status());
    println!("Content: {}", download_response.text().await?);

    // All went fine
    Ok(())
}

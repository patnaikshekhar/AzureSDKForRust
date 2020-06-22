use crate::prelude::*;
use crate::responses::ListDatabasesResponse;
use crate::ResourceType;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    cosmos_client: &'b CosmosClient<'a, CUB>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<String>,
    max_item_count: i32,
}

impl<'a, 'b, CUB> ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        cosmos_client: &'b CosmosClient<'a, CUB>,
    ) -> ListDatabasesBuilder<'a, 'b, CUB> {
        ListDatabasesBuilder {
            cosmos_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, 'b, CUB> CosmosClientRequired<'a, 'b, CUB> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn cosmos_client(&self) -> &'b CosmosClient<'a, CUB> {
        self.cosmos_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> UserAgentOption<'b> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB> ContinuationOption for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn continuation(&self) -> Option<&str> {
        match &self.continuation {
            Some(continuation) => Some(continuation),
            None => None,
        }
    }
}

impl<'a, 'b, CUB> MaxItemCountOption for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, 'b, CUB>;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListDatabasesBuilder {
            cosmos_client: self.cosmos_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, 'b, CUB>;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListDatabasesBuilder {
            cosmos_client: self.cosmos_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, 'b, CUB>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListDatabasesBuilder {
            cosmos_client: self.cosmos_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> ContinuationSupport for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, 'b, CUB>;

    fn with_continuation(self, continuation: String) -> Self::O {
        ListDatabasesBuilder {
            cosmos_client: self.cosmos_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
        }
    }
}

impl<'a, 'b, CUB> MaxItemCountSupport for ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDatabasesBuilder<'a, 'b, CUB>;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListDatabasesBuilder {
            cosmos_client: self.cosmos_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListDatabasesBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListDatabasesResponse, AzureError> {
        trace!("ListDatabasesBuilder::execute called");

        let request =
            self.cosmos_client
                .prepare_request("dbs", hyper::Method::GET, ResourceType::Databases);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let future_response = self.cosmos_client.hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListDatabasesResponse, AzureError>> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .with_continuation(continuation_token)
                                .execute()
                                .await
                        }
                        None => return None,
                    };

                    // the ? operator does not work in async move (yet?)
                    // so we have to resort to this boilerplate
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => return Some((Err(err), None)),
                    };

                    let continuation_token = match &response.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}

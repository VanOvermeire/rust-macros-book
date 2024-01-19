use aws_config::BehaviorVersion;
use aws_sdk_lambda::Client;
use aws_sdk_lambda::config::Region;
use aws_sdk_lambda::error::SdkError;
use aws_sdk_lambda::operation::add_permission::{AddPermissionError, AddPermissionOutput};
use aws_sdk_lambda::operation::create_function::{CreateFunctionError};
use aws_sdk_lambda::operation::create_function::builders::CreateFunctionFluentBuilder;
use aws_sdk_lambda::types::{FunctionCode, Runtime};

use crate::input::Lambda;

pub struct LambdaClient {
    client: Client,
}

impl LambdaClient {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("eu-west-1"))
            .load()
            .await;
        LambdaClient {
            client: Client::new(&config),
        }
    }

    pub async fn create_lambda(&self, lambda: &Lambda) -> Result<String, SdkError<CreateFunctionError>> {
        match self.client.get_function().function_name(&lambda.name).send().await {
            Err(_) => {
                // let us assume the error is caused by the function existing. If that's untrue, we will probably just error in the next call
                let builder = self.create_lambda_builder(&lambda);
                let output = builder.send().await?;
                Ok(output.function_arn()
                    .expect("a new function to have an ARN")
                    .to_string())
            },
            Ok(output) => {
                // additionally, we could check configuration and do an update if needed!
                eprintln!("function exists, skipping creation");
                Ok(output.configuration()
                    .expect("function output to have a configuration")
                    .function_arn()
                    .expect("an existing function to have an ARN")
                    .to_string())
            }
        }
    }

    fn create_lambda_builder(&self, lambda: &&Lambda) -> CreateFunctionFluentBuilder {
        let mut builder = self.client
            .create_function()
            .function_name(&lambda.name)
            .role("arn:aws:iam::262438358359:role/lambda-basic-permissions-role")
            .code(FunctionCode::builder()
                .s3_bucket("lambdas-sam-van-overmeire")
                .s3_key("example.zip")
                .build()
            )
            .runtime(Runtime::Nodejs18x)
            .handler("handler.handler");

        if let Some(time) = lambda.time {
            builder = builder.timeout(time.into());
        };
        if let Some(mem) = lambda.memory {
            builder = builder.memory_size(mem.into())
        };
        builder
    }

    pub async fn add_bucket_permission(&self, lambda: &Lambda, bucket_name: &str) -> Result<AddPermissionOutput, SdkError<AddPermissionError>> {
        self.client.add_permission()
            .function_name(&lambda.name)
            .principal("*")
            .statement_id("StatementId")
            .action("lambda:InvokeFunction")
            .source_arn(format!("arn:aws:s3:::{}", bucket_name))
            .send()
            .await
    }
}
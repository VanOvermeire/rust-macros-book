use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::create_bucket::{CreateBucketError};
use aws_sdk_s3::operation::put_bucket_notification_configuration::{PutBucketNotificationConfigurationError, PutBucketNotificationConfigurationOutput};
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration, Event, LambdaFunctionConfiguration, NotificationConfiguration};

use crate::input::Bucket;

pub struct S3Client {
    client: Client,
    region: String,
}

impl S3Client {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .load()
            .await;
        S3Client {
            client: Client::new(&config),
            region: "eu-west-1".to_string(),
        }
    }

    pub async fn create_bucket(&self, bucket: &Bucket) -> Result<(), SdkError<CreateBucketError>> {
        let bucket_output = self.client.list_buckets().send().await
            .expect("listing buckets to work"); // ignoring possible permission errors for simplicity
        let buckets = bucket_output.buckets();
        let bucket_names: Vec<String> = buckets.iter()
            .map(|b| b.name().expect("bucket to have a name").to_string())
            .collect();

        if bucket_names.contains(&bucket.name) {
            eprintln!("bucket exists, skipping creation");
        } else {
            let constraint = BucketLocationConstraint::from(self.region.as_str());
            let cfg = CreateBucketConfiguration::builder()
                .location_constraint(constraint)
                .build();

            self.client.create_bucket()
                .bucket(&bucket.name)
                .create_bucket_configuration(cfg)
                .send()
                .await?;
        };
        Ok(())
    }

    pub async fn link_bucket_with_lambda(&self, bucket: &Bucket, lambda_arn: &str) -> Result<PutBucketNotificationConfigurationOutput, SdkError<PutBucketNotificationConfigurationError>> {
        self.client.put_bucket_notification_configuration()
            .bucket(&bucket.name)
            .notification_configuration(NotificationConfiguration::builder()
                .lambda_function_configurations(LambdaFunctionConfiguration::builder()
                    .lambda_function_arn(lambda_arn)
                    .events(Event::from("s3:ObjectCreated:*"))
                    .build()
                    .expect("to create valid lambda function config")
                )
                .build())
            .send()
            .await
    }
}
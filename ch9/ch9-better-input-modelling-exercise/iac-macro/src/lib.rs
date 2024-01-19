mod input;
mod s3;
mod lambda;
mod errors;

use proc_macro::TokenStream;
use quote::{quote};
use syn::parse_macro_input;

use input::IacInput;
use crate::errors::IacError;
use crate::lambda::LambdaClient;
use crate::s3::S3Client;

async fn create_infra(iac_input: IacInput) -> Result<(), IacError> {
    let s3_client = S3Client::new().await;
    let lambda_client = LambdaClient::new().await;

    match iac_input {
        IacInput::Normal(bucket, lambda) => {
            if let Some(lambda) = lambda {
                eprintln!("creating lambda...");
                lambda_client.create_lambda(&lambda).await?;
            }
            if let Some(bucket) = bucket {
                eprintln!("creating bucket...");
                s3_client.create_bucket(&bucket).await?;
            }
        },
        IacInput::EventBucket(bucket, lambda) => {
            eprintln!("creating lambda...");
            let output = lambda_client.create_lambda(&lambda).await?;
            eprintln!("creating bucket...");
            s3_client.create_bucket(&bucket).await?;
            eprintln!("linking bucket and lambda by an event...");
            let lambda_arn = output.function_arn()
                .expect("creating a lambda should return its ARN");
            lambda_client.add_bucket_permission(&lambda, &bucket.name).await?;
            s3_client.link_bucket_with_lambda(&bucket, &lambda_arn).await?;
        }
    }
    Ok(())
}

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
    let ii: IacInput = parse_macro_input!(item);

    if ii.has_resources() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        match rt.block_on(create_infra(ii)) {
            Ok(_) => quote!().into(),
            Err(e) => e.into_compile_error()
        }
    } else {
        quote!().into()
    }
}

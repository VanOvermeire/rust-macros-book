use proc_macro::TokenStream;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use aws_sdk_lambda::error::SdkError;
use aws_sdk_lambda::operation::add_permission::AddPermissionError;
use aws_sdk_lambda::operation::create_function::CreateFunctionError;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::operation::create_bucket::CreateBucketError;
use aws_sdk_s3::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationError;
use proc_macro2::Span;

#[derive(Debug)]
pub enum IacError {
    BucketError(String),
    LambdaError(String),
    EventError(String),
}

impl IacError {
    pub fn into_compile_error(self) -> TokenStream {
        match self {
            IacError::BucketError(message) => {
                syn::Error::new(Span::call_site(), format!("bucket could not be created: {}", message))
                    .into_compile_error()
                    .into()
            }
            IacError::LambdaError(message) => {
                syn::Error::new(Span::call_site(), format!("lambda could not be created: {}", message))
                    .into_compile_error()
                    .into()
            }
            IacError::EventError(message) => {
                syn::Error::new(Span::call_site(), format!("event to link bucket and lambda could not be created: {}", message))
                    .into_compile_error()
                    .into()
            }
        }
    }
}

impl Error for IacError {}

impl Display for IacError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("retrieval error")
    }
}

macro_rules! generate_from_error {
    ($mine:expr, $aws:ty) => {
        impl From<SdkError<$aws>> for IacError {
            fn from(value: SdkError<$aws>) -> Self {
                let message = value.message()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "no message".to_string());
                $mine(message)
            }
        }
    }
}

generate_from_error!(IacError::BucketError,CreateBucketError);
generate_from_error!(IacError::LambdaError,CreateFunctionError);
generate_from_error!(IacError::EventError,PutBucketNotificationConfigurationError);
generate_from_error!(IacError::EventError,AddPermissionError);

// or manually implemented //

// impl From<SdkError<CreateBucketError>> for IacError {
//     fn from(value: SdkError<CreateBucketError>) -> Self {
//         let message = value.message()
//             .map(|v| v.to_string())
//             .unwrap_or_else(|| "no message".to_string());
//         IacError::BucketError(message)
//     }
// }
//
// impl From<SdkError<CreateFunctionError>> for IacError {
//     fn from(value: SdkError<CreateFunctionError>) -> Self {
//         let message = value.message()
//             .map(|v| v.to_string())
//             .unwrap_or_else(|| "no message".to_string());
//         IacError::LambdaError(message)
//     }
// }
//
// impl From<SdkError<PutBucketNotificationConfigurationError>> for IacError {
//     fn from(value: SdkError<PutBucketNotificationConfigurationError>) -> Self {
//         let message = value.message()
//             .map(|v| v.to_string())
//             .unwrap_or_else(|| "no message".to_string());
//         IacError::EventError(message)
//     }
// }
//
// impl From<SdkError<AddPermissionError>> for IacError {
//     fn from(value: SdkError<AddPermissionError>) -> Self {
//         let message = value.message()
//             .map(|v| v.to_string())
//             .unwrap_or_else(|| "no message".to_string());
//         IacError::EventError(message)
//     }
// }
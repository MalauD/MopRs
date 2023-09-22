use log::info;
use once_cell::sync::OnceCell;
use s3::{creds::Credentials, error::S3Error, Bucket, BucketConfiguration, Region};
use tokio::sync::Mutex;

pub struct S3Client {
    bucket: Bucket,
}

pub struct S3Config {
    pub s3_url: String,
    pub s3_region: String,
}

impl S3Client {
    pub fn get_bucket(&self) -> &Bucket {
        &self.bucket
    }
}

static S3: OnceCell<S3Client> = OnceCell::new();
static S3_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub async fn get_s3(s3_config: Option<S3Config>) -> &'static S3Client {
    if let Some(c) = S3.get() {
        return c;
    }
    let initializing_mutex = S3_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));

    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        let s3_config = s3_config.unwrap();
        let s3_url = s3_config.s3_url;
        let s3_region = s3_config.s3_region;

        let bucket = match Bucket::create_with_path_style(
            "moprs",
            Region::Custom {
                region: s3_region.clone(),
                endpoint: s3_url.clone(),
            },
            Credentials::from_env_specific(None, None, None, None).unwrap(),
            BucketConfiguration::private(),
        )
        .await
        {
            Ok(b) => b.bucket,
            Err(S3Error::Http(409, _)) => Bucket::new(
                "moprs",
                Region::Custom {
                    region: s3_region.clone(),
                    endpoint: s3_url.clone(),
                },
                Credentials::from_env_specific(None, None, None, None).unwrap(),
            )
            .unwrap()
            .with_path_style(),
            Err(e) => {
                panic!("Failed to create bucket: {}", e)
            }
        };
        if S3.set(S3Client { bucket }).is_ok() {
            info!(target: "mop-rs::s3", "Connected to s3");
            *initialized = true;
        }
    }
    drop(initialized);
    S3.get().unwrap()
}

//! [`ListBuckets`](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html)

use crate::error::S3Result;
use crate::output::{wrap_output, S3Output};
use crate::utils::{ResponseExt, XmlWriterExt};
use crate::{BoxStdError, Response};

use crate::dto::{ListBucketsError, ListBucketsOutput, ListBucketsRequest};

/// extract operation request
pub fn extract() -> Result<ListBucketsRequest, BoxStdError> {
    Ok(ListBucketsRequest)
}

impl S3Output for ListBucketsOutput {
    fn try_into_response(self) -> S3Result<Response> {
        wrap_output(|res| {
            res.set_xml_body(4096, |w| {
                w.stack("ListBucketsOutput", |w| {
                    w.opt_stack("Buckets", self.buckets, |w, buckets| {
                        for bucket in buckets {
                            w.stack("Bucket", |w| {
                                w.opt_element("CreationDate", bucket.creation_date)?;
                                w.opt_element("Name", bucket.name)
                            })?;
                        }
                        Ok(())
                    })?;

                    w.opt_stack("Owner", self.owner, |w, owner| {
                        w.opt_element("DisplayName", owner.display_name)?;
                        w.opt_element("ID", owner.id)
                    })?;
                    Ok(())
                })
            })
        })
    }
}

impl S3Output for ListBucketsError {
    fn try_into_response(self) -> S3Result<Response> {
        match self {}
    }
}

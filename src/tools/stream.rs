use std::io;

use actix_web::web;

use actix_web::error::PayloadError;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct PayloadStream {
    pub payload: web::Payload,
}

unsafe impl Send for PayloadStream {}
unsafe impl Sync for PayloadStream {}

impl Stream for PayloadStream {
    type Item = Result<web::Bytes, io::Error>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.payload).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(res))) => Poll::Ready(Some(Ok(res))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(match e {
                PayloadError::Incomplete(o) => match o {
                    Some(e) => e,
                    None => io::Error::new(io::ErrorKind::Other, "PayloadError::Incomplete None"),
                },
                PayloadError::EncodingCorrupted => {
                    io::Error::new(io::ErrorKind::Other, "PayloadError::EncodingCorrupted")
                }
                PayloadError::Overflow => {
                    io::Error::new(io::ErrorKind::Other, "PayloadError::Overflow")
                }
                PayloadError::UnknownLength => {
                    io::Error::new(io::ErrorKind::Other, "PayloadError::UnknownLength")
                }
                PayloadError::Http2Payload(e) => io::Error::new(
                    io::ErrorKind::Other,
                    format!("PayloadError::Http2Payload {:?}", e),
                ),
                PayloadError::Io(e) => e,
                _ => io::Error::new(io::ErrorKind::Other, "PayloadError::UnknownError"),
            }))),
            Poll::Ready(None) => Poll::Ready(None),
        }
    }
}
pub struct ResponseStream<T>
where
    T: Stream<Item = reqwest::Result<bytes::Bytes>> + Unpin,
{
    pub stream: T,
}

impl<T> Stream for ResponseStream<T>
where
    T: Stream<Item = reqwest::Result<bytes::Bytes>> + Unpin,
{
    type Item = Result<web::Bytes, actix_web::Error>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.stream).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(res))) => {
                Poll::Ready(Some(Ok(unsafe { std::mem::transmute(res) })))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{:?}", e),
            )
            .into()))),
            Poll::Ready(None) => Poll::Ready(None),
        }
    }
}

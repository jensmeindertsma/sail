use axum::{body::Body as AxumBody, Error as AxumError};
use core::fmt;
use http_body_util::Full;
use hyper::{
    body::{Bytes, Frame, Incoming, SizeHint},
    Error as HyperError,
};
use std::{
    error::Error,
    fmt::Formatter,
    pin::Pin,
    task::{Context, Poll},
};

pub enum Body {
    Axum(AxumBody),
    Complete(Full<Bytes>),
    Incoming(Incoming),
}

impl hyper::body::Body for Body {
    type Data = Bytes;
    type Error = BodyError;

    fn poll_frame(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.get_mut() {
            Self::Axum(body) => {
                let pinned_body = Pin::new(body);
                pinned_body.poll_frame(context).map_err(BodyError::Axum)
            }
            Self::Complete(body) => {
                let pinned_body = Pin::new(body);
                pinned_body
                    .poll_frame(context)
                    .map_err(|_| BodyError::Infallible)
            }
            Self::Incoming(body) => {
                let pinned_body = Pin::new(body);
                pinned_body.poll_frame(context).map_err(BodyError::Hyper)
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        match self {
            Body::Axum(body) => body.is_end_stream(),
            Body::Complete(body) => body.is_end_stream(),
            Body::Incoming(body) => body.is_end_stream(),
        }
    }

    fn size_hint(&self) -> SizeHint {
        match self {
            Body::Axum(body) => body.size_hint(),
            Body::Complete(body) => body.size_hint(),
            Body::Incoming(body) => body.size_hint(),
        }
    }
}

#[derive(Debug)]
pub enum BodyError {
    Axum(AxumError),
    Hyper(HyperError),
    Infallible,
}

impl fmt::Display for BodyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Axum(error) => write!(f, "axum body error: {error}"),
            Self::Hyper(error) => write!(f, "hyper body error: {error}"),
            Self::Infallible => write!(f, "this should never happen"),
        }
    }
}

impl Error for BodyError {}

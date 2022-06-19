//! Provides a trait for a datastore that responds to a key with an actix_web::HttpResponse.

pub use actix_web::HttpResponse;

pub trait HttpState {
    type Key;
    
    fn get(&self, key: Self::Key) -> HttpResponse;
}

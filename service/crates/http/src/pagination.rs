use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use mire_model::Page;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pagination {
    /// The offset of the first record in the overall resultset
    pub offset: usize,
    /// The total number of records in the resultset
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct PageModel<D> {
    pub data: Vec<D>,
    pub pagination: Pagination,
}

impl<I, D> From<Page<I>> for PageModel<D>
where
    D: Serialize,
    I: Into<D>,
{
    fn from(input: Page<I>) -> Self {
        Self {
            pagination: Pagination {
                offset: input.offset,
                total: input.total,
            },
            data: input.data.into_iter().map(Into::into).collect(),
        }
    }
}

impl<D> From<&PageModel<D>> for HttpResponse
where
    D: Serialize,
{
    fn from(page: &PageModel<D>) -> Self {
        Self::Ok().json(page)
    }
}

impl<D> From<PageModel<D>> for HttpResponse
where
    D: Serialize,
{
    fn from(page: PageModel<D>) -> Self {
        Self::from(&page)
    }
}

impl<D> Responder for PageModel<D>
where
    D: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        ready(Ok(self.into()))
    }
}

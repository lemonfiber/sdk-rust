/// `GET /api/news`, answered with `news-items`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct News {}

impl crate::Answer for super::News {
    const TAGGED: bool = false;
}

impl crate::Read for News {
    type Answer = super::News;
    const PATH: &'static str = "/api/news";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["news-items"];
}

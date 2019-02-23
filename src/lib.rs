use std::path::Path;
use reqwest::multipart::Form;

pub mod maybe;
pub mod models;

pub struct SaucenaoRequest {
    output_type: OutputType,
    api_key: String,
    testmode: i64,
    db: DbChoose,
    numres: u64,
    input: ImageSource,
}

impl SaucenaoRequest {
    pub fn execute(&self) -> Result<models::Response, failure::Error> {

        let client = reqwest::Client::new();
        let query = client.get("https://saucenao.com/search.php");

        let db = match self.db {
            DbChoose::Db(i) => ("db", i),
            DbChoose::DbMask(i) => ("dbmask", i),
            DbChoose::DbMaskI(i) => ("dbmaski", i),
        };

        let query = query
            .query(&("output_type", (self.output_type.clone() as u8)))
            .query(&("api_key", &self.api_key))
            .query(&("testmode", self.testmode))
            .query(&db)
            .query(&("numres", self.numres));

        let query = match self.input {
            ImageSource::Image(ref path) => {
                let form = Form::new().file("image.png", Path::new(path))?;
                query.multipart(form)
            },
            ImageSource::Url(ref url) => query.query(&("url", url.clone())),
        };

        println!("{:?}", query.)

        let res = query
            .send()?
            .json()?;
        Ok(res)
    }
}

#[derive(Clone)]
#[repr(u8)]
pub enum OutputType {
    Html,
    Xml,
    Json,
}

pub enum DbChoose {
    Db(u64),
    DbMask(u64),
    DbMaskI(u64),
}

pub enum ImageSource {
    Image(String),
    Url(String),
}

pub struct SaucenaoRequestBuilder {
    api_key: String,
    input: ImageSource,
    db: DbChoose,
    numres: u64,
}

impl SaucenaoRequestBuilder {
    fn new(api_key: impl Into<String>, input: ImageSource) -> Self {
        SaucenaoRequestBuilder {
            api_key: api_key.into(),
            input: input,
            db: DbChoose::Db(999),
            numres: 16,
        }
    }

    pub fn from_url(api_key: impl Into<String>, url: impl Into<String>) -> Self {
        Self::new(api_key, ImageSource::Url(url.into()))
    }

    pub fn from_file(api_key: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(api_key, ImageSource::Image(path.into()))
    }

    pub fn with_db_mask(mut self, mask: u64) -> Self {
        self.db = DbChoose::DbMask(mask);
        self
    }

    // TODO: нормальное имя
    pub fn with_db_mask_i(mut self, mask: u64) -> Self {
        self.db = DbChoose::DbMaskI(mask);
        self
    }

    pub fn with_numres(mut self, numres: u64) -> Self {
        self.numres = numres;
        self
    }

    pub fn build(self) -> SaucenaoRequest {

        SaucenaoRequest {
            output_type: OutputType::Json,
            api_key: self.api_key,
            testmode: 1,
            db: self.db,
            numres: self.numres,
            input: self.input
        }
    }
}
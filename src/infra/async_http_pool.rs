#![allow(non_snake_case)]

use hyper::client::HttpConnector;
use hyper::{Body, Client, HeaderMap, Method, Response, StatusCode};
use hyper_tls::HttpsConnector;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::error::Elapsed;

pub struct HttpPool {
    client: Client<HttpsConnector<HttpConnector>>,
    buf: Mutex<VecDeque<HttpHandle>>,
}

impl HttpPool {
    pub fn new() -> Self {
        let tls_conn = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(tls_conn);
        Self {
            client,
            buf: Mutex::new(VecDeque::new()),
        }
    }
    pub fn new_init() -> (Arc<HttpPool>, HttpClient) {
        let p = Arc::new(Self::new());
        let client = HttpClient::new(p.clone());
        (p, client)
    }
    pub async fn task_handle(self: Arc<Self>) {
        let mut need_sleep = false;
        loop {
            if need_sleep {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            let mut lock = match self.buf.lock() {
                Ok(o) => o,
                Err(err) => {
                    println!("HttpPool lock buf error:{}", err);
                    need_sleep = true;
                    continue;
                }
            };
            need_sleep = lock.is_empty();
            while let Some(i) = lock.pop_front() {
                let client = self.clone();
                tokio::spawn(async move {
                    let function = async move {
                        let mut req = hyper::Request::builder().method(i.method).uri(i.uri);
                        for (k, v) in i.headers.into_iter() {
                            req = req.header(k, v);
                        }
                        let req = if let Some(body) = i.body {
                            req.body(body)?
                        } else {
                            req.body(Body::empty())?
                        };
                        let resp = client.client.request(req).await?;
                        Ok(resp)
                    };

                    let res: Result<anyhow::Result<Response<Body>>, Elapsed> =
                        tokio::time::timeout(i.timeout, function).await;

                    let result = match res {
                        Ok(Ok(resp)) => {
                            let status = resp.status();
                            let headers = resp.headers().clone();
                            match hyper::body::to_bytes(resp.into_body()).await {
                                Ok(bs) => Ok((status, headers, bs.to_vec())),
                                Err(err) => Err(anyhow::anyhow!("read body:{}", err)),
                            }
                        }
                        Ok(Err(err)) => Err(anyhow::anyhow!("http request send:{}", err)),
                        Err(err) => Err(anyhow::anyhow!("timeout:{}", err)),
                    };

                    (i.handle)(result);
                });
            }
        }
    }
    fn add_http_task(&self, hp: HttpHandle) {
        let mut lock = self.buf.lock().expect("HttpPool.add_http_task lock error");
        lock.push_back(hp);
    }
}

pub struct HttpHandle {
    method: Method,
    uri: String,
    timeout: Duration,
    headers: HashMap<String, String>,
    body: Option<Body>,
    handle:
        Box<dyn FnOnce(anyhow::Result<(StatusCode, HeaderMap, Vec<u8>)>) + Send + Sync + 'static>,
}

pub trait HttpHandleBuilder {
    fn build(self, hh: &mut HttpHandle);
}

impl HttpHandleBuilder for () {
    fn build(self, _hh: &mut HttpHandle) {}
}
impl HttpHandleBuilder for Method {
    fn build(self, hh: &mut HttpHandle) {
        hh.method = self;
    }
}
impl HttpHandleBuilder for String {
    fn build(self, hh: &mut HttpHandle) {
        hh.uri = self;
    }
}
impl HttpHandleBuilder for &str {
    fn build(self, hh: &mut HttpHandle) {
        hh.uri = self.to_string();
    }
}
impl HttpHandleBuilder for HashMap<String, String> {
    fn build(self, hh: &mut HttpHandle) {
        for (k, v) in self.into_iter() {
            hh.headers.insert(k, v);
        }
    }
}
impl HttpHandleBuilder for Duration {
    fn build(self, hh: &mut HttpHandle) {
        hh.timeout = self;
    }
}
impl HttpHandleBuilder for Body {
    fn build(self, hh: &mut HttpHandle) {
        hh.body = Some(self);
    }
}
impl HttpHandleBuilder for Vec<u8> {
    fn build(self, hh: &mut HttpHandle) {
        hh.body = Some(Body::from(self));
    }
}
impl<T> HttpHandleBuilder for T
where
    T: FnOnce(anyhow::Result<(StatusCode, HeaderMap, Vec<u8>)>) + Send + Sync + 'static,
{
    fn build(self, hh: &mut HttpHandle) {
        hh.handle = Box::new(self);
    }
}

impl Default for HttpHandle {
    fn default() -> Self {
        HttpHandle {
            method: Method::GET,
            uri: String::new(),
            timeout: Duration::from_secs(30),
            headers: HashMap::new(),
            body: None,
            handle: Box::new(http_handle_default),
        }
    }
}
fn http_handle_default(_res: anyhow::Result<(StatusCode, HeaderMap, Vec<u8>)>) {}

macro_rules! from_http_handle_template {
    () => {
    impl From<()> for HttpHandle {
        fn from(_value: ()) -> Self {
            HttpHandle::default()
        }
    }
    };
    ($($t:tt),*)=>{
        impl<$($t,)*> From<($($t,)*)> for HttpHandle
        where $($t:HttpHandleBuilder,)*
        {
            fn from(($($t,)*): ($($t,)*)) -> Self {
                let mut hd = HttpHandle::default();
                $($t.build(&mut hd);)*
                hd
            }
        }
    }
}

from_http_handle_template!();
from_http_handle_template!(T0);
from_http_handle_template!(T0, T1);
from_http_handle_template!(T0, T1, T2);
from_http_handle_template!(T0, T1, T2, T3);
from_http_handle_template!(T0, T1, T2, T3, T4);
from_http_handle_template!(T0, T1, T2, T3, T4, T5);
from_http_handle_template!(T0, T1, T2, T3, T4, T5, T6);
from_http_handle_template!(T0, T1, T2, T3, T4, T5, T6, T7);
from_http_handle_template!(T0, T1, T2, T3, T4, T5, T6, T7, T8);

pub struct HttpClient {
    pool: Arc<HttpPool>,
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl HttpClient {
    pub fn new(pool: Arc<HttpPool>) -> Self {
        Self { pool }
    }

    pub fn request<Req: Into<HttpHandle>>(&self, r: Req) {
        self.pool.add_http_task(r.into());
    }

    #[allow(dead_code)]
    pub fn get<Uri: AsRef<str>>(
        &self,
        uri: Uri,
        headers: HashMap<String, String>,
        handle: impl FnOnce(anyhow::Result<(StatusCode, HeaderMap, Vec<u8>)>) + Send + Sync + 'static,
    ) {
        self.request((Method::GET, headers, uri.as_ref(), handle));
    }
    #[allow(dead_code)]
    pub fn post<Uri: AsRef<str>, B: Into<Body>>(
        &self,
        uri: Uri,
        headers: HashMap<String, String>,
        body: B,
        handle: impl FnOnce(anyhow::Result<(StatusCode, HeaderMap, Vec<u8>)>) + Send + Sync + 'static,
    ) {
        self.request((Method::GET, headers, uri.as_ref(), body.into(), handle));
    }
}

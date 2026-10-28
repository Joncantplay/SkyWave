// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Minimal HTTP/1.1 client for radio streams.
//!
//! Written for this use case instead of using a general client: Shoutcast servers answer
//! with `ICY 200 OK`, streams never end, and a stopped radio must be able to cut the
//! socket from another thread.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

/// Who the request says it comes from.
#[derive(Clone, Copy)]
pub enum Agent {
    /// Streams and station websites: an ordinary player, no mod or game name.
    Player,
    /// MusicBrainz and the Cover Art Archive want apps to name themselves and a contact.
    App,
}

impl Agent {
    fn header(self) -> &'static str {
        match self {
            Agent::Player => "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            Agent::App => concat!("SkyWave/", env!("CARGO_PKG_VERSION"), " (+https://github.com/Joncantplay/SkyWave)"),
        }
    }
}
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_REDIRECTS: usize = 8;
const MAX_HEAD_BYTES: usize = 64 * 1024;

pub fn other_error(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message.into())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Url {
    pub https: bool,
    pub host: String,
    pub port: u16,
    /// Path including query, always starting with `/`.
    pub path: String,
}

fn strip_prefix_ignore_case<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() >= prefix.len() && s.is_char_boundary(prefix.len()) && s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

impl Url {
    pub fn parse(input: &str) -> Result<Url, String> {
        let s = input.trim();
        let (https, rest) = if let Some(r) = strip_prefix_ignore_case(s, "https://") {
            (true, r)
        } else if let Some(r) = strip_prefix_ignore_case(s, "http://") {
            (false, r)
        } else {
            return Err(format!("unsupported URL '{s}' (only http and https)"));
        };
        let split = rest.find(['/', '?', '#']).unwrap_or(rest.len());
        let (authority, tail) = rest.split_at(split);
        let tail = tail.split('#').next().unwrap_or("");
        let path = if tail.is_empty() {
            "/".to_string()
        } else if tail.starts_with('?') {
            format!("/{tail}")
        } else {
            tail.to_string()
        };
        let authority = authority.rsplit('@').next().unwrap_or("");
        let default_port = if https { 443 } else { 80 };
        let (host, port) = if let Some(inner) = authority.strip_prefix('[') {
            let end = inner.find(']').ok_or_else(|| format!("invalid host in '{s}'"))?;
            let port = match inner[end + 1..].strip_prefix(':') {
                Some(p) => p.parse().map_err(|_| format!("invalid port in '{s}'"))?,
                None => default_port,
            };
            (inner[..end].to_string(), port)
        } else {
            match authority.rfind(':') {
                Some(i) => {
                    let port_text = &authority[i + 1..];
                    let port = if port_text.is_empty() {
                        default_port
                    } else {
                        port_text.parse().map_err(|_| format!("invalid port in '{s}'"))?
                    };
                    (authority[..i].to_string(), port)
                }
                None => (authority.to_string(), default_port),
            }
        };
        if host.is_empty() {
            return Err(format!("missing host in '{s}'"));
        }
        Ok(Url { https, host: host.to_ascii_lowercase(), port, path })
    }

    fn default_port(&self) -> u16 {
        if self.https {
            443
        } else {
            80
        }
    }

    pub fn host_header(&self) -> String {
        let host = if self.host.contains(':') { format!("[{}]", self.host) } else { self.host.clone() };
        if self.port == self.default_port() {
            host
        } else {
            format!("{host}:{}", self.port)
        }
    }

    /// Path without the query string.
    pub fn path_only(&self) -> &str {
        self.path.split('?').next().unwrap_or("/")
    }

    /// Resolves a (possibly relative) reference against this URL.
    pub fn join(&self, reference: &str) -> Result<Url, String> {
        let r = reference.trim();
        if strip_prefix_ignore_case(r, "http://").is_some() || strip_prefix_ignore_case(r, "https://").is_some() {
            return Url::parse(r);
        }
        if let Some(rest) = r.strip_prefix("//") {
            let scheme = if self.https { "https" } else { "http" };
            return Url::parse(&format!("{scheme}://{rest}"));
        }
        let r = r.split('#').next().unwrap_or("");
        let mut url = self.clone();
        if r.is_empty() {
            return Ok(url);
        }
        if r.starts_with('/') {
            url.path = normalize_path(r);
        } else if r.starts_with('?') {
            url.path = format!("{}{r}", self.path_only());
        } else {
            let base = self.path_only();
            let dir = &base[..base.rfind('/').map_or(0, |i| i + 1)];
            url.path = normalize_path(&format!("{dir}{r}"));
        }
        if !url.path.starts_with('/') {
            url.path.insert(0, '/');
        }
        Ok(url)
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheme = if self.https { "https" } else { "http" };
        write!(f, "{scheme}://{}{}", self.host_header(), self.path)
    }
}

/// Removes `.` and `..` segments from a path, leaving the query untouched.
fn normalize_path(path: &str) -> String {
    let (p, query) = match path.find('?') {
        Some(i) => (&path[..i], &path[i..]),
        None => (path, ""),
    };
    let mut out: Vec<&str> = Vec::new();
    let segments: Vec<&str> = p.split('/').collect();
    let last = segments.len().saturating_sub(1);
    for (i, seg) in segments.iter().enumerate() {
        match *seg {
            "." => {
                if i == last {
                    out.push("");
                }
            }
            ".." => {
                if out.len() > 1 {
                    out.pop();
                }
                if i == last {
                    out.push("");
                }
            }
            s => out.push(s),
        }
    }
    let mut joined = out.join("/");
    if !joined.starts_with('/') {
        joined.insert(0, '/');
    }
    format!("{joined}{query}")
}

/// Lets another thread cut a running request.
pub struct Abort {
    aborted: AtomicBool,
    socket: Mutex<Option<TcpStream>>,
}

impl Abort {
    pub fn new() -> Abort {
        Abort { aborted: AtomicBool::new(false), socket: Mutex::new(None) }
    }

    pub fn abort(&self) {
        self.aborted.store(true, Ordering::SeqCst);
        if let Some(socket) = self.socket.lock().unwrap_or_else(|e| e.into_inner()).take() {
            let _ = socket.shutdown(Shutdown::Both);
        }
    }

    pub fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::SeqCst)
    }

    fn attach(&self, socket: &TcpStream) {
        if let Ok(clone) = socket.try_clone() {
            *self.socket.lock().unwrap_or_else(|e| e.into_inner()) = Some(clone);
        }
        if self.is_aborted() {
            let _ = socket.shutdown(Shutdown::Both);
        }
    }
}

enum Connection {
    Plain(TcpStream),
    Tls(Box<native_tls::TlsStream<TcpStream>>),
}

impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Connection::Plain(s) => s.read(buf),
            Connection::Tls(s) => s.read(buf),
        }
    }
}

impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Connection::Plain(s) => s.write(buf),
            Connection::Tls(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Connection::Plain(s) => s.flush(),
            Connection::Tls(s) => s.flush(),
        }
    }
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    /// Address that answered, after redirects.
    pub url: Url,
    pub body: Box<dyn Read + Send>,
}

impl Response {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }

    /// Media type in lower case without parameters, e.g. `audio/mpeg`.
    pub fn content_type(&self) -> String {
        self.header("content-type")
            .map(|v| v.split(';').next().unwrap_or("").trim().to_ascii_lowercase())
            .unwrap_or_default()
    }
}

/// Sends a GET request as an ordinary player and follows redirects.
pub fn get(url: &Url, abort: &Abort) -> io::Result<Response> {
    get_as(url, abort, Agent::Player)
}

/// Like [`get`], with a chosen user agent.
pub fn get_as(url: &Url, abort: &Abort, agent: Agent) -> io::Result<Response> {
    let mut current = url.clone();
    for _ in 0..MAX_REDIRECTS {
        let response = request(&current, abort, agent)?;
        if matches!(response.status, 301 | 302 | 303 | 307 | 308) {
            if let Some(location) = response.header("location") {
                current = current.join(location).map_err(other_error)?;
                continue;
            }
        }
        return Ok(response);
    }
    Err(other_error("too many redirects"))
}

fn connect(url: &Url, abort: &Abort) -> io::Result<TcpStream> {
    let addresses: Vec<SocketAddr> = (url.host.as_str(), url.port).to_socket_addrs()?.collect();
    let mut last_error = None;
    for address in addresses {
        if abort.is_aborted() {
            return Err(other_error("stopped"));
        }
        match TcpStream::connect_timeout(&address, CONNECT_TIMEOUT) {
            Ok(stream) => return Ok(stream),
            Err(e) => last_error = Some(e),
        }
    }
    Err(last_error.unwrap_or_else(|| other_error(format!("host '{}' not found", url.host))))
}

fn request(url: &Url, abort: &Abort, agent: Agent) -> io::Result<Response> {
    let tcp = connect(url, abort)?;
    tcp.set_read_timeout(Some(READ_TIMEOUT))?;
    tcp.set_write_timeout(Some(CONNECT_TIMEOUT))?;
    tcp.set_nodelay(true)?;
    abort.attach(&tcp);

    let mut connection = if url.https {
        let connector = native_tls::TlsConnector::new().map_err(|e| other_error(e.to_string()))?;
        let tls = connector.connect(&url.host, tcp).map_err(|e| other_error(format!("TLS: {e}")))?;
        Connection::Tls(Box::new(tls))
    } else {
        Connection::Plain(tcp)
    };

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nAccept: */*\r\nIcy-MetaData: 1\r\nConnection: close\r\n\r\n",
        url.path,
        url.host_header(),
        agent.header()
    );
    connection.write_all(request.as_bytes())?;
    connection.flush()?;

    let mut reader = BufReader::with_capacity(64 * 1024, connection);
    let (status, headers) = read_head(&mut reader)?;
    let find = |name: &str| headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone());
    let chunked = find("transfer-encoding").is_some_and(|v| v.to_ascii_lowercase().contains("chunked"));
    let length = find("content-length").and_then(|v| v.trim().parse::<u64>().ok());

    let body: Box<dyn Read + Send> = if chunked {
        Box::new(ChunkedReader::new(reader))
    } else if let Some(length) = length {
        Box::new(reader.take(length))
    } else {
        Box::new(reader)
    };
    Ok(Response { status, headers, url: url.clone(), body })
}

/// Decodes header text; old Shoutcast servers send Latin-1.
pub fn decode_text(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => bytes.iter().map(|&b| b as char).collect(),
    }
}

fn read_head<R: BufRead>(reader: &mut R) -> io::Result<(u16, Vec<(String, String)>)> {
    let mut total = 0;
    let mut line = Vec::new();
    let mut read_line = |reader: &mut R, line: &mut Vec<u8>| -> io::Result<String> {
        line.clear();
        let n = reader.read_until(b'\n', line)?;
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "connection closed before response"));
        }
        total += n;
        if total > MAX_HEAD_BYTES {
            return Err(other_error("response header too large"));
        }
        Ok(decode_text(line).trim_end_matches(['\r', '\n']).to_string())
    };

    let status_line = read_line(reader, &mut line)?;
    let mut parts = status_line.split_whitespace();
    let protocol = parts.next().unwrap_or("");
    if !(protocol.starts_with("HTTP/") || protocol == "ICY") {
        return Err(other_error(format!("not an HTTP response: '{status_line}'")));
    }
    let status = parts
        .next()
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| other_error(format!("invalid status line '{status_line}'")))?;

    let mut headers = Vec::new();
    loop {
        let header_line = read_line(reader, &mut line)?;
        if header_line.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = header_line.split_once(':') {
            headers.push((name.trim().to_string(), value.trim().to_string()));
        }
    }
    Ok((status, headers))
}

/// Body reader for `Transfer-Encoding: chunked`.
pub struct ChunkedReader<R: BufRead> {
    inner: R,
    remaining: u64,
    done: bool,
}

impl<R: BufRead> ChunkedReader<R> {
    pub fn new(inner: R) -> Self {
        ChunkedReader { inner, remaining: 0, done: false }
    }

    fn read_size(&mut self) -> io::Result<u64> {
        let mut line = String::new();
        loop {
            line.clear();
            if self.inner.read_line(&mut line)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "chunked body ended early"));
            }
            let text = line.trim();
            if text.is_empty() {
                continue;
            }
            let hex = text.split(';').next().unwrap_or("").trim();
            return u64::from_str_radix(hex, 16).map_err(|_| other_error(format!("invalid chunk size '{hex}'")));
        }
    }
}

impl<R: BufRead> Read for ChunkedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.done || buf.is_empty() {
            return Ok(0);
        }
        if self.remaining == 0 {
            let size = self.read_size()?;
            if size == 0 {
                self.done = true;
                return Ok(0);
            }
            self.remaining = size;
        }
        let want = buf.len().min(self.remaining.min(usize::MAX as u64) as usize);
        let n = self.inner.read(&mut buf[..want])?;
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "chunked body ended early"));
        }
        self.remaining -= n as u64;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_urls() {
        let u = Url::parse("https://Stream.Example.com/live/radio.mp3?x=1#frag").unwrap();
        assert_eq!(u, Url { https: true, host: "stream.example.com".into(), port: 443, path: "/live/radio.mp3?x=1".into() });
        let u = Url::parse("http://10.0.0.1:8000").unwrap();
        assert_eq!((u.host.as_str(), u.port, u.path.as_str()), ("10.0.0.1", 8000, "/"));
        assert_eq!(u.host_header(), "10.0.0.1:8000");
        let u = Url::parse("http://[::1]:8080/a").unwrap();
        assert_eq!((u.host.as_str(), u.port), ("::1", 8080));
        assert_eq!(u.host_header(), "[::1]:8080");
        assert!(Url::parse("ftp://x").is_err());
        assert!(Url::parse("http:///path").is_err());
    }

    #[test]
    fn joins_references() {
        let base = Url::parse("https://cdn.example.com/hls/radio/master.m3u8?token=1").unwrap();
        assert_eq!(base.join("low/index.m3u8").unwrap().to_string(), "https://cdn.example.com/hls/radio/low/index.m3u8");
        assert_eq!(base.join("../seg1.ts").unwrap().to_string(), "https://cdn.example.com/hls/seg1.ts");
        assert_eq!(base.join("/abs.aac").unwrap().to_string(), "https://cdn.example.com/abs.aac");
        assert_eq!(base.join("//other.org/x").unwrap().to_string(), "https://other.org/x");
        assert_eq!(base.join("http://plain.org:8000/s").unwrap().to_string(), "http://plain.org:8000/s");
        assert_eq!(base.join("?token=2").unwrap().to_string(), "https://cdn.example.com/hls/radio/master.m3u8?token=2");
    }

    #[test]
    fn reads_icy_and_http_heads() {
        let mut r = Cursor::new(b"ICY 200 OK\r\nicy-name: Test FM\r\nicy-metaint:16000\r\n\r\nDATA".to_vec());
        let (status, headers) = read_head(&mut r).unwrap();
        assert_eq!(status, 200);
        assert_eq!(headers[1], ("icy-metaint".to_string(), "16000".to_string()));
        let mut rest = String::new();
        r.read_to_string(&mut rest).unwrap();
        assert_eq!(rest, "DATA");

        let mut r = Cursor::new(b"HTTP/1.1 302 Found\nLocation: /x\n\n".to_vec());
        let (status, headers) = read_head(&mut r).unwrap();
        assert_eq!((status, headers[0].1.as_str()), (302, "/x"));
    }

    #[test]
    fn decodes_chunked_bodies() {
        let raw = b"4\r\nWiki\r\n5;ext=1\r\npedia\r\n0\r\n\r\n".to_vec();
        let mut r = ChunkedReader::new(BufReader::new(Cursor::new(raw)));
        let mut out = String::new();
        r.read_to_string(&mut out).unwrap();
        assert_eq!(out, "Wikipedia");
    }

    #[test]
    fn decodes_latin1_fallback() {
        assert_eq!(decode_text(&[b'K', 0xF6, b'l', b'n']), "Köln");
        assert_eq!(decode_text("Köln".as_bytes()), "Köln");
    }
}

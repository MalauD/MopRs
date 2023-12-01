use std::pin::Pin;
use std::task::Poll;
use std::{convert::TryInto, io};

use aes::Aes128;
use block_modes::{block_padding::NoPadding, block_padding::Pkcs7, BlockMode, Cbc, Ecb};
use blowfish::Blowfish;
use serde::{Deserialize, Serialize};

pub struct StreamingCredentials {
    pub arl: String,
    pub sid: String,
    pub token: String,
}

impl StreamingCredentials {
    pub fn new(arl: String) -> Self {
        Self {
            arl,
            sid: String::new(),
            token: String::new(),
        }
    }

    pub fn set_sid(&mut self, sid: String) {
        self.sid = sid;
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnofficialDeezerMusic {
    #[serde(rename = "SNG_ID")]
    id: String,
    #[serde(rename = "MD5_ORIGIN")]
    md5: String,
    #[serde(rename = "MEDIA_VERSION")]
    media_version: String,
    #[serde(rename = "FILESIZE_MP3_128")]
    size: String,
}

impl UnofficialDeezerMusic {
    fn get_track_name(&self) -> String {
        let data = vec![
            self.md5.clone(),
            "1".to_string(),
            self.id.clone(),
            self.media_version.clone(),
        ]
        .join("¤");
        let data_ascii = to_vec_u8_ascii(&data);

        let data_md5 = md5::compute(&data_ascii);
        let mut joined_data = format!("{}¤{}¤", hex::encode(data_md5.0), data);
        while joined_data.chars().count() % 16 > 0 {
            //Slow chars is o(n)
            joined_data.push(' ');
        }

        type Aes128Ecb = Ecb<Aes128, Pkcs7>;
        let cipher =
            Aes128Ecb::new_from_slices("jo6aey6haid2Teih".to_string().as_bytes(), &[]).unwrap();

        hex::encode(cipher.encrypt_vec(&to_vec_u8_ascii(&joined_data)))
    }

    pub fn get_url(&self) -> String {
        let f_md5 = self.md5.chars().next().unwrap();
        format!(
            "http://e-cdn-proxy-{}.deezer.com/mobile/1/{}",
            f_md5,
            self.get_track_name()
        )
    }

    pub fn get_bf_key(&self) -> String {
        let secret = "g4el58wc0zvf9na1";
        let md5_music_id = hex::encode(md5::compute(self.id.to_string().as_bytes()).0);
        let mut blowfish_key = String::new();
        for i in 0..16 {
            blowfish_key.push_str(
                &String::from_utf16(&[md5_music_id.chars().nth(i).unwrap() as u16
                    ^ md5_music_id.chars().nth(i + 16).unwrap() as u16
                    ^ secret.chars().nth(i).unwrap() as u16])
                .unwrap(),
            );
        }
        blowfish_key
    }
}

fn to_vec_u8_ascii(data: &str) -> Vec<u8> {
    let mut data_ascii: Vec<u8> = vec![];
    for c in data.chars() {
        if c == '¤' {
            data_ascii.push(164);
        } else {
            data_ascii.push(c as u8)
        }
    }
    data_ascii
}

pub struct ChunkedStream<T>
where
    T: futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
{
    stream: T,
    buffer: Vec<u8>,
    idx: u32,
}

impl<T> ChunkedStream<T>
where
    T: futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
{
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            buffer: Vec::with_capacity(2048),
            idx: 0,
        }
    }
}

impl<T> futures::Stream for ChunkedStream<T>
where
    T: futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Unpin,
{
    type Item = Result<[u8; 2048], actix_web::Error>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.idx > 2048 {
            // Drain the buffer and return the first 2048 bytes
            self.idx = self.idx - 2048;
            let next: [u8; 2048] = self.buffer[..2048].try_into().unwrap();
            // Remove the first 2048 bytes
            self.buffer.drain(..2048);

            return Poll::Ready(Some(Ok(next)));
        }
        match Pin::new(&mut self.stream).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(res))) => {
                self.buffer.extend_from_slice(&res);
                self.idx += res.len() as u32;
                self.poll_next(cx)
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{:?}", e),
            )
            .into()))),
            Poll::Ready(None) => {
                if self.buffer.len() > 0 {
                    let next: [u8; 2048] = self.buffer[..2048].try_into().unwrap();
                    Poll::Ready(Some(Ok(next)))
                } else {
                    Poll::Ready(None)
                }
            }
        }
    }
}

pub struct DeezerMusicStream<T>
where
    T: futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
{
    cipher: Cbc<Blowfish, NoPadding>,
    stream: ChunkedStream<T>,
    chunk_count: u32,
}

impl<T> DeezerMusicStream<T>
where
    T: futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
{
    pub fn new(
        music: &UnofficialDeezerMusic,
        credentials: &StreamingCredentials,
        stream: ChunkedStream<T>,
    ) -> Self {
        let bf_key = music.get_bf_key();
        let cipher = Cbc::new_from_slices(bf_key.as_bytes(), &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
        Self {
            cipher,
            stream,
            chunk_count: 0,
        }
    }
}

impl<T> futures::Stream for DeezerMusicStream<T>
where
    T: futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Unpin,
{
    type Item = Result<bytes::Bytes, actix_web::Error>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match Pin::new(&mut self.stream).poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(res))) => {
                if self.chunk_count % 3 > 0 {
                    self.chunk_count += 1;
                    Poll::Ready(Some(Ok(bytes::Bytes::copy_from_slice(&res))))
                } else {
                    self.chunk_count += 1;
                    Poll::Ready(Some(Ok(self.cipher.decrypt_vec(&res).unwrap().into())))
                }
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

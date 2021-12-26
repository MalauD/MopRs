use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};
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

    pub fn set_sid(&mut self, sid: String) -> () {
        self.sid = sid;
    }

    pub fn set_token(&mut self, token: String) -> () {
        self.token = token;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StreamMusic {
    #[serde(rename = "SNG_ID")]
    id: String,
    #[serde(rename = "MD5_ORIGIN")]
    md5: String,
    #[serde(rename = "MEDIA_VERSION")]
    media_version: String,
    #[serde(rename = "FILESIZE_MP3_128")]
    size: String,
}

impl StreamMusic {
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
        let md5_music_id = hex::encode(&md5::compute(self.id.to_string().as_bytes()).0);
        let mut blowfish_key = String::new();
        for i in 0..16 {
            blowfish_key.push_str(
                &String::from_utf16(&vec![
                    md5_music_id.chars().nth(i).unwrap() as u16
                        ^ md5_music_id.chars().nth(i + 16).unwrap() as u16
                        ^ secret.chars().nth(i).unwrap() as u16,
                ])
                .unwrap(),
            );
        }
        return blowfish_key;
    }
}

fn to_vec_u8_ascii(data: &String) -> Vec<u8> {
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

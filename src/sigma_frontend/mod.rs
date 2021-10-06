use bytes::Bytes;
use std::path::Path;


/// Singleton (by design) struct created for reading data
#[derive(Default)]
pub struct FrontendData {
    index: Option<FrontendBytes>,
    catalogue: Option<FrontendBytes>,
    cabinet: Option<FrontendBytes>,
    login: Option<FrontendBytes>
}

impl FrontendData {
    #[inline]
    pub fn get_index(&self) -> Option<&FrontendBytes> {
        self.index.as_ref()
    }

    #[inline]
    pub fn get_catalogue(&self) -> Option<&FrontendBytes> {
        self.catalogue.as_ref()
    }

    #[inline]
    pub fn get_cabinet(&self) -> Option<&FrontendBytes> {
        self.cabinet.as_ref()
    }

    #[inline]
    pub fn get_login(&self) -> Option<&FrontendBytes> {
        self.login.as_ref()
    }
}

pub struct FrontendBytes {
    pub html: Option<Bytes>,
    pub css: Option<Bytes>,
    pub js: Option<Bytes>
}

pub struct ReadFrontend;

impl ReadFrontend {
    #[inline]
    fn read<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Bytes, std::io::Error> {
         let bytes: Bytes = std::fs::read(path)?.into();
         Ok(bytes)
    }

    fn read_dir(dir: &'static str) -> Result<FrontendBytes, std::io::Error> {
        println!("Reading dir: {}", dir);

        let html = Self::read(Path::new("res").join("sigma_frontend").join(dir).join("index.html"));
        let css = Self::read(Path::new("res").join("sigma_frontend").join(dir).join("style.css"));
        let js = Self::read(Path::new("res").join("sigma_frontend").join(dir).join("script.js"));

        println!("\tHtml: Ok,  css: {}, js:{}\n", css.is_ok(), js.is_ok());

        Ok(FrontendBytes {
            html: if let Ok(c) = html { Some(c) } else { None },
            css: if let Ok(c) = css { Some(c) } else { None },
            js: if let Ok(c) = js { Some(c) } else { None }
        })
    }

    #[inline]
    pub fn create_data() -> FrontendData {
        FrontendData {
            index: Some(Self::read_dir("index").expect("Unable to create data! Index directory was not read.")),
            // catalogue: Self::read_dir("catalogue"),
            cabinet: Some(Self::read_dir("cabinet").expect("Unable to create data! Cabinet directory was not read")),
            login: Some(Self::read_dir("login").expect("Unable to create data! Login directory was not read.")),
            .. Default::default()
        }
    }
}

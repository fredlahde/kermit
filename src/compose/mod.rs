use walkdir::WalkDir;

pub struct Scanner {}

pub struct ScanResult {
    pub path: String,
}

impl ScanResult {
    fn new(path: &str) -> ScanResult {
        ScanResult {
            path: String::from(path),
        }
    }
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {}
    }
}

impl Scanner {
    pub fn scan(&self, base_path: &str) -> Vec<ScanResult> {
        let mut return_vec = Vec::new();
        for entry in WalkDir::new(base_path) {
            match entry {
                Ok(ref e) => {
                    let res = ScanResult::new(e.path().to_str().expect("path with no name"));
                    return_vec.push(res);
                }
                Err(_e) => continue,
            }
        }
        return_vec
    }
}

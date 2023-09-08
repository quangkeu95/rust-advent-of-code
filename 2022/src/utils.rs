use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct FileUtils {}

impl FileUtils {
    pub fn parse_text_file<P, T>(file_path: P) -> Result<Vec<T>, std::io::Error>
    where
        P: AsRef<Path>,
        T: From<String>,
    {
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);

        let contents = buf_reader
            .lines()
            .map(|line| {
                let line = line?;
                Ok(T::from(line))
            })
            .collect::<Result<Vec<T>, std::io::Error>>()?;

        Ok(contents)
    }
}

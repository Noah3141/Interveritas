
#[derive(Debug)]
pub struct Citation {
    pub source_type: CitSourceType,
    pub authors: String,
    pub year: String,
    pub title: String,
    pub journal: String,
    pub issue_num: String,
    pub article_num: String,
    pub pp: String,
    pub citation_count: String,
    pub method: String,
}

#[derive(Debug)]
pub enum CitSourceType {
    ExpArticle,
    RevArticle,
    Book,
    Lecture,
    Unspecified,
}

pub enum LocationSpecs {
    Pages(u16, u16),
    Doi(u32),
}



impl Citation {


    pub fn build(mut self) -> String {
        let mut citation = String::new();
        let mut parts = vec![
            self.authors,
            self.year,
            self.title,
            self.journal,
            self.issue_num,
            self.article_num,
            self.pp,
        ];
        parts[1] = "(".to_owned() + &parts[1] + ").";
        for i in parts {
            citation.push_str((i + " ").as_str());
        }
        citation
    }

    pub fn new() -> Self {
        Self {
            source_type: CitSourceType::Unspecified,
            authors: String::new(),
            year: String::new(),
            title: String::new(),
            journal: String::new(),
            issue_num: String::new(),
            article_num: String::new(),
            pp: String::new(),
            citation_count: String::new(),
            method: String::new(),
        }
    }
}


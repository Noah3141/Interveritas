fn main() {
    #[derive(Debug)]
    struct Citation {
        authors: String,
        year: String,
        title: String,
        journal: String,
        issue_num: String,
        article_num: String,
        pp: String,
        citation_count: String,
        method: String,
    }
    impl Citation {
        fn build(mut self) -> String {
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

        fn new(
            authors: String,
            year: String,
            title: String,
            journal: String,
            issue_num: String,
            article_num: String,
            pp: String,
            citation_count: String,
            method: String,
        ) -> Self {
            Self {
                authors,
                year,
                title,
                journal,
                issue_num,
                article_num,
                pp,
                citation_count,
                method,
            }
        }
    }

    let mut citation_01 = Citation::new(
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    );

    citation_01
        .authors
        .push_str("Kupeciec, M., Sandvoll, L. G., & Steckley, N. J. D.");
    citation_01.year.push_str("2021");
    citation_01
        .title
        .push_str("Effects of Ketogenic Diets on Mental Health");
    citation_01.journal.push_str("Journal of Psychiatry");

    println!("{:#?}", citation_01);

    let reference_01 = citation_01.build();

    println!("{}", reference_01);
}

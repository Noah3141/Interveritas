

fn main() {

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

    fn new_citation(authors:String, year:String, title:String, journal:String, issue_num:String, article_num:String, pp:String, citation_count:String, method:String) -> Citation {
        Citation {
            authors,
            year, 
            title, 
            journal, 
            issue_num, 
            article_num, 
            pp, 
            citation_count, 
            method
        }
    }

    let mut citation_01 = new_citation(String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new(),String::new());

    citation_01.authors.push_str("Kupeciec, M., Sandvoll, L. G., & Steckley, N. J. D.");
    citation_01.year.push_str("2021");
    citation_01.title.push_str("Effects of Ketogenic Diets on Mental Health");
    citation_01.journal.push_str("Journal of Psychiatry");

    println!("{}", citation_01.authors);
    println!("{}", citation_01.year);
    println!("{}", citation_01.title);
    println!("{}", citation_01.journal);


    let citation_02 = Citation {
        title:String::from("A Revision to Previous Publication"),
        ..citation_01
    };

    println!("{}", citation_02.authors);
    println!("{}", citation_02.year);
    println!("{}", citation_02.title);
    println!("{}", citation_02.journal);


}


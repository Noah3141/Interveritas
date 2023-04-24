use crate::models::citation_list::CitationList;

pub fn google_scholar_list(list: String) -> CitationList {


    CitationList::new_blank()
}

// todo: Use keywords to generate further prompts and fill up the database from auto-googles 
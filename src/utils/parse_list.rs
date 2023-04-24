use crate::models::citation_list::CitationList;
use crate::models::citation::Citation;
use regex::Regex;

pub fn parse_list(list: String) -> CitationList {

    println!("Received String to parse_list : {list}");


    // Determine the structure of the String: 
    /* 
    Possible structures of the String 
    
    "6. Matthews KA, Xu W, Gaglioti AH, et al. Racial and ethnic estimates of Alzheimer’s disease and related dementias in the United States (2015–2060) in adults aged 65 years. Alzheimers Dement. 2018;15(1):17-24. 7. Baumgart M, Snyder HM, Carrillo MC, Fazio S, Kim H, Johns H. Summary of the evidence on modifiable risk factors for cognitive decline and dementia: a population-based perspective. Alzheimers Dement. 2015;11(6):718-726. 8. Norton S, Matthews FE, Barnes DE, Yaffe K, Brayne C. Potential for primary prevention of Alzheimer’s disease: an analysis of population-based data. Lancet Neurol. 2014;13(8):788-794. "
    
    
    */


    // Break up the list into a vector of strings
    let re = Regex::new(r"").unwrap();


    // Loop through the strings and make a citation for it

        let citation: Citation = Citation::new_article(authors, year, title, journal, volume, issue, pages, identifier);
        // Put the parts into a new Citation
        
        citation_list.list.push(citation);


    let citation_list: CitationList = CitationList::new_blank();

    citation_list
}
mod citation;
use citation::Citation;
use citation::CitSourceType;
use std::collections::HashMap;

// When done testing, build Citation as structs within an Enum{struct BookCit, struct ReviewCit, struct ExpCit}
// Because a book citation needs different fields in the first place, and the build impl will need to look different for each
fn main() {


    let mut citation_categories = HashMap::new();
    citation_categories.insert("Psychology", vec!["Perception", "Neuro","Organizational","Clinical"]);

    println!("{:?}", citation_categories["Psychology"])

}

fn break_down_reference_list(_raw_list: String) {

    todo!();
    // Input a copy and pasted listed of references from an article similar to yours, and get
    // 1. Data on year distribution
    // 2. Prevalence of Authors in the literature
    // 3. Prevalence Journal
}


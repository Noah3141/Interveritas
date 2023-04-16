mod citation;
use citation::Citation;
use citation::CitSourceType;

// When done testing, build Citation as structs within an Enum{struct BookCit, struct ReviewCit, struct ExpCit}
// Because a book citation needs different fields in the first place, and the build impl will need to look different for each
fn main() {
    // Create a citation
    let mut citation_01 = Citation::new();
    citation_01.source_type = CitSourceType::RevArticle;
    citation_01
        .authors
        .push_str("Kupeciec, M., Sandvoll, L. G., & Steckley, N. J. D.");
    citation_01.year.push_str("2021");
    citation_01
        .title
        .push_str("Effects of Ketogenic Diets on Mental Health");
    citation_01
        .journal
        .push_str("Journal of Metabolic Psychiatry");

    println!("\n\nThe debug output is:\n{:#?}", citation_01);

    let reference_01 = citation_01.build();
    println!("\nThe build citation is:\n{}\n\n", reference_01);

    /*  Error handling by wrapping in Option and 'expecting' non-none */

    let some_num: Option<i32> = Some(5); // some_num is of type "Option" (an 'enum'), the variant is set to "Some"-thing, and holds 5
    // let some_num: Option<i32> = None; // some_num is of type "Option" (an 'enum'), the variant is set to nothing, but *would* hold i32

    let five_plus_unsure_variable: i32 =
        5 + some_num.expect("Expecting i32 to be set for some_num");

    println!("Total: {}", five_plus_unsure_variable);
    assert_eq!(10, five_plus_unsure_variable);

    /* Error handling by unwrapping the Option, and dealing with both Some and None possibilities */

    let some_other_num: Option<i32> = Some(5);
    // let some_other_num: Option<i32> = None;

    // Unwrapping the Option:
    let the_other_num: i32 = match some_other_num {
        // Using match on an Option<T> to unpack its possible states
        None => panic!("Uh oh! some_other_num didn't have something in it."), // If nothing is in it, throw me an error!
        Some(_num) => some_other_num.unwrap(), // If set to Some, and holding a variable, call it 'num' and return me it!
    };

    let five_plus_other_unsure_variable: i32 = 5 + the_other_num;

    println!("Total: {}", five_plus_other_unsure_variable);
    assert_eq!(10, five_plus_other_unsure_variable);
}

fn break_down_reference_list(_raw_list: String) {

    todo!();
    // Input a copy and pasted listed of references from an article similar to yours, and get
    // 1. Data on year distribution
    // 2. Prevalence of Authors in the literature
    // 3. Prevalence Journal
}


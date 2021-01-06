extern crate json;

fn main() -> Result<(), json::Error> {
    let source = r#"
    //asd asd 
    // asdas
    /* asdasd
    11
    2
    ***
    */
{
        "features": [
            1, 
            //asd 
        // asd
        /* asd
        1

        1
        ***********

        */
            2, 
        ],
        //asd 
        // asd
        /* asd
        1

        1
        ***********

        */
        "a": [
            3, 2, //asd
            // asdas
            1, "a", 2, //asdasd
        ],
}
//asd
// asda
/*asdas

a

b 

*****

*/"#;
    println!("Source: {}\n", &source);

    let parsed = json::parse(&source)?;

    println!("Parsed: {:?}\n", parsed);

    println!("stringify: {:?}\n", json::stringify(parsed) );

    Ok(())
}
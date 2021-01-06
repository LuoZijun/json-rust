extern crate json;

fn main() -> Result<(), json::Error> {
    let source = r#"
    //asd asd 
{
        "features": [
            1, 2, 
        ],
        "a": [
            3, 2, 1, "a", 2, 
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
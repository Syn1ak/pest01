use pest::Parser;
use anyhow::anyhow;
use pest01::*;

#[test]
fn field_test() -> anyhow::Result< () > {
    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );

    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}


#[test]
fn record_test() -> anyhow::Result< () > {
    let pair = Grammar::parse(Rule::record, "-273.15,99")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "-273.15,99" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );

    let pair = Grammar::parse(Rule::record, "");
    assert!( pair.is_err() );
    Ok( () )
}

#[test]
fn file_test_single_record_modified() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::file, "42.0,-99.9\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "42.0,-99.9\n");
    Ok(())
}

#[test]
fn file_test_multiple_records_modified() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::file, "123.45,-0.1\n67.89,23.4\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "123.45,-0.1\n67.89,23.4\n");
    Ok(())
}

#[test]
fn file_test_incorrect_format_modified() {
    let pair = Grammar::parse(Rule::file, "123.45,-0.1 67.89,23.4\n");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::file, "123.45,-0.1\n67.89;23.4\n");
    assert!(pair.is_err());

    let pair = Grammar::parse(Rule::file, "123.45,-0.1\n,\n67.89,23.4\n");
    assert!(pair.is_err());
}
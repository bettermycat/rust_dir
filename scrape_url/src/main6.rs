use std::fs;

pub fn main6() {

    let arr = [1, 2, 3];
    assert_eq!(arr[..], [1,2,3], "aaaa");
    assert_eq!(arr[0..=1], [1,2], "dbbbb");

    parse_err();
}

// 函数现在返回一个 Result
fn parse_err() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "ruest2.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};


fn get_file_content(filepath: &str) -> String {

    // how to read it??
    let file = File::open(filepath).expect("|Err| Cannot open file");
    let reader = BufReader::new(file);

    let parser = EventReader::new(reader);

    let mut content =String::new();

    for event in parser {
        // println!("{:?}", event);
        // this gives the event as an iterator, I have to consume only characters
        match event.unwrap() {
            XmlEvent::Characters(event)=> {
                content += &(event.as_str().to_owned() + " "); 
            }
        _ => {}
        }
    }

    content
}
fn main() {
    // todo -> build a xml parser that reads
    let file_path = "/Users/apple/Documents/github/rusindex/rusindex/docs.gl/gl4/glActiveShaderProgram.xhtml";
    let content = get_file_content(file_path);
    println!("{:?}", content)

    // get a file 
    
}

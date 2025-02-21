fn main() {
    let texti = "abc def ghi jkl";
    if let Some((fyrsta_ordid, afgangur)) = texti
                                                      .split_once(' ') {
       println!("1. {}, afgangur: {}", fyrsta_ordid, afgangur)                                                 
    }
    
    
    let stadsetning = "H-202"; // H 2 2
    //let huslidir = stadsetning.split('-').collect::<Vec<&str>>();
    if let Some((hus, numer)) = stadsetning.split_once('-') {
        //hus = Hus::try_from(hus)?
        // numer er "202"
        let haed = &numer[0..1]; // og svo parse-a
        let herbergi_nr = &numer[1..]; // og svo parse-a
    }
}

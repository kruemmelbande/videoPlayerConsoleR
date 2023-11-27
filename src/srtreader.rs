
pub fn read_file(file_path: &str, timestamp: f32) -> Result<String, String> {
    //file reading logic here
    //return Ok(format!("This is a test subtitle at time {}       ", timestamp));
    // Return Ok(result) if successful, or Err(error_message) if an error occurs
    

    //to get the subtitles, we first need to read the file
    //then we need to find the first element. This is the number of the subtitle, which is represented as [\r\n]|$[0-9]+[\r\n]
    let file_contents = match std::fs::read_to_string(file_path){
        Ok(result) => result,
        Err(error) => return Err(format!("Error reading file: {}", error))
    };
    let mut file_contents = file_contents.split("\n");

    loop{
        let line = match file_contents.next(){
            Some(result) => result,
            None => return Err(format!("\x1B[K"))
        };
        if line == ""{
            continue;
        }
        if line.parse::<u32>().is_ok(){
            let range = match file_contents.next(){
                Some(result) => result,
                None => return Err(format!("\x1B[K"))
            };
            if is_in_time_range(range, timestamp){
                let mut result = file_contents.next().unwrap().to_string();
                loop{
                    let line = file_contents.next().unwrap();
                    if line == ""{
                        break;
                    }
                    result.push_str(&format!("\n{}", line));
                }
                return Ok(format!("\x1B[K{}",result));
            }
        }
    }
    
}
fn is_in_time_range(range: &str, timestamp: f32) -> bool{
    //range is in the format of "00:00:00,000 --> 00:00:05,000"
    //timestamp is in seconds
    //returns true if timestamp is in range, false otherwise
    //print!("{} ", range);
    let mut range = range.split(" --> ");
    let start = range.next().unwrap();
    let end = range.next().unwrap();
    let mut start = start.split(":");
    let mut end = end.split(":");

    let mut start_seconds = start.next().unwrap().parse::<f32>().unwrap()*3600.;
    start_seconds += start.next().unwrap().parse::<f32>().unwrap()*60.;
    start_seconds += start.next().unwrap().replace(",", ".").parse::<f32>().unwrap();
    let mut end_seconds = end.next().unwrap().parse::<f32>().unwrap()*3600.;
    end_seconds += end.next().unwrap().parse::<f32>().unwrap()*60.;
    end_seconds += end.next().unwrap().replace(",", ".").parse::<f32>().unwrap();
    return timestamp >= start_seconds && timestamp <= end_seconds;
}
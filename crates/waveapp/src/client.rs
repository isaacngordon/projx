use std::fmt::Display;

pub struct WaveAppClient {
    url: String,
    session: String
}

use std::fmt;

pub struct WaveAppPayload {
    operation: String, 
    variables: String
}

impl fmt::Display for WaveAppPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaveAppPayload {{ operation: {}, variables: {} }}", self.operation, self.variables)
    }
}

impl WaveAppClient {
    fn query(&self , payload: WaveAppPayload) {
        print!("Query Payload: {}", payload);
        todo!()
    }
}


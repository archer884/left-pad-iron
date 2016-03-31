use pad::{PadStr, Alignment};

pub struct PaddingRequest {
    length: usize,
    content: String,
    padding_character: char,
}

impl PaddingRequest {
    pub fn new(length: usize, content: String, padding_character: char) -> PaddingRequest {
        PaddingRequest {
            length: length,
            content: content,
            padding_character: padding_character,
        }
    }
}

#[derive(RustcEncodable)]
pub struct PaddingResponse {
    original: String,
    padded: String,
    length: usize,
}

pub fn left(params: PaddingRequest) -> PaddingResponse {
    PaddingResponse {
        length: params.length,
        padded: params.content.pad(params.length, params.padding_character, Alignment::Right, true),
        original: params.content.to_owned(),
    }
}

pub fn right(params: PaddingRequest) -> PaddingResponse {
    PaddingResponse {
        length: params.length,
        padded: params.content.pad(params.length, params.padding_character, Alignment::Left, true),
        original: params.content.to_owned(),
    }
}
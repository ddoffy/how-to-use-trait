pub trait DataProcessor {
    fn process(&self, data: &[u8]) -> Vec<u8>;
    fn can_process(&self, data_type: &str) -> bool;
}

pub struct ImageCompressor {
    pub quality: u8,
}

impl DataProcessor for ImageCompressor {
    fn process(&self, data: &[u8]) -> Vec<u8> {
        println!("Compressing image with quality at leval {}", self.quality);

        data.to_vec()
    }

    fn can_process(&self, data_type: &str) -> bool {
        matches!(data_type, "image" | "jpeg" | "png" | "webp")
    }
}

pub struct EncryptionProcessor {
    pub key: [u8; 32],
}

impl DataProcessor for EncryptionProcessor {
    fn process(&self, data: &[u8]) -> Vec<u8> {
        println!("Encrypting data with 256-bit key");

        data.to_vec()
    }

    fn can_process(&self, data_type: &str) -> bool {
        true
    }
}

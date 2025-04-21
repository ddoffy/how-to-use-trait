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

    fn can_process(&self, _data_type: &str) -> bool {
        true
    }
}

pub fn demonstrate_behavior_abstraction() {
    println!("Demonstrating behavior abstraction with trait objects");
    let data = vec![1, 2, 3, 4, 5];

    let image_compressor = ImageCompressor { quality: 75 };
    let encryption_processor = EncryptionProcessor { key: [0; 32] };

    let processors: Vec<&dyn DataProcessor> = vec![&image_compressor, &encryption_processor];

    for processor in processors {
        if processor.can_process("image") {
            let processed_data = processor.process(&data);
            println!("Processed data: {:?}", processed_data);
        } else {
            println!("Cannot process this data type");
        }
    }
}

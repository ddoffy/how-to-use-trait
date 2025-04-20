use crate::associated_types_constants::demonstrate_database;
use crate::behavior_abstraction::DataProcessor;
use crate::static_dynamic_dispatch::{demonstrate_dynamic_dispatch, demonstrate_static_dispatch};
use crate::trait_bounding::demonstrate_find_duplicates;
use crate::zero_cost_abstraction::{Addition, Mutiplication, perform_operation};

mod advanced_trait_features;
mod associated_types_constants;
mod behavior_abstraction;
mod default_implementation;
mod static_dynamic_dispatch;
mod trait_bounding;
mod zero_cost_abstraction;

fn main() {
    // Using behavior abstraction
    let data = vec![1, 2, 3, 4, 5];

    let image_compressor = behavior_abstraction::ImageCompressor { quality: 75 };

    let processor = &image_compressor;

    let processed_data = process_data(processor, &data);

    println!("Processed data: {:?}", processed_data);

    let encryption_processor = behavior_abstraction::EncryptionProcessor { key: [0; 32] };

    let processor = &encryption_processor;

    let processed_data = process_data(processor, &data);

    println!("Processed data: {:?}", processed_data);

    // Using zero-cost abstraction
    let result1 = perform_operation(Addition, 5.0, 10.0);
    let result2 = perform_operation(Mutiplication, 5.0, 3.0);

    println!("Result of addition: {}", result1);
    println!("Result of multiplication: {}", result2);

    // Using static dispatch
    demonstrate_static_dispatch();

    // Using dynamic dispatch
    demonstrate_dynamic_dispatch();

    // Using trait bounding
    demonstrate_find_duplicates();

    // Using associated types and constants
    demonstrate_database();
}

fn process_data<T: DataProcessor>(processor: &T, data: &[u8]) -> Vec<u8> {
    if processor.can_process("image") {
        processor.process(data)
    } else {
        println!("Cannot process this data type");
        vec![]
    }
}

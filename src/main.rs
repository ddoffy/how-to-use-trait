use crate::advanced_trait_features::{
    demonstrate_blanket_implementation, demonstrate_marker_traits, demonstrate_streaming_iterator,
    demonstrate_supertraits,
};
use crate::architecture_implementation::demonstrate_architecture_implementation;
use crate::associated_types_constants::demonstrate_database;
use crate::behavior_abstraction::{DataProcessor, demonstrate_behavior_abstraction};
use crate::composition_and_extensions::demonstrate_composition_and_extensions;
use crate::error_handling_architecture::demonstrate_error_handling_architecture;
use crate::performance_considerations::demonstrate_performance_considerations;
use crate::runtime_vs_compile_decision::{
    demonstraate_static_runtime_dispatch, demonstrate_dynamic_runtime_dispatch,
};

use crate::dependency_inversion::{
    demonstrate_dependency_inversion, demonstrate_dependency_inversion_with_trait_objects,
};
use crate::static_dynamic_dispatch::{demonstrate_dynamic_dispatch, demonstrate_static_dispatch};
use crate::trait_bounding::demonstrate_find_duplicates;
use crate::zero_cost_abstraction::{Addition, Mutiplication, perform_operation};

mod advanced_trait_features;
mod architecture_implementation;
mod associated_types_constants;
mod behavior_abstraction;
mod composition_and_extensions;
mod default_implementation;
mod dependency_inversion;
mod error_handling_architecture;
mod performance_considerations;
mod runtime_vs_compile_decision;
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

    // Using architecture implementation
    demonstrate_architecture_implementation();

    // Using performance considerations
    demonstrate_performance_considerations();

    // Using behavior abstractiion
    demonstrate_behavior_abstraction();

    // Using super trait
    demonstrate_supertraits();

    // Using marker traits
    demonstrate_marker_traits();

    // Using streaming iterator
    demonstrate_streaming_iterator();

    // Using blanket implementation
    demonstrate_blanket_implementation();

    // Using composition and extensions
    demonstrate_composition_and_extensions();

    // Use error handling architecture
    demonstrate_error_handling_architecture();

    // Using runtime vs compile decision
    demonstraate_static_runtime_dispatch();
    demonstrate_dynamic_runtime_dispatch();

    // Using dependency inversion
    demonstrate_dependency_inversion();
    demonstrate_dependency_inversion_with_trait_objects();
}

fn process_data<T: DataProcessor>(processor: &T, data: &[u8]) -> Vec<u8> {
    if processor.can_process("image") {
        processor.process(data)
    } else {
        println!("Cannot process this data type");
        vec![]
    }
}

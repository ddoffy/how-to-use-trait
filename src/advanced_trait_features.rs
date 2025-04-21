// Object-safe trait (can be used as a trait object)
trait Drawable {
    fn draw(&self);
    fn dimensions(&self) -> (u32, u32);
}

// Non-object-safe trait (cannot be used as a trait object)
trait Generic<T> {
    fn process(&self, item: T);
    fn create(&self) -> T;
}

struct Canvas {
    elements: Vec<Box<dyn Drawable>>, // Can use Drawable as trait object
}

impl Canvas {
    fn add_element(&mut self, element: Box<dyn Drawable>) {
        self.elements.push(element);
    }

    fn render(&self) {
        for element in &self.elements {
            element.draw();
        }
    }
}

struct Circle {
    radius: u32,
}

struct Square {
    width: u32,
    height: u32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }

    fn dimensions(&self) -> (u32, u32) {
        let diameter = self.radius * 2;
        (diameter, diameter)
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!(
            "Drawing a square with width {} and height {}",
            self.width, self.height
        );
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Circle {
    fn new(radius: u32) -> Circle {
        Circle { radius }
    }
}

impl Square {
    fn new(width: u32, height: u32) -> Square {
        Square { width, height }
    }
}

pub fn demonstrate_object_safety() {
    let mut canvas = Canvas {
        elements: Vec::new(),
    };

    let circle = Circle { radius: 10 };
    canvas.add_element(Box::new(circle));

    let circle_2 = Circle::new(23);
    canvas.add_element(Box::new(circle_2));

    let square = Square::new(23, 87);
    canvas.add_element(Box::new(square));

    canvas.render();
}

// Supertraits

trait Identifiable {
    fn id(&self) -> u32;
    fn type_name(&self) -> &'static str;
}

trait Storable: Identifiable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self
    where
        Self: Sized;

    // Uses methods from Identifiable
    fn storage_key(&self) -> String {
        format!("{}:{}", self.type_name(), self.id())
    }
}

#[derive(Debug)]
struct Product {
    product_id: u32,
    name: String,
    price: f64,
}

impl Identifiable for Product {
    fn id(&self) -> u32 {
        self.product_id
    }

    fn type_name(&self) -> &'static str {
        "Product"
    }
}

impl Storable for Product {
    fn serialize(&self) -> Vec<u8> {
        // Simulate serialization
        format!("{}:{}:{}", self.product_id, self.name, self.price).into_bytes()
    }

    fn deserialize(data: &[u8]) -> Self {
        // Simulate deserialization
        let data_str = String::from_utf8_lossy(data);
        let parts: Vec<&str> = data_str.split(':').collect();
        Product {
            product_id: parts[0].parse().unwrap(),
            name: parts[1].to_string(),
            price: parts[2].parse().unwrap(),
        }
    }
}

pub fn demonstrate_supertraits() {
    let product = Product {
        product_id: 1,
        name: String::from("Laptop"),
        price: 999.99,
    };

    println!("Product ID: {}", product.id());
    println!("Product Type: {}", product.type_name());
    println!("Serialized Data: {:?}", product.serialize());
    println!("Storage Key: {}", product.storage_key());

    let serialized_data = product.serialize();
    let deserialized_product = Product::deserialize(&serialized_data);
    println!("Deserialized Product: {:?}", deserialized_product);
}

// Blanket implementation

use std::fmt::Display;

// A trait for converting values to JSON representation
trait ToJson {
    fn to_json(&self) -> String;
}

// Blanket implementation for all types that implement Display
impl<T: Display> ToJson for T {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

pub fn demonstrate_blanket_implementation() {
    let name = "Alice";
    let age = 30;
    let person = Person {
        name: String::from(name),
        age,
    };

    println!("Person as JSON: {}", person.to_json());
    println!("Person as Display: {}", person);
    println!("Name as JSON: {}", name.to_json());
    println!("Age as JSON: {}", age.to_json());
}

// Marker traits
// Marker trait indicating a type can be safely sent between threads
trait ThreadSafe {}

// Marker trait indicating a type is compatible with out caching system
trait Cacheable: Send + Sync + 'static {}

// Implement for specific types that meet the criteria
impl ThreadSafe for String {}
impl ThreadSafe for i32 {}
impl ThreadSafe for Vec<String> {}

impl<T: Send + Sync + 'static> Cacheable for T {}

struct ThreadPool {
    max_threads: usize,
}

impl ThreadPool {
    fn new(max_threads: usize) -> Self {
        ThreadPool { max_threads }
    }

    fn spawn<F, T>(&self, _f: F, _arg: T)
    where
        F: FnOnce(T) + Send + 'static,
        T: ThreadSafe + Send + 'static,
    {
        println!("Spawning thread with safe argument");
    }
}

// Caching system that only works with Cacheable types
struct Cache<T: Cacheable> {
    items: Vec<T>,
}

impl<T: Cacheable> Cache<T> {
    fn new() -> Self {
        Cache { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

pub fn demonstrate_marker_traits() {
    let pool = ThreadPool::new(4);
    pool.spawn(|x| println!("Thread running with arg: {}", x), 42);

    let mut cache = Cache::new();

    cache.add(String::from("Cached String"));
}

trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct Slice<T> {
    slice: Vec<T>,
    index: usize,
}

impl<T> Slice<T> {
    fn new(slice: Vec<T>) -> Self {
        Slice { slice, index: 0 }
    }
}

impl<T> StreamingIterator for Slice<T> {
    type Item<'a>
        = &'a T
    where
        T: 'a,
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.index < self.slice.len() {
            let item = &self.slice[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn process_streaming<S>(iter: &mut S)
where
    S: StreamingIterator,
    for<'a> S::Item<'a>: std::fmt::Debug,
{
    while let Some(item) = iter.next() {
        println!("{:?}", item);
    }
}

pub fn demonstrate_streaming_iterator() {
    let data = vec![1, 2, 3, 4, 5];
    let mut slice_iter = Slice::new(data);

    process_streaming(&mut slice_iter);
}

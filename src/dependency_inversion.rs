use std::sync::Arc;
// ----- Domain Models -----
struct User {
    id: u64,
    email: String,
    name: String,
}

#[derive(Clone)]
struct Product {
    id: u64,
    name: String,
    price: f64,
}

struct Order {
    id: u64,
    user_id: u64,
    items: Vec<OrderItem>,
    total: f64,
    status: OrderStatus,
}

struct OrderItem {
    product_id: u64,
    quantity: u32,
    price: f64,
}

enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

// ----- Repository Abstractions -----
trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn save(&mut self, user: User) -> Result<User, String>;
}

trait ProductRepository {
    fn find_by_id(&self, id: u64) -> Option<Product>;
    fn find_all(&self) -> Vec<Product>;
}

trait OrderRepository {
    fn find_by_id(&self, id: u64) -> Option<Order>;
    fn find_by_user_id(&self, user_id: u64) -> Vec<Order>;
    fn save(&mut self, order: Order) -> Result<Order, String>;
    fn update_status(&mut self, order_id: u64, status: OrderStatus) -> Result<(), String>;
}

// immplementations for repositories
struct OrderRepositoryImpl {
    // connection details
}

impl OrderRepository for OrderRepositoryImpl {
    fn find_by_id(&self, id: u64) -> Option<Order> {
        // Implementation to find order by ID
        None
    }

    fn find_by_user_id(&self, user_id: u64) -> Vec<Order> {
        // Implementation to find orders by user ID
        vec![]
    }

    fn save(&mut self, order: Order) -> Result<Order, String> {
        // Implementation to save order
        Ok(order)
    }

    fn update_status(&mut self, order_id: u64, status: OrderStatus) -> Result<(), String> {
        // Implementation to update order status
        Ok(())
    }
}

// ----- Service Abstractions -----
trait EmailService {
    fn send_order_confirmation(&self, user: &User, order: &Order) -> Result<(), String>;
    fn send_shipping_notification(&self, user: &User, order: &Order) -> Result<(), String>;
}

trait PaymentService {
    fn process_payment(&self, user: &User, amount: f64) -> Result<String, String>;
    fn refund_payment(&self, transaction_id: &str, amount: f64) -> Result<(), String>;
}

// implementations for services
struct StripePaymentService {
    api_key: String,
}

impl PaymentService for StripePaymentService {
    fn process_payment(&self, user: &User, amount: f64) -> Result<String, String> {
        // Implementation to process payment with Stripe
        println!("Processing payment of ${} for user {}", amount, user.id);
        Ok("transaction_id".to_string())
    }

    fn refund_payment(&self, transaction_id: &str, amount: f64) -> Result<(), String> {
        // Implementation to refund payment
        println!("Refunding ${} for transaction {}", amount, transaction_id);
        Ok(())
    }
}

// ----- High-Level Business Logic -----
struct OrderService<U, P, O, E, PM>
where
    U: UserRepository,
    P: ProductRepository,
    O: OrderRepository,
    E: EmailService,
    PM: PaymentService,
{
    user_repository: U,
    product_repository: P,
    order_repository: O,
    email_service: E,
    payment_service: PM,
}

impl<U, P, O, E, PM> OrderService<U, P, O, E, PM>
where
    U: UserRepository,
    P: ProductRepository,
    O: OrderRepository,
    E: EmailService,
    PM: PaymentService,
{
    fn new(
        user_repository: U,
        product_repository: P,
        order_repository: O,
        email_service: E,
        payment_service: PM,
    ) -> Self {
        Self {
            user_repository,
            product_repository,
            order_repository,
            email_service,
            payment_service,
        }
    }

    fn place_order(
        &mut self,
        user_id: u64,
        item_requests: Vec<(u64, u32)>,
    ) -> Result<Order, String> {
        // Find the user
        let user = self
            .user_repository
            .find_by_id(user_id)
            .ok_or_else(|| format!("User {} not found", user_id))?;

        // Create order items
        let mut items = Vec::new();
        let mut total = 0.0;

        for (product_id, quantity) in item_requests {
            let product = self
                .product_repository
                .find_by_id(product_id)
                .ok_or_else(|| format!("Product {} not found", product_id))?;

            let item_price = product.price * quantity as f64;
            total += item_price;

            items.push(OrderItem {
                product_id,
                quantity,
                price: product.price,
            });
        }

        // Create the order
        let order = Order {
            id: generate_id(), // Assume this function exists
            user_id,
            items,
            total,
            status: OrderStatus::Pending,
        };

        // Process payment
        let transaction_id = self.payment_service.process_payment(&user, total)?;

        // Save the order
        let mut saved_order = self.order_repository.save(order)?;

        // Update status
        self.order_repository
            .update_status(saved_order.id, OrderStatus::Paid)?;
        saved_order.status = OrderStatus::Paid;

        // Send confirmation email
        self.email_service
            .send_order_confirmation(&user, &saved_order)?;

        Ok(saved_order)
    }

    // Other business methods...
}

// For the example - not a complete implementation
fn generate_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs()
}

// ----- Concrete Implementations -----
struct PostgresUserRepository {
    // connection details
}

impl UserRepository for PostgresUserRepository {
    fn find_by_id(&self, id: u64) -> Option<User> {
        println!("Finding user {} in PostgreSQL", id);
        Some(User {
            id,
            email: format!("user{}@example.com", id),
            name: format!("User {}", id),
        })
    }

    fn save(&mut self, user: User) -> Result<User, String> {
        println!("Saving user {} to PostgreSQL", user.id);
        Ok(user)
    }
}

struct InMemoryProductRepository {
    products: Vec<Product>,
}

impl ProductRepository for InMemoryProductRepository {
    fn find_by_id(&self, id: u64) -> Option<Product> {
        println!("Finding product {} in memory", id);
        self.products.iter().find(|p| p.id == id).cloned()
    }

    fn find_all(&self) -> Vec<Product> {
        println!("Finding all products in memory");
        self.products.clone()
    }
}

struct SmtpEmailService {
    smtp_server: String,
}

impl EmailService for SmtpEmailService {
    fn send_order_confirmation(&self, user: &User, order: &Order) -> Result<(), String> {
        println!(
            "Sending order confirmation to {} via SMTP server {}",
            user.email, self.smtp_server
        );
        Ok(())
    }

    fn send_shipping_notification(&self, user: &User, order: &Order) -> Result<(), String> {
        println!(
            "Sending shipping notification to {} via SMTP server {}",
            user.email, self.smtp_server
        );
        Ok(())
    }
}

// ----- Usage Example -----
fn dependency_inversion_example() {
    // Create concrete implementations
    let user_repo = PostgresUserRepository { /* connection details */ };

    let products = vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 1299.99,
        },
        Product {
            id: 2,
            name: "Smartphone".to_string(),
            price: 699.99,
        },
        Product {
            id: 3,
            name: "Headphones".to_string(),
            price: 159.99,
        },
    ];
    let product_repo = InMemoryProductRepository { products };

    let order_repo = OrderRepositoryImpl { /* connection details */ };
    let email_service = SmtpEmailService {
        smtp_server: "smtp.example.com".to_string(),
    };
    let payment_service = StripePaymentService {
        api_key: "[your api key]".to_string(),
    };

    // Create the service with concrete implementations
    let mut order_service = OrderService::new(
        user_repo,
        product_repo,
        order_repo,
        email_service,
        payment_service,
    );

    // Use the service
    let items = vec![(1, 1), (3, 2)]; // 1 laptop, 2 headphones
    match order_service.place_order(1001, items) {
        Ok(order) => println!("Order placed: {:?}", order.id),
        Err(err) => println!("Failed to place order: {}", err),
    }
}

// demonstrate_dependency_inversion, demonstrate_dependency_inversion_with_trait_objects,
pub fn demonstrate_dependency_inversion() {
    dependency_inversion_example();
}

pub fn demonstrate_dependency_inversion_with_trait_objects() {
    let user_repo = PostgresUserRepository { /* connection details */ };
    let product_repo = InMemoryProductRepository { products: vec![] };
    let order_repo = OrderRepositoryImpl { /* connection details */ };

    // Create generic service that takes trait objects
    let mut order_service = OrderService::new(
        user_repo,
        product_repo,
        order_repo,
        SmtpEmailService {
            smtp_server: "smtp.example.com".to_string(),
        },
        StripePaymentService {
            api_key: "[your api key]".to_string(),
        },
    );

    // Use the service
    let items = vec![(1, 1), (3, 2)]; // 1 laptop, 2 headphones
    match order_service.place_order(1001, items) {
        Ok(order) => println!("Order placed: {:?}", order.id),
        Err(err) => println!("Failed to place order: {}", err),
    }
}

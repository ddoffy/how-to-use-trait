trait PaymentProcessor {
    fn process_payment(&self, amount: f64, currency: &str) -> Result<PaymentResult, PaymentError>;
    fn supports_currency(&self, currency: &str) -> bool;
    fn name(&self) -> &'static str;
    fn transaction_fee(&self, amount: f64) -> f64;
}

struct CreditCardProcessor {
    api_key: String,
    supported_currencies: Vec<String>,
}

impl PaymentProcessor for CreditCardProcessor {
    fn process_payment(&self, amount: f64, currency: &str) -> Result<PaymentResult, PaymentError> {
        if !self.supports_currency(currency) {
            return Err(PaymentError::UnsupportedCurrency(currency.to_string()));
        }

        println!(
            "Processing payment credit card payment of {} {}",
            amount, currency
        );

        // In real implementation, would call payment gateway API

        // Simulate payment processing
        Ok(PaymentResult {
            transaction_id: "cc_12345".to_string(),
            amount,
            fee: self.transaction_fee(amount),
            timestamp: chrono::Utc::now(),
        })
    }

    fn supports_currency(&self, currency: &str) -> bool {
        matches!(currency, "USD" | "EUR" | "GBP" | "CAD" | "AUD")
    }

    fn name(&self) -> &'static str {
        "Credit Card"
    }

    fn transaction_fee(&self, amount: f64) -> f64 {
        // 2.9% + 0.30 per transaction
        (amount * 0.029) + 0.30
    }
}

struct PayPalProcessor {
    client_id: String,
    supported_currencies: Vec<String>,
}

impl PaymentProcessor for PayPalProcessor {
    fn process_payment(&self, amount: f64, currency: &str) -> Result<PaymentResult, PaymentError> {
        if !self.supports_currency(currency) {
            return Err(PaymentError::UnsupportedCurrency(currency.to_string()));
        }

        println!("Processing PayPal payment of {} {}", amount, currency);

        // In real implementation, would call PayPal API

        // Simulate payment processing
        Ok(PaymentResult {
            transaction_id: "pp_67890".to_string(),
            amount,
            fee: self.transaction_fee(amount),
            timestamp: chrono::Utc::now(),
        })
    }

    fn supports_currency(&self, currency: &str) -> bool {
        self.supported_currencies.iter().any(|c| c == currency)
    }

    fn name(&self) -> &'static str {
        "PayPal"
    }

    fn transaction_fee(&self, amount: f64) -> f64 {
        // 3.49% + 0.49 per transaction
        (amount * 0.0349) + 0.49
    }
}

struct CryptoCurrencyProcessor {
    wallet_address: String,
    blockchain: String,
}

impl PaymentProcessor for CryptoCurrencyProcessor {
    fn process_payment(&self, amount: f64, currency: &str) -> Result<PaymentResult, PaymentError> {
        if !self.supports_currency(currency) {
            return Err(PaymentError::UnsupportedCurrency(currency.to_string()));
        }

        println!(
            "Processing cryptocurrency payment of {} {}",
            amount, currency
        );

        // In real implementation, would call blockchain API

        // Simulate payment processing
        Ok(PaymentResult {
            transaction_id: format!("{} {}", self.blockchain, "abcdef123456"),
            amount,
            fee: self.transaction_fee(amount),
            timestamp: chrono::Utc::now(),
        })
    }

    fn supports_currency(&self, currency: &str) -> bool {
        match self.blockchain.as_str() {
            "Bitcoin" => matches!(currency, "BTC"),
            "Ethereum" => matches!(currency, "ETH" | "LTC" | "USDT" | "USDC"),
            _ => false,
        }
    }

    fn name(&self) -> &'static str {
        "Cryptocurrency"
    }

    fn transaction_fee(&self, amount: f64) -> f64 {
        // 0.001 BTC per transaction
        match self.blockchain.as_str() {
            "Bitcoin" => 5.0,
            "Ethereum" => 10.0,
            _ => 1.0,
        }
    }
}

//
// -----------  Supporting types ---------------
//
#[derive(Debug)]
enum PaymentError {
    InvalidAmount,
    UnsupportedCurrency(String),
    ProcessionFailed(String),
    ConfigurationError,
}

#[derive(Debug)]
struct PaymentResult {
    transaction_id: String,
    amount: f64,
    fee: f64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// ---- Compile time (static) Approach

// An order processing service that uses a specific payment processor
// determined at compile time
struct OrderService<P: PaymentProcessor> {
    payment_processor: P,
    order_prefix: String,
}

impl<P: PaymentProcessor> OrderService<P> {
    fn new(payment_processor: P, order_prefix: String) -> Self {
        OrderService {
            payment_processor,
            order_prefix,
        }
    }

    fn process_order(&self, order_id: &str, amount: f64, currency: &str) -> Result<String, String> {
        println!(
            "Processing order with prefix: {} with {} payment method",
            order_id,
            self.payment_processor.name()
        );

        // Validate order before processing
        if amount <= 0.0 {
            return Err("Invalid amount".to_string());
        }

        // Process the payment using the statically determined processor
        match self.payment_processor.process_payment(amount, currency) {
            Ok(result) => {
                let comfirmation_code = format!(
                    "{}-{}-{}",
                    self.order_prefix, result.transaction_id, result.timestamp
                );

                println!("Payment processed successfully: {}", comfirmation_code);

                println!("Fee: {:.2} {}", result.fee, currency);

                Ok(comfirmation_code)
            }
            Err(PaymentError::UnsupportedCurrency(_)) => Err(format!(
                "{} doesnot support {} currency",
                self.payment_processor.name(),
                currency
            )),
            Err(e) => Err(format!("Payment processing failed: {:?}", e)),
        }
    }
}

/// -------------------- Runtime (dynamic) Approach

// A paymnet getway that uses different payment processors based on runtime conditions
struct PaymentGetway {
    processors: Vec<Box<dyn PaymentProcessor>>,
}

impl PaymentGetway {
    fn new() -> Self {
        PaymentGetway {
            processors: Vec::new(),
        }
    }

    fn add_processor(&mut self, processor: Box<dyn PaymentProcessor>) {
        self.processors.push(processor);
    }

    fn process_payment(
        &self,
        amount: f64,
        currency: &str,
        prefered_method: Option<&str>,
    ) -> Result<PaymentResult, PaymentError> {
        // Find  processors that support this given currency
        let mut eligible_processors: Vec<&Box<dyn PaymentProcessor>> = self
            .processors
            .iter()
            .filter(|p| p.supports_currency(currency))
            .collect();

        if eligible_processors.is_empty() {
            return Err(PaymentError::UnsupportedCurrency(currency.to_string()));
        }

        // Try to find prefered processor if specified
        if let Some(method) = prefered_method {
            let processor = eligible_processors.iter().find(|p| p.name() == method);

            if let Some(processor) = processor {
                return processor.process_payment(amount, currency);
            } else {
                return Err(PaymentError::UnsupportedCurrency(method.to_string()));
            }
        }

        // Otherwise, find the processor with the lowest transaction fee
        eligible_processors.sort_by(|a, b| {
            let fee_a = a.transaction_fee(amount);
            let fee_b = b.transaction_fee(amount);

            fee_a
                .partial_cmp(&fee_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Use the processor with the lowest transaction fee
        match eligible_processors.first() {
            Some(processor) => {
                println!(
                    "Selected {} as the optimal payment method",
                    processor.name()
                );
                processor.process_payment(amount, currency)
            }
            None => Err(PaymentError::UnsupportedCurrency(currency.to_string())),
        }
    }
}

/// -------------- Usage example ------------------

pub fn demonstraate_static_runtime_dispatch() {
    // Create a payment method
    let credit_card = CreditCardProcessor {
        api_key: "[replace with your api key]".to_string(),
        supported_currencies: vec!["USD".to_string(), "EUR".to_string()],
    };

    // Create an order service with the payment method
    // The compiler will generate specialized code for this specific processor
    let order_service = OrderService::new(credit_card, "ORD".to_string());

    // Process an order
    match order_service.process_order("12345", 100.0, "USD") {
        Ok(confirmation_code) => println!("Order processed successfully: {}", confirmation_code),
        Err(e) => println!("Error processing order: {}", e),
    }
}

pub fn demonstrate_dynamic_runtime_dispatch() {
    // Create multiple payment processors
    let credit_card = Box::new(CreditCardProcessor {
        api_key: "[Replace with your api key]".to_string(),
        supported_currencies: vec!["USD".to_string(), "EUR".to_string()],
    });

    let paypal = Box::new(PayPalProcessor {
        client_id: "your_paypal_client_id".to_string(),
        supported_currencies: vec!["USD".to_string(), "EUR".to_string()],
    });

    let crypto = Box::new(CryptoCurrencyProcessor {
        wallet_address: "your_crypto_wallet_address".to_string(),
        blockchain: "Bitcoin".to_string(),
    });

    // Create a payment gateway and add the processors
    let mut payment_gateway = PaymentGetway::new();
    payment_gateway.add_processor(credit_card);
    payment_gateway.add_processor(paypal);
    payment_gateway.add_processor(crypto);

    // Process payment with different runtime criteria
    let results = vec![
        payment_gateway.process_payment(100.0, "USD", None),
        payment_gateway.process_payment(50.0, "EUR", Some("PayPal")),
        payment_gateway.process_payment(0.0, "BTC", None),
        payment_gateway.process_payment(200.0, "ETH", None),
    ];

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(payment_result) => {
                println!(
                    "[{}]. Payment {}:{} (Fee: {:.2})",
                    i, payment_result.transaction_id, payment_result.amount, payment_result.fee
                );
            }
            Err(PaymentError::UnsupportedCurrency(currency)) => {
                println!("[{}]. Unsupported currency: {}", i, currency);
            }
            Err(e) => println!("[{}]. Error processing payment: {:?}", i, e),
        }
    }
}

# ht - Simple HTTP Client
###### **http terminal**
**ht** is a CLI-based HTTP client, allowing users to make HTTP requests conveniently from the command line.
### **Features:**
Supports various HTTP methods like GET, POST, etc.
Allows setting request headers.
Facilitates sending request body, especially for POST requests.
Built-in URL validation.
Colorized output for easy readability.
Modular architecture with clear separation of concerns.
- Supports various HTTP methods like GET, POST, etc.
- Allows setting request headers.
- Facilitates sending request body, especially for POST requests.
- Built-in URL validation.
- Colorized output for easy readability.
- Modular architecture with clear separation of concerns.
### **Requirements:**
 - Rust
 - tokio
 - clap
 - reqwest
 - colored
### **Usage:**
```sh
ht [OPTIONS] URL
```
### **Options:**
- **-m, --method**: Type of the HTTP request (e.g., GET, POST, ...).
- **-h, --headers**: Headers for the HTTP request.
- **-b, --body**: The request body (useful for POST requests).
### **Example**
```sh
ht -m post -b '{"name": "John"}' -h "Content-Type: application/json" https://api.example.com/users
```
### **Modules:**
 - **printer**: Module responsible for displaying outputs.
 - **converters**: Includes request_converter and response_converter for handling request-response transformations.
 - **dto**: Data Transfer Object module with request_dto and response_dto submodules.
 - **builders**: Houses request_builder and response_builder for creating and managing requests and responses respectively.
 - **http_client**: The core module that manages HTTP connections and sends the requests.
 - **argument_parser**: Parses CLI arguments.
 - **validator**: Contains logic for validating URLs.
 - **request_type**: Enum representing different types of HTTP methods.
#### Author: 
Kolia Der










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
ht -m post -b "{ name: John }" -h "Content-Type: application/json" https://api.example.com/users
```
#### Author: 
Kolia Der










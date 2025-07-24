# Rust Study Plan

## Section 1: Basic Rust Syntax & Ownership
### Topics:
- Variables and mutability
- Data types and type inference
- Ownership, borrowing, and lifetimes
- References and slices


### Hands-on Project:
##### ✅ Done [https://github.com/yumingchangsabodota/rust_lang/tree/master/cli_calculator]
**Build a Simple CLI Calculator**
- Create a command-line calculator that takes two numbers and an operator (`+`, `-`, `*`, `/`) as input and returns the result.
- Ensure proper handling of user input and ownership of values.

---

## Section 2: Control Flow and Functions
### Topics:
- Conditionals (`if`, `else`, `match`)
- Loops (`for`, `while`, `loop`)
- Functions and return types
- Rust error handling (`Result`, `Option`)

### Hands-on Project:
##### ✅ Done [https://github.com/yumingchangsabodota/rust_lang/tree/master/guessing_game]
**Build a Number Guessing Game**
- Generate a random number (1-100) and prompt the user to guess it.
- Provide hints like "Too High" or "Too Low" after each guess.
- Use loops and `match` statements to handle conditions.

---

## Section 3: Structs, Enums, and Pattern Matching
### Topics:
- Structs and methods
- Enums and pattern matching
- Implementing traits

### Hands-on Project:
##### ✅ Done [https://github.com/yumingchangsabodota/rust_lang/tree/master/to_do_list]
**Build a To-Do List CLI App**
- Create a `Task` struct with fields: `title`, `description`, and `status (Pending/Done)`.
- Use an `enum` for task status.
- Allow adding, listing, updating, and removing tasks.

---

## Section 4: Collections and Iterators
### Topics:
- Vectors (`Vec<T>`)
- Hash maps (`HashMap<K, V>`)
- Strings and string operations
- Iterators and closures

### Hands-on Project:
**Build a Word Frequency Counter**
- Read a text file, count the occurrences of each word, and display the top 5 most frequent words.
- Use a `HashMap<String, u32>` to store counts.
- Implement iterators and closures for processing text efficiently.

---

## Section 5: Error Handling and Modules
### Topics:
- Error handling with `Result` and `Option`
- Creating and using modules
- Writing and using tests (`cargo test`)

### Hands-on Project:
**Build a File Organizer**
- Create a program that scans a directory and sorts files into folders based on their file types (`.txt`, `.jpg`, `.mp4`, etc.).
- Implement robust error handling for missing directories or read/write failures.
- Use modularity by splitting logic into multiple files.

---

## Section 6: Concurrency in Rust
### Topics:
- Threads and the `std::thread` module
- Message passing and channels
- Shared state concurrency (`Mutex`, `RwLock`, `Arc`)

### Hands-on Project:
**Build a Multi-threaded Web Scraper**
- Use `reqwest` to fetch webpages concurrently.
- Spawn multiple threads to download multiple URLs simultaneously.
- Store scraped data in a file.

---

## Section 7: Advanced Topics and Final Project
### Topics:
- Unsafe Rust and when to use it
- Macros and metaprogramming
- Building and publishing Rust crates
- Using Rust in web development or system programming

### Final Hands-on Project:
**Choose One:**
1. **Build a Simple HTTP Server** (Use `hyper` or `actix-web`)
   - Handle GET and POST requests.
   - Serve static files.
   - Implement basic request routing.

2. **Build a CLI Password Manager**
   - Encrypt and store passwords securely.
   - Allow adding, retrieving, and deleting stored passwords.
   - Implement file-based storage using `serde` for JSON storage.

3. **Build a Simple Blockchain Prototype**
   - Create a block structure with hash, previous hash, and data.
   - Implement mining and proof-of-work.
   - Store a small chain in memory.
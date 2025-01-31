### **Rush: A Simple Shell Implementation**

Rush is a simple, custom-built command-line shell written in Rust. It provides basic shell functionality such as changing directories (`cd`), displaying the current directory (`pwd`), executing commands, and handling piping of commands. The shell supports basic error handling and allows interaction with underlying system commands, all while implementing a custom parser and executor.

* * * * *

### **Features**

-   **Command Execution:** Rush supports the execution of user-defined shell commands such as `pwd`, `cd`, `exit`, and `grep`.
-   **Piping:** The shell can handle piping between multiple commands. For example, executing `command1 | command2`.
-   **Error Handling:** Proper error messages for incorrect commands or usage.
-   **Dynamic Prompt:** The prompt dynamically updates based on the current directory, reflecting the user's location in the file system.
-   **Custom-built parser:** The shell implements its own parser to break down commands and arguments.

* * * * *

### **Project Structure**

-   `main.rs`: Entry point for the shell, where command handling and execution take place.
-   `parser.rs`: Contains the logic for parsing user input, including command parsing and piping handling.
-   `commands.rs`: Contains logic for various shell commands like `cd`, `pwd`, `exit`, and `grep`.
-   `Cargo.toml`: Configuration for the Rust project, managing dependencies and metadata.

* * * * *

### **Installation**

To run the `Rush` shell on your local machine, follow these steps:

#### **Step 1: Install Rust**

Rush is built using Rust, so the first step is to ensure Rust is installed on your system.

-   If Rust is not installed, visit the [official Rust installation guide](https://www.rust-lang.org/tools/install) to set it up.

#### **Step 2: Clone the Repository**

Clone this repository to your local machine.

```
git clone https://github.com/anubhav-auth/rush.git

```

#### **Step 3: Build the Project**

Navigate to the project folder and build the project using Cargo, the Rust package manager:

```
cd rush
cargo build --release

```

This will compile the shell and generate an executable file inside the `target/release/` directory.

#### **Step 4: Run the Shell**

To run the `Rush` shell, simply execute the binary:

```
./target/release/rush

```

This will launch the custom shell, and you should see a prompt like the following:

```
rush <current-directory> $

```

You can now use the shell just like a typical terminal.

* * * * *

### **Usage**

Once the shell is running, you can use the following commands:

-   **`cd <directory>`**: Changes the current working directory.
-   **`pwd`**: Prints the current working directory.
-   **`exit`**: Exits the shell.
-   **`grep <pattern> [file]`**: Searches for a pattern in the specified file or from standard input if no file is provided.
-   **Command Piping:** You can pipe the output of one command to another. For example:

```
cat file.txt | grep "search_term"

```

The shell will process the commands and execute them sequentially, piping output between them as specified.

* * * * *

### **Code Explanation**

-   **Main Loop (`main.rs`)**: This is the entry point where the shell starts. It displays the prompt and waits for user input. When the user enters a command, it is parsed and executed accordingly. Commands like `cd`, `pwd`, `exit`, and `grep` are handled directly, while others are passed to the system for execution.

-   **Command Parsing (`parser.rs`)**: The `parse_input` function breaks the user input into a list of commands and arguments, handling command piping (`|`). It processes each command and separates the command name and arguments for further processing.

-   **Command Handling (`commands.rs`)**: Functions like `handle_cd`, `handle_pwd`, `handle_exit`, and `handle_grep` are defined here. These functions execute their respective commands or interact with the system to execute built-in commands.

-   **Piping (`handle_piping`)**: This function handles the piping of multiple commands. It connects the output of one command to the input of another, creating a pipeline. The function spawns new processes for each command in the pipeline and manages their I/O streams.

* * * * *

### **License**

This project is licensed under the [MIT License](https://github.com/anubhav-auth/rush/blob/master/LICENSE.txt).

* * * * *

### **Contributing**

Feel free to fork this repository and submit pull requests with improvements, bug fixes, or new features. Ensure that your code adheres to the style of the existing project and that you have tested it thoroughly before submitting.

* * * * *

**Example Usage in the Shell**:

```
rush /home/user $ cd /var/log
rush /var/log $ pwd
/var/log
rush /var/log $ grep "error" system.log

```

* * * * *

### **Additional Notes**

-   **Error Handling**: If a command fails (for example, `cd` with an invalid path), the shell will print an appropriate error message.
-   **Current Directory in Prompt**: The prompt updates dynamically based on the current directory (`rush <path> $`).

* * * * *

This is a minimal shell implementation designed for educational purposes. It showcases basic Rust functionalities such as process management, standard I/O handling, and command parsing.

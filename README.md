# 🗂️ Kanban

A lightweight, command-line **Kanban board** written in **Rust**.

---

## 📖 Overview

**Kanban** is a simple yet powerful **terminal-based Kanban board** that helps you manage your tasks and workflows directly from the command line.

With its **intuitive interface** and **fast performance**, Kanban allows you to organize your projects, track progress, and stay productive — all without leaving your terminal.

---

## ⚙️ Installation

### 🧩 Prerequisites
Before you begin, ensure you have **Rust** and **Cargo** installed on your system.  
You can install them using [Rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


```bash
git clone <repository-url>
cd kanban
cargo build --release

---
<img width="786" height="199" alt="image" src="https://github.com/user-attachments/assets/a3cce4ae-b58d-46df-9901-4c3682e9569e" />
---
```bash
# Show the current board
kanban show

# Add a new task
kanban add "Implement new feature"

# Promote a task to the next column
kanban promote 1

# Configure settings
kanban configure

# Delete a task
kanban delete 2
```


---
<img width="1068" height="492" alt="image" src="https://github.com/user-attachments/assets/8c4d7440-ce36-4b59-ae99-75c000bfa525" />
---

<img width="714" height="165" alt="image" src="https://github.com/user-attachments/assets/48de7978-7f88-4cb8-a969-3767e03c915d" />



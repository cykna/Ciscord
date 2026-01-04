#Ciscord
A fast and customizable Discord client — without bullshit.

## ⚠️ Experimental project
Ciscord is not (yet) a full-featured Discord client and is not intended for daily use.

### 📌 What is Ciscord?

Ciscord is an attempt to implement a Discord client — but that is not the real goal of the project.

The real purpose of Ciscord is to act as a real-world testbed for the mid-level library
👉 candy
.

Ciscord exists to:

- aggressively exercise candy in realistic scenarios

- expose design flaws, missing features, and edge cases

- validate architectural decisions under real application complexity

### 🍬 About Candy

#### candy
- is a mid-level library designed to:

- abstract low-level details without becoming a heavyweight framework

- allow fine-grained architectural control

- serve as a foundation for real, non-trivial applications

#### Ciscord is built specifically to answer questions like:

- Does this API actually scale?

- What breaks in a real app?

- What abstractions are missing?

- Which design decisions were wrong?

## 🎯 Project Goals

- ✅ Stress-test candy in a complex, real-world application

- ✅ Discover bugs, limitations, and missing features

- ✅ Validate design and architectural decisions

- ❌ Compete with existing Discord clients

- ❌ Guarantee stability, completeness, or long-term support

- If Ciscord ever becomes usable as a client, that’s a bonus — not the objective.

### 🧪 Status

Highly experimental

APIs may change without notice

Features may be incomplete, broken, or nonexistent

No guarantees of compatibility with Discord

### 🧠 Why a Discord client?

Because it’s an excellent stress test:

- Complex UI

- Heavy global state

- Real-time events

- Dynamic layouts

- Performance-sensitive code

- Architecture under pressure

- If candy can survive this, it can survive a lot.

### 📦 Project Structure

- ciscord → experimental application / test client

- candy → mid-level library (core project)

- They evolve together, but quality and stability efforts are focused on candy.

### 🚧 Final Note

- If you’re here looking for:

- a polished Discord client

- a stable end-user application

- something production-ready

### 👉 this project is probably not for you (yet).

- If you’re interested in:

- architecture

- API design

- mid-level libraries

- experimental projects

👉 Ciscord does exactly what it claims to do.

# Zirv-UI for Yew

Welcome to `zirv-ui`, a comprehensive component library for the [Yew framework](https://yew.rs/). Our goal is to provide a wide range of reusable components tailored for Yew applications, making your web development process smoother and more efficient. `zirv-ui` is designed to be easy to use, highly customizable, and suitable for any use case. Although still a work in progress (WIP), we are excited to offer components that are ready for production use today, including Toast notifications and a Flexbox layout component.

## Components

Currently, `zirv-ui` includes the following components:

- **Toast**: Provide feedback about an operation in a small popup at the corner of the screen.
- **Flex**: A flexible box layout component for efficiently arranging items within a container.

More components are under development and will be released as they are completed.

## Installation

To start using `zirv-ui` in your Yew project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
zirv-ui = { version = "0.1.0", branch = "master" }
```

Furthermore add the following to the <head> tag within your index.html file:
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/Zirv-io/Zirv-ui@main/style/dist/main.css"> 
```
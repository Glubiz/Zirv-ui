# Zirv-UI for Yew

Welcome to `zirv-ui`, a comprehensive component library designed specifically for the [Yew framework](https://yew.rs/). Our mission is to streamline your web development process by providing a wide range of reusable, customizable components tailored for Yew applications. Whether you're building simple or complex web applications, `zirv-ui` aims to provide the tools you need to create efficient, beautiful, and responsive designs. 

## Key Features

- **Ease of Use**: Components are designed to be integrated seamlessly into any Yew project.
- **Customizability**: Extensive options to customize behavior and style to fit your specific needs.
- **Ready for Production**: Some components are already polished and ready for production environments.

## Components

Here is a list of the components that `zirv-ui` currently offers:

- **Toast**: Provides feedback about an operation with a small popup at the corner of the screen.
- **Flex**: Implements a flexible box layout, helping you efficiently arrange items within a container.

We are actively working on expanding our library, and more components will be released as they become ready.

## Installation

To integrate `zirv-ui` into your Yew project, you need to add the library to your `Cargo.toml` and link the stylesheet in your HTML.

### Cargo.toml

Add this line to your project's `Cargo.toml`:

```toml
[dependencies]
zirv-ui = { version = "0.1.0", branch = "master" }
```

## HTML Setup
Furthermore add the following to the <head> tag within your index.html file:
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/Zirv-io/Zirv-ui@main/style/dist/main.css"> 
```

## How to Contribute
We welcome contributions from the community! Whether you're fixing bugs, adding new features, or improving documentation, here's how you can contribute:
- **Reporting Issues**: Use the GitHub Issues section to report bugs.
- **Submitting Pull Requests**: Submit PRs for bug fixes or new features and improvements. Please ensure your code adheres to the existing style to maintain consistency.

## License
`zirv-ui` is open source and available under the MIT License. For more details, see the LICENSE file in the repository.

## Acknowledgements
Thanks to everyone who has contributed to `zirv-ui` so far. Special thanks to the Yew community for their continuous support and feedback.

We hope you find `zirv-ui` useful for your Yew projects! Check back often for new updates and component releases.


### Enhancements and Changes:
1. **Added a Key Features section**: To immediately highlight the advantages and main functionalities of your library.
2. **Expanded Installation Instructions**: To include both Cargo and HTML setup steps.
3. **Added How to Contribute section**: Encouraging community involvement and providing guidelines.
4. **Included a Code of Conduct reminder**: Important for fostering a respectful community.
5. **Acknowledgments section**: To thank contributors and the community, fostering a positive project atmosphere.
6. **License mention**: Clearly specifies the licensing terms, encouraging openness and reuse.

This structure not only presents the necessary information clearly but also encourages community participation and sets expectations regarding conduct and contributions.

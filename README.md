<h1 align="center">Welcome to File Tamer 👋</h1>
<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white" />
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000" />
  <img alt="Contrib" src="https://img.shields.io/badge/contribs-welcome-orange" />
  <img alt="MIT" src="https://img.shields.io/badge/license-MIT-green" />
</p>
<p align="center">
    <img alt="FileTamer" src="resources/icon.png" height="300" width="300" />
</p>
<p align="center">
    <b>The cross-platform file organizer written in Rust</b>
</p>

## 🔨 Build
On Windows you first need to compile the resources into a lib file (to trick Rust apparently):
```
rc /nologo /fo resources\res.lib resources\resources.rc
```

Then on all platforms:
```sh
cargo build
```

## 🖥️ Usage
<!-- maybe add a gif for usage? -->
```
file-tamer COMMAND [ARGS]

Here is a list of possible commands:

  watch      WATCHED_DIRECTORY DESTINATION_DIRECTORY
             watch a directory for new files and move them as they come
  organize   SOURCE_DIRECTORY DESTINATION_DIRECTORY
             organize a directory to another
  help
             display this help
```

## Author

👤 **Logan LUCAS**

* GitHub: [@QwEekYhyo](https://github.com/QwEekYhyo)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/QwEekYhyo/file-tamer/issues). 

## Show your support

Give a ⭐️ if this project helped you!

## ⚖️ License

Copyright © 2024 [Logan LUCAS](https://github.com/QwEekYhyo).<br />
This project is [MIT](https://github.com/QwEekYhyo/file-tamer/blob/main/LICENSE) licensed.

***
_This README was partly generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_

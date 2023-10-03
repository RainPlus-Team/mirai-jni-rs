# Contributing
Thank you for your interest in this project, currently we welcome all kinds of contributions. If you'd like to make a contribution, this guide will help you through.

## Documentation
This project has almost no documentation now, you can help us write it. For example, add documentations to Rust crate or write some guides in [wiki](https://github.com/RainPlus-Team/mirai-jni-rs/wiki).

## Code
If you know how to code in Java/Kotlin or Rust, you can help us out by improving the code or add missing features. It's quite simple.

### Getting started with the project
Currently we use [Gradle](https://gradle.org/) as the main build tool for whole project, including the Rust part. You'll need some basic knowledge of Gradle to get started.

#### Prepare the environment
You will need a working JDK installation and a Rust installation.

For JDK, you can use [OpenJDK](https://openjdk.org/), which is our recommendation.

Rust can be easily installed with `rustup`, available at [Rust website](https://www.rust-lang.org/tools/install).

##### Tools
We suggest you use both [Visual Studio Code](https://code.visualstudio.com) and [IntelliJ IDEA](https://www.jetbrains.com/idea/) at the same time.

Visual Studio Code has free extension `rust-analyzer` available, which provides powerful Rust coding assistance.

IntelliJ IDEA has a free community edition, and it has a better intergration with Gradle and Java/Kotlin.

#### Setup the project
To set the project up, you can start with the `gradlew`/`gradlew.bat`, which wraps up the gradle for you.  
As for IntelliJ IDEA, you don't have to use the wrapper. It will configure the project automatically for you and provides an easy to use user interface.

#### Run and build the project
To run and build the project, you just need to simply run `build` or `run` task in Gradle.
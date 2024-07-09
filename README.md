<div align='center'>

# AWS Custodian <img src="src-tauri/icons/Square89x89Logo.png" align="right" />

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Frontend QA](https://img.shields.io/github/actions/workflow/status/maikbasel/aws-custodian/frontend_qa.yml?label=Frontend%20QA)](https://github.com/maikbasel/aws-custodian/actions/workflows/frontend_qa.yml)
[![Backend QA](https://img.shields.io/github/actions/workflow/status/maikbasel/aws-custodian/backend_qa.yml?label=Backend%20QA)](https://github.com/maikbasel/aws-custodian/actions/workflows/backend_qa.yml)

<p>
AWS Custodian is a Tauri-based desktop application that enables software developers and DevOps professionals to manage 
and navigate through multiple AWS accounts from a convenient central interface. Built with a backend powered by Rust 
and a responsive Next.js frontend, the application simplifies AWS tasks that can often become complex or cumbersome, 
particularly the handling of credentials, configurational files, and the management of SSM parameters and Secret 
Manager Secrets. Custodian is build to alleviate the stress of constantly swapping between accounts on the AWS web 
console or the complexities of dealing with the AWS CLI for parameter and secret management. It's intended for software 
developers or DevOps professionals working extensively with the AWS Cloud platform.
</p>

<h4> <span> · </span> <a href="https://github.com/maikbasel/aws-custodian/blob/master/README.md"> Documentation </a> <span> · </span> <a href="https://github.com/maikbasel/aws-custodian/issues"> Report Bug </a> <span> · </span> <a href="https://github.com/maikbasel/aws-custodian/issues"> Request Feature </a> </h4>

</div>

# :notebook_with_decorative_cover: Table of Contents

- [About the Project](#star2-about-the-project)
    - [Features](#dart-features)
    - [Tech Stack](#space_invader-tech-stack)
- [Roadmap](#compass-roadmap)
- [Getting Started](#toolbox-getting-started)
    - [Prerequisites](#bangbang-prerequisites)
    - [Running Tests](#test_tube-running-tests)
    - [Run Locally](#running-run-locally)
    - [Deployment](#triangular_flag_on_post-deployment)
- [License](#warning-license)

## :star2: About the Project

Welcome to AWS Custodian! This software is a personal endeavor to deepen my understanding of Next.js and Rust.
While I am excited to share my progress, please note that this project is a work in progress and not intended for
commercial use.

Feel free to explore, provide feedback, and contribute if you'd like. Your understanding and support as I navigate
through this learning journey are greatly appreciated.

Thank you for stopping by!

### :space_invader: Tech Stack

<details> <summary>Client</summary> <ul>
<li><a href="https://nextjs.org/">Next.JS</a></li>
</ul> </details>
<details> <summary>Server</summary> <ul>
<li><a href="https://www.rust-lang.org/">Rust</a></li>
<li><a href="https://tauri.app/v1/guides/getting-started/setup/next-js/">Tauri</a></li>
</ul> </details>

### :dart: Features

- CRUD operations for your AWS credentials & configuration files by profile. :white_check_mark:
- Efficient creation, reading, updating, deletion of SSM parameters and Secret Manager Secrets. :white_medium_square:
- Managing and downloading of images/packages from ECR and CodeArtifact repositories. :white_medium_square:

## :compass: Roadmap

- [x] Implement AWS Credentials & Configuration management.
- [ ] Implement AWS SSM Parameter Store management.
- [ ] Implement AWS Secret Manager management.
- [ ] Implement AWS ECR Repository management.
- [ ] Implement AWS CodeArtifact Repository management.

## :toolbox: Getting Started

### :bangbang: Prerequisites

- Install Node.JS on your computer<a href="https://nodejs.org/en"> Here</a>
- Install Tauri CLI

```bash
npm install --save-dev @tauri-apps/cli
```

- Install Rust on your computer<a href="https://rustup.rs/"> Here</a>

### :test_tube: Running Tests

To run frontend tests run the following command

```bash
npm run test
```

To run backend test run the following command

```bash
cargo test
```

### :running: Run Locally

Clone the project

```bash
https://github.com/maikbasel/aws-custodian.git
```

Install dependencies

```bash
npm install
```

Start the development build of the app

```bash
npm run tauri:dev
```

### :triangular_flag_on_post: Deployment

Build the app for production

```bash
npm run tauri build
```

## :warning: License

Distributed under the MIT License. See LICENSE.md for more information.

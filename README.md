<div align='center'>

<h1>AWS Custodian</h1>

[![](https://img.shields.io/badge/powered%20by-Nyx-blue)](https://github.com/mooltiverse/nyx)

<p>
Custodian is a Tauri-based desktop application that enables software developers and DevOps professionals to manage 
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
- [Roadmap](#compass-roadmap)
- [License](#warning-license)
- [Contact](#handshake-contact)

## :star2: About the Project

### :space_invader: Tech Stack

<details> <summary>Client</summary> <ul>
<li><a href="https://nextjs.org/">Next.JS</a></li>
<li><a href="https://tauri.app/v1/guides/getting-started/setup/next-js/">Tauri</a></li>
</ul> </details>
<details> <summary>Server</summary> <ul>
<li><a href="https://www.rust-lang.org/">Rust</a></li>
<li><a href="https://tauri.app/v1/guides/getting-started/setup/next-js/">Tauri</a></li>
</ul> </details>

### :dart: Features

- CRUD operations for your AWS credentials & configuration files by profile. :white_check_mark:
- Efficient creation, reading, updating, deletion of SSM parameters and Secret Manager Secrets. :white_square:
- Managing and downloading of images/packages from ECR and CodeArtifact repositories. :white_square:

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

## :compass: Roadmap

- [x] AWS Credentials & Configuration management.
- [ ] AWS SSM Parameter Store management.
- [ ] AWS Secret Manager management.
- [ ] AWS ECR Repository management.
- [ ] AWS CodeArtifact Repository management.

## :warning: License

Distributed under the MIT License. See LICENSE.md for more information.

## :handshake: Contact

Maik Basel - - to be added

Project Link: [https://github.com/maikbasel/aws-custodian.git](https://github.com/maikbasel/aws-custodian.git).

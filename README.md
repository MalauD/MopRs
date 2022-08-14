<div align="center">
  <h1>MopRs</h1>
  
  <p>
    An awesome deezer client !
  </p>

<!-- Badges -->
<p>
  <a href="https://github.com/MalauD/MopRs/actions">
    <img src="https://github.com/github/docs/actions/workflows/ci.yml/badge.svg" alt="build status" />
  </a>
</p>
   
<h4>
    <a href="https://github.com/MalauD/MopRs/issues/">Report Bug</a>
  <span> · </span>
    <a href="https://github.com/MalauD/MopRs/issues/">Request Feature</a>
  </h4>
</div>

<br />
<!-- About the Project -->

## About the Project

This is deezer client providing multiple features like having multiple users, music trends, related musics, making playlists and much more !

<!-- TechStack -->

### Tech Stack

<details>
  <summary>Client</summary>
  <ul>
    <li><a href="https://reactjs.org//">React</a></li>
    <li><a href="https://getbootstrap.com/">Bootstrap</a></li>
    <li><a href="https://akveo.github.io/eva-icons/">Eva Icon</a></li>
    <li><a href="https://akveo.github.io/react-native-ui-kitten/docs/design-system/eva-dark-theme">UI Kitten Theme</a></li>
  </ul>
</details>

<details>
  <summary>Server</summary>
  <ul>
    <li><a href="https://www.rust-lang.org/fr">Rust</a></li>
    <li><a href="https://actix.rs/">Actix</a></li>
    <li><a href="https://github.com/seanmonstar/reqwest">Reqwest</a></li>
    <li><a href="https://www.mongodb.com/docs/drivers/rust/">MongoDB Rust Driver</a></li>
  </ul>
</details>

<details>
<summary>Database</summary>
  <ul>
    <li><a href="https://www.mongodb.com/">MongoDB</a></li>
    <li><a href="https://min.io/">MinIO</a></li>
  </ul>
</details>

<details>
<summary>DevOps</summary>
  <ul>
    <li><a href="https://www.docker.com/">Docker</a></li>
    <li><a href="https://kubernetes.io/">Kubernetes</a></li>
    <li><a href="https://github.com/features/actions">Github Actions</a></li>
  </ul>
</details>

<!-- Features -->

### Features

-   Creating playlists
-   Suggestion to complete playlists
-   Trending musics

<!-- Env Variables -->

### Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`S3_URL`

`ARL`

`MONGO_URL`

<!-- Getting Started -->

## Getting Started without Kubernetes

<!-- Prerequisites -->

### Prerequisites

You will need the following software:

-   MinIO instance running
-   MongoDB database running

<!-- Installation -->

### Installation

Transpile MopRs client

```bash
  npm install
  npx webpack --mode production
```

Compile MopRs server

```bash
  cargo build --release
```

Run MopRs server

```bash
  cargo run --release
```

Your app should be available at 8080

## Getting Started with Kubernetes

<!-- Prerequisites -->

### Prerequisites

You will need the following software:

-   Kubernetes
-   MongoDB operator running on Kubernetes

<!-- Installation -->

### Installation

You will just need to apply `.yaml` files in `.kube` directory

```bash
  kubectl apply -f .kube
```

Your app should be available at 80

<!-- Contributing -->

## Contributing

<a href="https://github.com/MalauD/MopRs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=MalauD/MopRs" />
</a>

Contributions are always welcome!

Do not hesitate to start a pull request !

<!-- Code of Conduct -->

### Code of Conduct

Please read the [Code of Conduct](https://github.com/MalauD/MopRs/blob/master/CODE_OF_CONDUCT.md)

<!-- License -->

## License

Distributed under the no License. See [LICENSE](https://github.com/MalauD/MopRs/blob/master/LICENSE) for more information.

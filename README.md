# Webshop project NTNU Ålesund 2023, Group 04 
This repository contains the code for two portfolio projects in the courses [IDATA2301 - Web Technologies](https://www.ntnu.edu/studies/courses/IDATA2301) and [IDATA2306 - Application Development](https://www.ntnu.edu/studies/courses/IDATA2306) at NTNU Ålesund. The former course focuses on front-end web development, while the latter focuses on back-end web development. Both projects were combined into one, resulting in a webshop.

## About
Our task was to create a webshop for a fictional company named *ProFlex*, which sells software to enterprises. Instead of one-time purchases, we decided to have *ProFlex* sell licenses with expiration dates and a limited number of users to better reflect real-world scenarios.  

The website can be accessed at [group04.web-tek.ninja](https://group04.web-tek.ninja/). Please note that we only have a public IPv6 address. If your internet connection does not support IPv6, you may not be able to access the website. However, we do have an IPv4 address that is only accessible from NTNU's network. You can check if your internet connection supports IPv6 by visiting [ipv6-test.com](https://ipv6-test.com/).  

**Note:** The server may be down and the domain name used for other projects after summer 2023.

## Some technical details
### Front-end (IDATA2301)
We used [React](https://react.dev/) with [TypeScript](https://www.typescriptlang.org/) for the front-end development. For more information, please refer to the [webshop_frontend README](./webshop_frontend/README.md).

### Back-end (IDATA2306)
The back-end was written in [Rust](https://www.rust-lang.org/), using the [Actix](https://actix.rs/) framework. We utilized [SQLx](https://crates.io/crates/sqlx) for database access. For more information, please refer to the [webshop_server README](./webshop_server/README.md).
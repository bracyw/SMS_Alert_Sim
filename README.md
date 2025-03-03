# SMS SIM !!!!

## Tech / General Req's

- Docker  
- Angular 18+  
- Rust latest version / cargo  
- **YOU MUST INSTALL `sea-orm-cli` to continue** (also the other stuff but don't forget about this)

---

## Getting Started

### Backend Setup

**BACKEND FAST OPTION:**  
1. Running the database (in Docker):  
   ```bash
   cd sms_backend; docker compose up sms_db
   ```
2. Running the server (this will automatically apply migrations to the DB):  
   ```bash
   cd sms_backend; cargo run
   ```

**BACKEND FULL DOCKER OPTION:**  
If you don't want to go through all those commands, you can run:  
```bash
cd sms_backend; docker compose up sms_db
```

### Frontend Setup

1. Run the frontend:  
   ```bash
   cd sms_frontend; ng serve
   ```
2. Then, you will be able to go to `localhost:4200` and play around with the minimal MVP.

---

## Testing

### Backend Testing

Testing for the backend is relatively extensive, covering everything used by the routes but **not the routes and their handlers themselves**. Instead, **Postman** is used for testing routes. Any additional routes should be tested using Postman.

To run all backend tests (which will **OVERWRITE YOUR CURRENT DB** and create a new one, messing up any saved data):  
```bash
cd sms_backend; ./run_tests.sh
```

To run backend tests that **DON'T REQUIRE** a database:  
```bash
cd sms_backend; ./run_tests.sh --non-db
```

### Frontend Testing

Testing for the frontend ensures that components’ simple expected features work through mocks and API services. More testing is certainly needed; however, this confirms basic functionality (along with setting a small blueprint for future, more in-depth testing, at this early stage).

Frontend tests can be run with:  
```bash
cd sms_frontend; ng test
```  
(Requires Angular 18+)

---

## Design Overview

### `/sms_backend`  

This backend, written in **Rust**, uses **axum** for easy routing.  

- The axum router is set up in `sms_backend/src/main.rs`.  
- Routes are found in `src/routes`, which lead to handlers that take in any parameters/body from the requests in `src/handlers`.  

**Handlers:**  
- Handlers use structures from `/structures`, which are set up in `main.rs` and passed down through axum’s framework to create the underlying state and functionality of the backend.  
- `main.rs` also passes down a database connection to our **SeaORM-managed PostgreSQL** database, which currently is only responsible for historical data and could be used to set up authentication (authentication is not implemented currently).  

**Services:**  
- Handlers call `/services`, which handle most core logic (or chunks of it) for a request.  
- `/services` rely on:  
  - `/models`: For database and frontend-related types.  
  - `/utils`: For miscellaneous and shared logic among multiple components.  

---

### `/sms_frontend`  

While I'm less pleased with the frontend, here we go... It's actually pretty simple right now. Written in **Angular**, it allows for more HTML-like development, which I love (in comparison with React). I believe Angular continues to head in the right direction.  

**Overview:**  
The client only has one page, the **SMS Control Center** (a scary name with quite a simple layout).  

**Location:**  
`/src/app/features/pages/sms-control-center`.  

It has 4 components, all of which can be found in:  
`sms-control-center/components/<component-name>`.  

---

#### Key Components:
##### On the right hand side of the screen:
1. **System Config Component**  
   - Used for customizing the senders used for alerts (how exactly this works when an alert is happening can be explained later), their failure rate, and the mean/standard deviation of all sender wait times.  

   **A quick aside on design:**  
   - I chose to separate the system config component later in the project, which required some additions to the `SenderService` on the backend to allow its sender pool to be changed in real-time.  
   - The ability to arbitrarily add/remove senders from the pool was avoided for simplicity. Instead, the sender pool can be replaced as a whole.   
   - I used bounded channels because they essentially act as a simplified version of a pool—you can scoop messages out and pour them back in, with a starting capacity for your pool so that it doesn’t overflow.  

2. **Alert Manager**  
   - The heart of this project. The Alert Manager allows users to:
     - Choose a message to send out in their alert.
     - Or, if they’re crazy, **SEND RANDOMIZED MESSAGES**!  
     - They can also choose the quantity they want to send out.  

   **A quick aside on large sends:**  
   - Requests of more than 1 million begin to stack up fast on the 1000-capacity channel (used by the `SenderService`). Increasing that bound, as well as the sender count, may help.  

##### On the left-hand side of the screen, you can see alert info in two ways:  

3. **Progress Monitor:**  
   - Polls every configurable amount of seconds for new data on the current uptime stats of the servers sending and past sending (during uptime).  

4. **Alert History:**  
   - The Alert History component is possible because of our database.  
   - It allows for historical queries of specific blocks (alerts) instead of large swathes of data.  

---

#### Shared Components:
Shared components, services, utilities, and validators are located in the `/shared` directory.

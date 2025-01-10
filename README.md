# SMS SIM !!!!
### Tech / General Req's

Docker  
Angular 18+  
Rust latest version / cargo  
YOU MUST INSTALL sea-orm-cli to continue (also the other stuff but don't forget about this)

### Getting started

for getting the backend up fast run the below:
BACKEND FAST OPTION:
running the db (in docker),
`cd sms_backend; docker compose up sms_db`
then running the server (this will automatically apply migrations to the DB):
`cd sms_backend; cargo run`

BACKEND FULL DOCKER OPTIION
if you don't want to go through all those commands you can run:
`cd sms_backend; docker compose up sms_db`

then the frontend.
`cd sms_frontend; ng serve`

Then you will be able to go your localhost:4200 and play around with the minimal mvp.

### Testing

Testing for the backend is relatively extensive, covering everything used by the routes but not the routes and their handlers themselves. Instead Postman is used for testing routes. Any additional routes should be tested using Postman.

to run the backend tests simply run WHILE THE DATABASE IS RUNNING (we don't actually care about storing historical data long term so there is no need for a seperate database for testing):  
`cd sms_backend; cargo test`

Testing for the frontend ensures that components compile and api services are working. More testing is certainly needed however this confirms basic functionality, at this early stage.

Frontend tests can be run with:
`ng test`  (requires angular 18+)

### Design Overview

all of this is `/sms_backend`:  
	This backend, written in Rust uses axum for easy routing. The axum router is set up in `sms_backend/src/main.rs` 
 and it's routes are found in `src/routes`, 
 which lead to handlers to take in any paramaters / body from the requests in `src/handlers`, 
 
 those handlers also take in structures from `/structures`, 
 which are set up in main and past down through axums frame work to create the underlying state and functionality of the backend. 
 Main also passes down a database connection to our sea-orm managed postrgres database, which currently is only responsible for historical data, and could be used to setup authentication (authentication however is not implemented currently).
 
ANYWAYS, those handlers call `/services` which handle most core logic (or junks of it) to a request, which themeselves rely on `/models` for database and frontend related types and `/utils` for miscalenoues  and shared logic among multiple different places.  

all of this is in `/sms_frontend`: 
While I'm less pleased with the frontend here we gooo... it's actually pretty simple right now. Written in Angular it allows for more html like development which I love (in comparison with React) and I believe Angular continous to head in the right direction.... ANYWAYS the important logic for our app starts at the api handlers / utils in `src/api` which which handle sending and receiving data from the backend. There is only one page the `sms-control-center` which houses our components for interacting with the backend... which utilize a few generic components snagged and slightly redone from the Argos app at Electric Racing. The "Sms control Center" allows for users to update what kind and how many senders the want to use (System config), send alerts out (alert managager), and monitor live progress, as well as display data from past sends that have finished.

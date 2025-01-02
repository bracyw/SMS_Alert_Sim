# SMS SIM !!!!
### Tech / General Req's

Docker  
Angular 18+  
Rust latest version / cargo  
YOU MUST INSTALL sea-orm-cli to continue (also the other stuff but don't forget about this)

### Getting started

Currently the best way to get started is by simply:  
running the db (in docker),  
`cd sms_backend; docker-compose up sms_db`  
run: `sea-orm-cli migrate up` if it's your first time  
then the server,  
`cd sms_backend; cargo run`  

then the frontend.  
`cd sms_frontend; ng serve`  

Then you will be able to go your localhost:4200 and play around with the minimal mvp.

### Testing

Testing for the backend is relatively extensive, covering everything used by the routes but not the routes and their handlers themselves. Instead Postman is used for testing routes. Any additional routes should be tested using Postman.

to run the backend tests simply run:  
`cd sms_backend; cargo test`

Frontend testing... is pretty much non-existent. There is a minimal feature set currently and I would like to expand it more before setting any tests in stone (also just ran out of time).  

##### Postman Routes and payloads used for testing

Routes that aren't available via frontend UI:  

http://localhost:4201/progress-monitor/reset ... POST (no body)

pretty much everything else is available via frontend UI (as far as visualizing the info)

### Design Overview

all of this is `/sms_backend`:  
	This backend, written in Rust uses axum for easy routing set up in `sms_backend/src/main.rs` who takes routes made found in `src/routes`, which lead to handlers to take in any paramaters / body from the requests in `src/handlers`, those handlers also take in structures from `/structures` which are based in for most routes to use, along with a database connection to our sea-orm managed postrgres database.  
	ANYWAYS, those handlers call `/services` which handle most core logic (or junks of it) to a request, which themeselves rely on `/models` for database and frontend related types and `/utils` for miscalenoues  and shared logic among multiple different places.  

all of this is in `/sms_frontend`:  
	While I'm less pleased with the frontend here we gooo... it's actually pretty simple right now. Written in Angular it allows for more html like development which I love and Angular continous to head in the right direction.... ANYWAYS the important logic for our app starts at the api handlers / utils `src/api` which which handle sending and receiving data from the backend. There is only one page the `sms-control-center` which houses our components for interacting with the backend... which utilize a few custom components snagged and slightly redone from the Argos app at Electric Racing.

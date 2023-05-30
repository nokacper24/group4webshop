# Sprint reports for IDATA2306 App Dev

## Delegation of work for each sprint 
On [the issue board](https://github.com/nokacper24/group4webshop/issues) we have all the issues for each sprint, where each issue has a milestone that corresponds to a sprint. If an issue is marked “sprint 2” it means the goal was to finish it within the end of sprint 2. The user the issue is assigned to is the group member that worked on and finished it.  

## Sprint 1
Period: 12. January - 12. February
### Goals and outcomes: 
- Create database schema     
	- Done, there were minor changes during REST API development. 
- Query DB data 
	- Done, using SQLx crate. Mostly product details, some user-related queries. 
- Some REST API endpoints 
	- Done, mostly finished for acquiring products and product details. 
- Serving HTML files 
	- not done. 
- Running server application locally 
	- The server application with REST endpoints works, but no html files are served.
### Retrospective:
We have finished most of the goals we set for sprint 1. Serving HTML files will need to be done in sprint 2. We have also started working on things that are relevant to later sprints, such as product creation.  
There are several problems we have not included in the project plans that have come up. These problems include comprehensive API documentation or serving images through our webserver. All these issues should be addressed in sprint 2.

## Sprint 2
Period: 13. February – 12. March 
### Goals and outcomes: 
-   Authentication  
	-   Implemented, some API routes use it. 
-   Secure password storage (Hashing)  
	-   Done. 
-   HTTPS  
	-   Not implemented yet. Must be done in sprint 3. 
-   Host application on webserver 
	-   Not done. Must figure out building docker images and host it on a server.

### Left from sprint 1:
-   Serving HTML files 
	-   Done in sprint 2 
-   Running server application locally   
	-   The entire webserver, react app together with rest API, can be run locally now.

### Additional: 

-   We set up a Postman team to test our API endpoints. While it was not originally a goal, it’s a good idea to make the tests as we create the endpoints.

### Retrospective:
-   We finished most of the goals, only https left and serving from a server (rather than locally). As mentioned above, we need to figure out building docker images and host it on a server. 
-   We finished unfinished goals from sprint 1. Additionally, we implemented serving of images from filesystem, along with image upload and deletion routes.
## Sprint 3
Period 13. March - 9. April  
### Goals: 
- Proper hashing of passwords
- Proper user registration via e-mail
- Public and private routes for API endpoints
- Unfinished from previous sprint:
  - Setup HTTPS
  - Figure out docker and host on a server

### Finished:
- Hashing function implemented
- Registration with invites works using the email util
- Public and private routes for API endpoints
  - All product related endpoints are finished and documented
- HTTPS with a self-signed certificate

### Unfinished:
- Because of developer ergonomics hashing is not yet used for verification of passwords
- The email util is not currently connected to an email server, so it logs information to the log instead
- Docker, hosting on a server, certificate from letsencrypt
  - Worked on heavily at the end of sprint3/start of sprint4, should be done early in in sprint 4. At the time of this report Docker image is prepared. 

### Retrospective:  
Crucial functionality is in place, and this was generally a succesfull sprint. All product related endpoints are finished, and documented. The only thing left is to finish the docker image and host it on a server, which should be done early in sprint 4.

## Sprint 4
Period: 10. April – 14. May

### Goals
- Finish remaining features 

### Finished
- Added constraints to DB for licenses and user access 
- Webserver in a Docker image on a live server 
- Password hashing 
- Proper data for product in production 
- Postman tests for products 
- Logging to files 
- Testimonial endpoints 
- Email utility to handle different templates and send email to different users 

### Unfinished
- Few minor issues with emails 
- Video presentations not finished 
- Reset password 

### Reflection
During Sprint 4 in AppDev, we achieved significant milestones by adding constraints to the database, deploying the webserver in a Docker image on a live server, implementing password hashing, and completing various other features. However, a few minor issues with emails and unfinished video presentations remained. We will address before the project is handed in. Despite the challenges, we are satisfied with the progress made and remain committed to delivering a well-done project.
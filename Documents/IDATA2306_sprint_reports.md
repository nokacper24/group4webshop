# Sprint reports for IDATA2306 App Dev

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

## Sprint 4
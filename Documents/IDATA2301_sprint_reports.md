# Sprint reports for IDATA2301 Web Tech
## Delegation of work for each sprint 
On [the issue board](https://github.com/nokacper24/group4webshop/issues) we have all the issues for each sprint, where each issue has a milestone that corresponds to a sprint. If an issue is marked “sprint 2” it means the goal was to finish it within the end of sprint 2. The user the issue is assigned to is the group member that worked on and finished it.  
## Sprint 1

Period: 12. January-12. February

### Goals

- Basic layout and style for all pages (with React)
  - Frontpage
  - Product Grid page
  - Single Product page
  - About/Contact page
- Query DB data (products and users) with API and show on the page

### What we finished:

- We made a detailed mockup of all the pages
- Made the following pages (with little to no real functionality, using placeholder data):
  - Header and footer components, front page, all products page, purchase license page, support page, about us page, sign up/log in page, profile page, and manage license access page.
  - We started on the individual product page, but did not finish it.

### What we did not finish:

- The individual product page.
- Query DB data with API. The frontend is not connected to the backend.

### Reflection:

- The product page should have been finished in sprint 1, as the pages that are directly needed for the user to buy and use our products are more important than pages for management.
- Members of the group should communicate better in order to get a more cohesive style on the pages.
- The component structures are good. At first, we planned on using a class that extends the Component class but moved over to functional components instead to follow best practices.
- It's good that all the components are documented.
- In the Lighthouse analysis, the website scores 100 on both Best Practices and Accessibility, and less relevantly 100 on SEO, but around 70 on Performance, which should be improved.

## Sprint 2

Period: 13. February–12. March

### Goals

- Login/register page that communicates with the DB
- Fill all pages with content
- MVP

### What we finished:

- Filled most pages with content, except for company users and edit product pages.
- Update pages to use DB data instead of placeholder data.

### What we did not finish:

- Individual product page is mostly finished, but:
  - Images lack alt text
  - Decription/image rows are not properly displayed
  - Gallery slides lack animation
- The front-end is not connected to the API endpoints for login/register

### Reflection

- We should have set the sprint to last longer, as the website is not a finished MVP. It has the most important details but lacks the finishing touches, such as logging in.

## Sprint 3
Period 13. March - 9. April  
### Goals:
- Admin management panel 
- Fix bugs 

### Finished:  
- Log in API endpoints
- Alt-text for images
- Fixed description bug
- Fixed a few more bugs
  - Scrolling in table for collapsed rows
  - Description components for products not showing full width 

### Unfinished:
- Animation on gallery 
- Edit product page 

### Retrospective:  
We fixed a few bugs and connected most of the front end to the back end. There were not many specific goals for the sprint, since it was later in the project, but we managed quite well. Most issues with the sprint came from the fact that the earlier sprint should have been longer.  

## Sprint 4
Period: 10. April – 14. May

### Goals
- Finish remaining features
- Write and edit video presentation 

### Finished
- Presentation script and slides created 
- Some videos edited and some audio files created 
- Fixed bugs 
  - Select table properly selects rows 
  - Fix style in high-contrast mode 
  - Only customers can contact support 
  - Fix saving profile changes when no changes have been made 
- Implemented loading of Products from API into product manager. 

### Unfinished
- Manage product page 
- Video presentations not finished 

### Reflection
The manage product page is still in development as it was found more difficult to implement than first thought. The mental health of the developer also caused the developer to be out of action for two weeks, really slowing development down.  
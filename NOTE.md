# Feedback
* WITH OWNER in the tests starting from PG version 15
* Would be useful to talk about x-request-id
* deserialize_with is no longer needed(page 159)
* talk about golden testing
* Verify initial DB connection at startup. Avoid a broken container

# BACKEND TODO
* Fix CORS
* Improve frontend run command... Can't be stopped
* Run script should check if db is up and online, before starting anything.
* Add response for create/update on TODO
* Add transaction in the middleware
* Add a timeout on the request using middleware
* Add max body size middleware
* Add compression middleware(?)
* Add max request threshold using middleware(?)
* Add CI and all the checks
* Add cargo watch so that there is always a fresh copy when running the backend
  using run
* Authentication and Authorization
* Add a test that we are not sending cookie and authorization headers in the logs
* Add a test to check that we are sending the right logs
* Change the assert_response to a macro to check for x-request-id, to give error message
* Improve logs on failure
* Use quickcheck to validate the dummify function
* Add a test to properly validate the list todo item order by logic

# FRONTEND TODO

* Add tanstack form and convert the few forms to it
* Fix linting
* Create Main Page(No todo created and then shows the todo item of the favorite todo)
* Add dummy/offline "backend"
* The favorite todo is browser side
* Handle dates and time and not only a string
* Allow the deletion of a todo
* Allow the edition ofa todo item

# TODO?
* using test containers might be a good thing?
* Add more tests around invalid/missing values
* Work on the error model
* Add colored diff for golden testing

# Design

TODO: This is the main thing where todo item are hosted
TODO ITEM: Item that must be completed
Recurring Job: A job that is completed and upon completion a new cycle is started.
The application will automatically create a new TODO every time you complete.
* There should be a start time and a end time
* There should be a periodicity(daily, weekly, etc.) For now keep it kiss? Just use normal time and parse

## User Stories
### TODO
The user shall be able to create multiple todos
The todos shall be unique by name
If a todo is deleted, the associated todo item will be deleted as well

### TODO Item
todo item will be associated with a todo
Once a todo item is completed, the complete_time shall be set
Once a todo item is completed, it will not be possible to edit it
A todo item shall have an associated due date. If not due date is provided, the due date shall be set to NOW()
The list of todo item shall be ordered by due date ascending, and create_date
The list of todo items shall only list items that are NOT completed

### Recurring Job
A recurring job shall have a start time. If no start time is provided, it will
be assumed that the start time is NOW()
Once a recurring job is created, associated todo items will be created
Once a todo item associated with a recurring job is completed, a new todo item 
shall be created with the due date set to the periodicity of the recurring job

# Feedback
* WITH OWNER in the tests starting from PG version 15
* Would be useful to talk about x-request-id
* deserialize_with is no longer needed(page 159)
* talk about golden testing

# MAIN TODO
* List TODO
* Add TODO item
* List TODO item
* Update TODO item(done or not and when it was)
* Authentication and Authorization
* Add recurring todos 
* Add a timeout on the request using middleware
* Add max request threshold using middleware
* Add CI and all the checks

# TODO?
* using test containers might be a good thing?
* Add golden tests in the integration tests
* Add more tests around invalid/missing values
* Create a backend folder because of the incoming frontend component
* Work on the error model


# Design

TODO: This is the main thing where todo item are hosted
TODO ITEM: Item that must be completed
Recurring Job: A job that is completed and upon completion a new cycle is started.
The application will automatically create a new TODO every time you complete.
* There should be a start time and a end time
* There should be a periodicity(daily, weekly, etc.) For now keep it kiss? Just use normal time and parse

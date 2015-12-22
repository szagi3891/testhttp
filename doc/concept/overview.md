# Spur

_Highly asynchronic callback-driven engine for generating and serving http content._

### Primary goals:
1. Fast!
2. As non-blocking as possible
3. Constant number of threads

### Spur consist of:
1. spur::**gear**

	An _eventloop_ that reads requests from sockets, passes them to rack and sends ready responses.
    
2. spur::**rack**

	_Channel_ with workers that grabs requests and spills responses as callbacks.

3. spur::rack::**quern**

	_Worker_ that either throws a callback or builds up a tree of other querns that collapses into sequence of callbacks resulting in ready to send response.

4. spur::**hopper**

	_Backend_ for querns (workers) providing reading from or writing to a data source (database or file storage).

5. spur::hopper::**pond**

	Intermediate _cache_ for hopper.

### State of early work

* _gear_ - in progress
* _rack_ - building mock
* _quern_ - waiting
* _hopper_ - waiting

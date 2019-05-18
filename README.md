# docker-pastebin

A minimal pastebin webapp written in rust, taken from the examples found in [Rocket](https://rocket.rs).

## USAGE

POST /

  accepts raw data in the body of the request and responds with a URL of
  a page containing the body's content

  EXAMPLE: curl --data-binary @file.txt http://example.com/pastebin

GET /<id>


Credit for the code goes to [Sergio Benitez](https://github.com/SergioBenitez/) :)

# docker-pastebin

A minimal pastebin webapp written in rust, taken from the examples found in [Rocket](https://rocket.rs).

### POST /

> accepts raw data in the body of the request and responds with a URL of a page containing the body's content
>
> creates metadata containing the content-type, currently ignores anything that isn't `image/*`, setting it as `text/plain`
>
> EXAMPLE:
> ```sh
> cat file.txt | curl --data-binary @- https://example.com/pastebin
> ```

### GET /<id>

> retrieves the content for the paste with id `<id>`
>
> content type will be either `text/plain` or `image/*`

Credit for the initial code goes to [Sergio Benitez](https://github.com/SergioBenitez/) :)

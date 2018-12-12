const http = require('http')

const hello = 'Hello world!'
http.createServer(
    (req, res) => {
        res.write(hello)
        res.end()
    }
).listen(8000)
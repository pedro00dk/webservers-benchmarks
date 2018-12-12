const express = require('express')

const app = express()
hello = 'Hello world!'
app.get('/', (req, res) => res.send('Hello world!'))
app.listen(8000, () => { })
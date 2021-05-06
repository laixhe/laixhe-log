import express, {Request, Response} from 'express'
import routes from './routes/routes'

const app: express.Application = express()

app.use("/", routes)

app.listen(5050, ()=>{
    console.log('listening on port 5050')
})

module.exports = app
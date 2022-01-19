import {Router, Request, Response} from 'express'

const router: Router = Router()

router.get('/', (req: Request, res: Response)=>{
    res.send('express typescript index...')
})

router.get('/info', (req: Request, res: Response)=>{
    res.send('get info ...')
})

export default router
import { createRoute } from '@hono/zod-openapi'
import { TaskFullSchema } from '../../schemas/task'
import { taskRouter } from './router'


const taskUpdate = createRoute({
  method: 'put',
  path: '/update',
  request: {
    body: {
      content: {
        'application/json': {
          schema: TaskFullSchema,
        },
      },
    },
  },
  responses: {
    200: {
      content: {
        'application/json': {
          schema: TaskFullSchema,
        },
      },
      description: 'Edit a task',
    },
  },
})

taskRouter.openapi(taskUpdate, (c) => {
  const task = c.req.valid('json')
  return c.json({ ...task }, 200)
})

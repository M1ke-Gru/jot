import { createRoute } from '@hono/zod-openapi'
import { TaskFullSchema, TaskPatchSchema } from '../../schemas/task'
import { randomUUIDv7 } from 'bun'
import { taskRouter } from './router'


const taskCreate = createRoute({
  method: 'post',
  path: '/create',
  request: {
    body: {
      content: {
        'application/json': {
          schema: TaskPatchSchema,
        },
      },
    },
  },
  responses: {
    201: {
      content: {
        'application/json': {
          schema: TaskFullSchema,
        },
      },
      description: 'Create a task',
    },
  },
})

taskRouter.openapi(taskCreate, (c) => {
  const task = c.req.valid('json')
  return c.json({ id: randomUUIDv7(), ...task }, 201)
})

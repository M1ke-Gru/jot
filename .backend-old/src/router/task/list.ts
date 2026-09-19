import { createRoute } from '@hono/zod-openapi'
import { TaskListSchema } from '../../schemas/task'
import { randomUUIDv7 } from 'bun'
import { taskRouter } from './router'


const taskList = createRoute({
  method: 'get',
  path: '/list',
  responses: {
    200: {
      content: {
        'application/json': {
          schema: TaskListSchema,
        },
      },
      description: 'List tasks',
    },
  },
})

taskRouter.openapi(taskList, (c) => {
  return c.json({  }, 200)
})

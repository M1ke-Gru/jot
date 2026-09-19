import { OpenAPIHono } from '@hono/zod-openapi'
import { taskRouter } from './router/task/router';

const app = new OpenAPIHono();
app.route('/api/tasks', taskRouter)

// The OpenAPI documentation will be available at /doc
app.doc('/doc/api', {
  openapi: '3.0.0',
  info: {
    version: '1.0.0',
    title: 'My API',
  },
})

export default app

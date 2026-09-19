import { z } from '@hono/zod-openapi'

export const TaskPatchSchema = z.object({
  title: z.string().openapi({
    example: 'Create a task manager in React',
  }),
  description: z.string().openapi({
    example: 'Make it work in CLI and mobile, while also having a PC GUI'
  }),
  state: z.boolean(),
  importance: z.enum(["low", "medium", "high"]),
  due_date: z.date().nullable(),
}).openapi('TaskPatch')

export const TaskFullSchema = TaskPatchSchema.extend({
  id: z.uuidv7(),
}).openapi('Task');

export const TaskListSchema = z.object({
  tasks: [TaskFullSchema]
})

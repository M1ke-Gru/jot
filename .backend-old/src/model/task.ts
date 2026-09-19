import { boolean, pgEnum, pgTable, text, uuid, varchar } from "drizzle-orm/pg-core"
import { createSelectSchema } from "drizzle-orm/zod"

export const priotiryEnum = pgEnum("priotiry", ["low", "medium", "high"])

export const taskTable = pgTable("tasks", {
  id: uuid().defaultRandom().primaryKey(),
  title: text().notNull(),
  done: boolean().default(false),
  description: text().default(""),
  priotiry: priotiryEnum().default("medium")
});

export const taskSelectSchema = createSelectSchema(taskTable);



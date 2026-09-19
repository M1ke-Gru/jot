from django.db.models import QuerySet, deletion
from django.shortcuts import get_object_or_404
from ninja import Router, Schema
from ninja.security import SessionAuth
from datetime import datetime

from pydantic import UUID4

from apps.tasks.models import TaskModel


router = Router(tags=["tasks"])
auth = SessionAuth()


class TaskIn(Schema):
    title: str
    description: str
    status: bool
    created_at: datetime


class TaskFull(TaskIn):
    id: UUID4


class TaskUpdate(Schema):
    title: str | None = None
    description: str | None = None
    status: bool | None = None


@router.get("/", response=list[TaskFull], auth=auth)
def list_tasks(request) -> QuerySet[TaskModel]:
    return TaskModel.objects.filter(owner=request.auth).order_by("-created_at")


@router.post("/", response=TaskFull, auth=auth)
def add_task(request, taskIn: TaskIn):
    return TaskModel.objects.create(owner=request.auth, **taskIn.model_dump())


@router.post("/update/{task_id}", response=TaskFull, auth=auth)
def update_task(request, task_id: UUID4, taskUpdate: TaskUpdate):
    task = get_object_or_404(TaskModel, id=task_id, owner=request.auth)
    task_changed = taskUpdate.model_dump(exclude_unset=True, exclude_none=True)
    for f, v in task_changed.items():
        setattr(task, f, v)

    if task_changed:
        task.save(update_fields=list(task_changed))

    return task


@router.delete("/{task_id}", response=TaskFull, auth=auth)
def delete_task(request, task_id: UUID4):
    task = TaskModel.objects.get(id=task_id, owner=request.auth)
    task.delete()
    return task

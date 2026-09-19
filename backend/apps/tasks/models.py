from uuid import uuid4
from django.db.models import (
    CharField,
    Model,
    UUIDField,
    TextField,
    BooleanField,
    ForeignKey,
    DateTimeField,
    CASCADE,
)
from django.conf import settings


# Create your models here.
class TaskModel(Model):
    id: UUIDField = UUIDField(primary_key=True, default=uuid4, editable=False)

    title: CharField = CharField(max_length=200, null=False, blank=False)
    description: TextField = TextField()
    status: BooleanField = BooleanField(blank=False)

    created_at: DateTimeField = DateTimeField(blank=False)

    owner: ForeignKey = ForeignKey(
        settings.AUTH_USER_MODEL,
        on_delete=CASCADE,
        related_name="tasks",
    )
